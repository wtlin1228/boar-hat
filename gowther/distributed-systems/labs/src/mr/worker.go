package mr

import (
	"crypto/rand"
	"encoding/hex"
	"errors"
	"fmt"
	"hash/fnv"
	"log"
	"net/rpc"
	"time"
)

// Map functions return a slice of KeyValue.
type KeyValue struct {
	Key   string
	Value string
}

// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by Map.
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}

func newID() string {
	b := make([]byte, 16)
	rand.Read(b)
	return hex.EncodeToString(b)
}

// main/mrworker.go calls this function.
func Worker(
	mapf func(string, string) []KeyValue,
	reducef func(string, []string) string,
) {
	// every worker has an unique id
	workerId := newID()

	for {
		newTask, err := CallNewTask(workerId)
		// TODO: remove this debugging code
		fmt.Println(newTask, err)
		if err == nil {
			time.Sleep(1 * time.Second)
			continue
		}

		switch newTask.TaskType {
		case TaskTypeMap:
			err := mapTask(newTask.InputFilenames[0], newTask.ReducerCount, mapf)
			if err != nil {
				CallTaskFail(workerId, newTask.No, err.Error())
			} else {
				CallTaskSucceed(workerId, newTask.No)
			}
		case TaskTypeReduce:
			err := reduceTask(newTask.InputFilenames, newTask.OutputFilename, reducef)
			if err != nil {
				CallTaskFail(workerId, newTask.No, err.Error())
			} else {
				CallTaskSucceed(workerId, newTask.No)
			}
		default:
			// ignore the unexpected task
			time.Sleep(1 * time.Second)
		}
	}
}

func mapTask(inputFilename string, reducerCount int, mapf func(string, string) []KeyValue) error {

	// 1. read input

	// 2. call map function

	// 3. write to intermediate files

	return nil
}

func reduceTask(inputFilenames []string, outputFilename string, reducef func(string, []string) string) error {
	// 1. read inputs

	// 2. call reduce function

	// write to output file

	return nil
}

// example function to show how to make an RPC call to the coordinator.
//
// the RPC argument and reply types are defined in rpc.go.
func CallExample() {

	// declare an argument structure.
	args := ExampleArgs{}

	// fill in the argument(s).
	args.X = 99

	// declare a reply structure.
	reply := ExampleReply{}

	// send the RPC request, wait for the reply.
	// the "Coordinator.Example" tells the
	// receiving server that we'd like to call
	// the Example() method of struct Coordinator.
	ok := call("Coordinator.Example", &args, &reply)
	if ok {
		// reply.Y should be 100.
		fmt.Printf("reply.Y %v\n", reply.Y)
	} else {
		fmt.Printf("call failed!\n")
	}
}

func CallNewTask(workerId string) (NewTaskReply, error) {
	args := NewTaskArgs{workerId}
	reply := NewTaskReply{}
	ok := call("Coordinator.NewTask", &args, &reply)
	if !ok {
		return reply, errors.New("no new task")
	}
	return reply, nil
}

func CallTaskSucceed(workerId string, no int) {
	args := TaskSucceedArgs{workerId, no}
	reply := TaskSucceedReply{}
	call("Coordinator.TaskSucceed", &args, &reply)
}

func CallTaskFail(workerId string, no int, reason string) {
	args := TaskFailArgs{workerId, no, reason}
	reply := TaskFailReply{}
	call("Coordinator.TaskFail", &args, &reply)
}

// send an RPC request to the coordinator, wait for the response.
// usually returns true.
// returns false if something goes wrong.
func call(rpcname string, args interface{}, reply interface{}) bool {
	// c, err := rpc.DialHTTP("tcp", "127.0.0.1"+":1234")
	sockname := coordinatorSock()
	c, err := rpc.DialHTTP("unix", sockname)
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		return true
	}

	fmt.Println(err)
	return false
}
