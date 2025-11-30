package mr

import (
	"errors"
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"sync"
	"time"
)

type TaskStatus string

const (
	StatusUnscheduled TaskStatus = "unscheduled"
	StatusScheduled   TaskStatus = "scheduled"
	StatusCompleted   TaskStatus = "completed"
)

type Task struct {
	no          int
	status      TaskStatus
	createdAt   time.Time
	scheduledAt time.Time
	completedAt time.Time
}

type MapTask struct {
	Task
	inputFilename string
}

type ReduceTask struct {
	Task
	inputFilenames []string
	outputFilename string
}

type Coordinator struct {
	mu sync.Mutex

	mapTasks            []MapTask
	unscheduledMapTasks []int          // task no
	scheduledMapTasks   map[string]int // worker id -> task no

	reduceTasks            []ReduceTask
	unscheduledReduceTasks []int          // task no
	scheduledReduceTasks   map[string]int // worker id -> task no
}

// Your code here -- RPC handlers for the worker to call.

// an example RPC handler.
//
// the RPC argument and reply types are defined in rpc.go.
func (c *Coordinator) Example(args *ExampleArgs, reply *ExampleReply) error {
	reply.Y = args.X + 1
	return nil
}

func (c *Coordinator) NewTask(args *NewTaskArgs, reply *NewTaskReply) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	// worker is asking for new task without updating its previous map task
	if no, ok := c.scheduledMapTasks[args.WorkerId]; ok {
		delete(c.scheduledMapTasks, args.WorkerId)
		c.unscheduledMapTasks = append(c.unscheduledMapTasks, no)
		task := &c.mapTasks[no]
		task.status = StatusUnscheduled
	}

	// worker is asking for new task without updating its previous reduce task
	if no, ok := c.scheduledReduceTasks[args.WorkerId]; ok {
		delete(c.scheduledReduceTasks, args.WorkerId)
		c.unscheduledReduceTasks = append(c.unscheduledReduceTasks, no)
		task := &c.reduceTasks[no]
		task.status = StatusUnscheduled
	}

	if len(c.unscheduledMapTasks) > 0 {
		// pop one task
		first := c.unscheduledMapTasks[0]
		c.unscheduledMapTasks = c.unscheduledMapTasks[1:]
		task := &c.mapTasks[first]

		// assign to worker
		c.scheduledMapTasks[args.WorkerId] = task.no
		reply.No = task.no
		reply.TaskType = TaskTypeMap
		reply.InputFilenames = []string{task.inputFilename}
		reply.ReducerCount = len(c.reduceTasks)

		// update task's metadata
		task.status = StatusScheduled
		task.scheduledAt = time.Now()
	} else if len(c.scheduledMapTasks) > 0 {
		return errors.New("all map tasks are scheduled but haven't finished yet")
	} else if len(c.unscheduledReduceTasks) > 0 {
		// pop one task
		first := c.unscheduledReduceTasks[0]
		c.unscheduledReduceTasks = c.unscheduledReduceTasks[1:]
		task := &c.reduceTasks[first]

		// assign to worker
		c.scheduledReduceTasks[args.WorkerId] = task.no
		reply.No = task.no
		reply.TaskType = TaskTypeReduce
		reply.InputFilenames = task.inputFilenames
		reply.OutputFilename = task.outputFilename

		// update task's metadata
		task.status = StatusScheduled
		task.scheduledAt = time.Now()
	} else if len(c.scheduledReduceTasks) > 0 {
		return errors.New("all reduce tasks are scheduled but haven't finished yet")
	}

	return nil
}

func (c *Coordinator) TaskSucceed(args *TaskSucceedArgs, reply *TaskSucceedReply) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	workerId := args.WorkerId

	if no, ok := c.scheduledMapTasks[workerId]; ok && no == args.No {
		delete(c.scheduledMapTasks, workerId)
		task := &c.mapTasks[no]
		task.status = StatusCompleted
		task.completedAt = time.Now()
		return nil
	}

	if no, ok := c.scheduledReduceTasks[workerId]; ok && no == args.No {
		delete(c.scheduledReduceTasks, workerId)
		task := &c.reduceTasks[no]
		task.status = StatusCompleted
		task.completedAt = time.Now()
		return nil
	}

	return errors.New("this task has been rescheduled")
}

