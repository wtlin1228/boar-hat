package mr

import (
	"errors"
	"fmt"
	"log"
	"net"
	"net/http"
	"net/rpc"
	"os"
	"time"
)

type Task struct {
	no          int
	status      string // "unscheduled" | "scheduled" | "completed"
	createdAt   time.Time
	scheduleAt  time.Time
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
	if len(c.unscheduledMapTasks) > 0 {
		// pop one task
		first := c.unscheduledMapTasks[0]
		c.unscheduledMapTasks = c.unscheduledMapTasks[1:]
		task := &c.mapTasks[first]

		// assign to worker
		c.scheduledMapTasks[args.WorkerId] = task.no
		reply.No = task.no
		reply.TaskType = "map"
		reply.InputFilenames = []string{task.inputFilename}
		reply.ReducerCount = len(c.reduceTasks)

		// update task's metadata
		task.status = "scheduled"
		task.scheduleAt = time.Now()
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
		reply.TaskType = "reduce"
		reply.InputFilenames = task.inputFilenames
		reply.OutputFilename = task.outputFilename

		// update task's metadata
		task.status = "scheduled"
		task.scheduleAt = time.Now()
	} else if len(c.scheduledReduceTasks) > 0 {
		return errors.New("all reduce tasks are scheduled but haven't finished yet")
	}

	return nil
}

func (c *Coordinator) TaskSucceed(args *TaskSucceedArgs, reply *TaskSucceedReply) error {
	return nil
}

func (c *Coordinator) TaskFail(args *TaskFailArgs, reply *TaskFailReply) error {
	return nil
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

// main/mrcoordinator.go calls Done() periodically to find out
// if the entire job has finished.
func (c *Coordinator) Done() bool {
	ret := false

	// Your code here.

	return ret
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
		mapTasks,
		unscheduledMapTasks,
		scheduledMapTasks,
		reduceTasks,
		unscheduledReduceTasks,
		scheduledReduceTasks,
	}

	c.server()
	return &c
}
