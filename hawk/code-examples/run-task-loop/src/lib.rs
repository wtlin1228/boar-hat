// Original version: https://github.com/web-infra-dev/rspack/blob/main/crates/rspack_core/src/utils/task_loop.rs

use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::mpsc::{self, error::TryRecvError};

pub type TaskResult<Ctx> = Result<Vec<Box<dyn Task<Ctx>>>, String>;

pub enum TaskType {
    Sync,
    Async,
}

#[async_trait::async_trait]
pub trait Task<Ctx>: Debug + Send {
    fn get_task_type(&self) -> TaskType;

    async fn sync_run(self: Box<Self>, _context: &mut Ctx) -> TaskResult<Ctx> {
        unimplemented!()
    }

    async fn async_run(self: Box<Self>) -> TaskResult<Ctx> {
        unimplemented!()
    }
}

pub async fn run_task_loop<Ctx: 'static>(
    ctx: &mut Ctx,
    init_tasks: Vec<Box<dyn Task<Ctx>>>,
) -> Result<(), String> {
    // create channel to receive async task result
    let (tx, mut rx) = mpsc::unbounded_channel::<TaskResult<Ctx>>();
    // mark whether the task loop has been returned
    // the async task should not call `tx.send` after this mark to true
    let is_expected_shutdown = Arc::new(AtomicBool::new(false));
    let mut queue = VecDeque::from(init_tasks);
    let mut active_async_task_count = 0;

    loop {
        let task = queue.pop_front();

        if task.is_none() && active_async_task_count == 0 {
            return Ok(());
        }

        if let Some(task) = task {
            match task.get_task_type() {
                TaskType::Async => {
                    let tx = tx.clone();
                    let is_expected_shutdown = is_expected_shutdown.clone();
                    active_async_task_count += 1;
                    tokio::spawn(async move {
                        let r = task.async_run().await;
                        if !is_expected_shutdown.load(Ordering::Relaxed) {
                            tx.send(r).expect("failed to send task result");
                        }
                    });
                }
                TaskType::Sync => {
                    // merge sync task result directly
                    match task.sync_run(ctx).await {
                        Ok(r) => queue.extend(r),
                        Err(e) => {
                            is_expected_shutdown.store(true, Ordering::Relaxed);
                            return Err(e);
                        }
                    }
                }
            }
        }

        let data = if queue.is_empty() && active_async_task_count != 0 {
            let res = rx.recv().await.expect("should recv success");
            Ok(res)
        } else {
            rx.try_recv()
        };

        match data {
            Ok(r) => {
                active_async_task_count -= 1;
                // merge async task result
                match r {
                    Ok(r) => queue.extend(r),
                    Err(e) => {
                        is_expected_shutdown.store(true, Ordering::Relaxed);
                        return Err(e);
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            _ => {
                panic!("unexpected recv error")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Context {
        call_sync_task_count: u32,
        max_sync_task_call: u32,
        sync_return_error: bool,
        async_return_error: bool,
    }

    #[derive(Debug)]
    struct SyncTask;

    #[async_trait::async_trait]
    impl Task<Context> for SyncTask {
        fn get_task_type(&self) -> TaskType {
            TaskType::Sync
        }
        async fn sync_run(self: Box<Self>, context: &mut Context) -> TaskResult<Context> {
            if context.sync_return_error {
                return Err("throw sync error".into());
            }

            let async_return_error = context.async_return_error;
            context.call_sync_task_count += 1;
            if context.call_sync_task_count < context.max_sync_task_call {
                return Ok(vec![
                    Box::new(AsyncTask { async_return_error }),
                    Box::new(AsyncTask { async_return_error }),
                ]);
            }
            Ok(vec![])
        }
    }

    #[derive(Debug)]
    struct AsyncTask {
        async_return_error: bool,
    }
    #[async_trait::async_trait]
    impl Task<Context> for AsyncTask {
        fn get_task_type(&self) -> TaskType {
            TaskType::Async
        }
        async fn async_run(self: Box<Self>) -> TaskResult<Context> {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            if self.async_return_error {
                Err("throw async error".into())
            } else {
                Ok(vec![Box::new(SyncTask)])
            }
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_run_task_loop() {
        let mut context = Context {
            call_sync_task_count: 0,
            max_sync_task_call: 4,
            sync_return_error: false,
            async_return_error: false,
        };
        let res = run_task_loop(
            &mut context,
            vec![Box::new(AsyncTask {
                async_return_error: false,
            })],
        )
        .await;
        assert!(res.is_ok(), "task loop should be run success");
        assert_eq!(context.call_sync_task_count, 7);

        let mut context = Context {
            call_sync_task_count: 0,
            max_sync_task_call: 4,
            sync_return_error: true,
            async_return_error: false,
        };
        let res = run_task_loop(
            &mut context,
            vec![Box::new(AsyncTask {
                async_return_error: false,
            })],
        )
        .await;
        assert!(
            format!("{:?}", res).contains("throw sync error"),
            "should return sync error"
        );
        assert_eq!(context.call_sync_task_count, 0);

        let mut context = Context {
            call_sync_task_count: 0,
            max_sync_task_call: 4,
            sync_return_error: false,
            async_return_error: true,
        };
        let res = run_task_loop(
            &mut context,
            vec![Box::new(AsyncTask {
                async_return_error: false,
            })],
        )
        .await;
        assert!(
            format!("{:?}", res).contains("throw async error"),
            "should return async error"
        );
        assert_eq!(context.call_sync_task_count, 1);
    }
}