func (c *Coordinator) TaskFail(args *TaskFailArgs, reply *TaskFailReply) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	workerId := args.WorkerId

	if no, ok := c.scheduledMapTasks[workerId]; ok && no == args.No {
		delete(c.scheduledMapTasks, workerId)
		c.unscheduledMapTasks = append(c.unscheduledMapTasks, no)
		task := &c.mapTasks[no]
		task.status = StatusUnscheduled
		return nil
	}

	if no, ok := c.scheduledReduceTasks[workerId]; ok && no == args.No {
		delete(c.scheduledReduceTasks, workerId)
		c.unscheduledReduceTasks = append(c.unscheduledReduceTasks, no)
		task := &c.reduceTasks[no]
		task.status = StatusUnscheduled
		return nil
	}

	return errors.New("this task has been rescheduled")
}

// start a thread that listens for RPCs from worker.go
func (c *Coordinator) server() {
	rpc.Register(c)
	rpc.HandleHTTP()
	//l, e := net.Listen("tcp", ":1234")
	sockname := coordinatorSock()
	os.Remove(sockname)
	l, e := net.Listen("unix", sockname)
	if e != nil {
		log.Fatal("listen error:", e)
	}
	go http.Serve(l, nil)
}

func (c *Coordinator) checkExpiredTasks() {
	for {
		c.mu.Lock()

		tenSecondsAgo := time.Now().Add(-10 * time.Second)

		for workerId, no := range c.scheduledMapTasks {
			task := &c.mapTasks[no]
			if task.scheduledAt.Before(tenSecondsAgo) {
				c.unscheduledMapTasks = append(c.unscheduledMapTasks, no)
				task.status = StatusUnscheduled
				delete(c.scheduledMapTasks, workerId)
			}
		}

		for workerId, no := range c.scheduledReduceTasks {
			task := &c.reduceTasks[no]
			if task.scheduledAt.Before(tenSecondsAgo) {
				c.unscheduledReduceTasks = append(c.unscheduledReduceTasks, no)
				task.status = StatusUnscheduled
				delete(c.scheduledReduceTasks, workerId)
			}
		}

		c.mu.Unlock()

		time.Sleep(1 * time.Second)
	}
}

// main/mrcoordinator.go calls Done() periodically to find out
// if the entire job has finished.
func (c *Coordinator) Done() bool {
	c.mu.Lock()
	defer c.mu.Unlock()

	if len(c.unscheduledMapTasks) == 0 &&
		len(c.scheduledMapTasks) == 0 &&
		len(c.unscheduledReduceTasks) == 0 &&
		len(c.scheduledReduceTasks) == 0 {
		return true
	}

	return false
}

// create a Coordinator.
// main/mrcoordinator.go calls this function.
// nReduce is the number of reduce tasks to use.
func MakeCoordinator(files []string, nReduce int) *Coordinator {
	mapTasks := make([]MapTask, len(files))
	unscheduledMapTasks := make([]int, len(files))
	scheduledMapTasks := make(map[string]int)

	for x, file := range files {
		mapTasks[x] = MapTask{
			Task: Task{
				no:        x,
				status:    "unscheduled",
				createdAt: time.Now(),
			},
			inputFilename: file,
		}

		unscheduledMapTasks[x] = x
	}

	reduceTasks := make([]ReduceTask, nReduce)
	unscheduledReduceTasks := make([]int, nReduce)
	scheduledReduceTasks := make(map[string]int)

	for y := range nReduce {
		inputFilenames := make([]string, len(files))
		for x := range len(files) {
			inputFilenames[x] = fmt.Sprintf("mr-%d-%d", x, y)
		}

		reduceTasks[y] = ReduceTask{
			Task: Task{
				no:        y,
				status:    "unscheduled",
				createdAt: time.Now(),
			},
			inputFilenames: inputFilenames,
			outputFilename: fmt.Sprintf("mr-out-%d", y),
		}

		unscheduledReduceTasks[y] = y
	}

	c := Coordinator{
		sync.Mutex{},

		mapTasks,
		unscheduledMapTasks,
		scheduledMapTasks,

		reduceTasks,
		unscheduledReduceTasks,
		scheduledReduceTasks,
	}

	c.server()
	go c.checkExpiredTasks()

	return &c
}
