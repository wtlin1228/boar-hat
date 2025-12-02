package mr

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"hash/fnv"
	"log"
	"net/rpc"
	"os"
	"sort"
	"time"
)

// Map functions return a slice of KeyValue.
type KeyValue struct {
	Key   string
	Value string
}

// for sorting by key.
type ByKey []KeyValue

// for sorting by key.
func (a ByKey) Len() int           { return len(a) }
func (a ByKey) Swap(i, j int)      { a[i], a[j] = a[j], a[i] }
func (a ByKey) Less(i, j int) bool { return a[i].Key < a[j].Key }

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
		if err != nil {
			time.Sleep(1 * time.Second)
			continue
		}

		switch newTask.TaskType {
		case TaskTypeMap:
			err := mapTask(newTask.InputFilenames[0], newTask.No, newTask.ReducerCount, mapf)
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

func mapTask(inputFilename string, no int, reduceCount int, mapf func(string, string) []KeyValue) error {
	// 1. read input
	content, err := os.ReadFile(inputFilename)
	if err != nil {
		log.Fatal(err)
		return err
	}

	// 2. call map function
	kva := mapf(inputFilename, string(content))
	sort.Sort(ByKey(kva))

	// 3. write outputs
	ofiles := make([]*json.Encoder, reduceCount)
	for i := range reduceCount {
		file, err := os.Create(fmt.Sprintf("mr-%d-%d", no, i))
		if err != nil {
			log.Fatal(err)
			return err
		}
		defer file.Close()
		enc := json.NewEncoder(file)
		ofiles[i] = enc
	}
	for _, kv := range kva {
		err := ofiles[ihash(kv.Key)%reduceCount].Encode(&kv)
		if err != nil {
			log.Fatal(err)
			return err
		}
	}

	return nil
}

func reduceTask(inputFilenames []string, outputFilename string, reducef func(string, []string) string) error {
	// 1. read inputs
	kva := []KeyValue{}
	for _, inputFilename := range inputFilenames {
		infile, err := os.Open(inputFilename)
		if err != nil {
			log.Fatal(err)
			return err
		}
		dec := json.NewDecoder(infile)
		for {
			var kv KeyValue
			if err := dec.Decode(&kv); err != nil {
				break
			}
			kva = append(kva, kv)
		}
		infile.Close()
	}
	sort.Sort(ByKey(kva))

	// 2. call reduce function and write output
	ofile, err := os.Create(outputFilename)
	if err != nil {
		log.Fatal(err)
	}
	defer ofile.Close()

	i := 0
	for i < len(kva) {
		j := i + 1
		for j < len(kva) && kva[j].Key == kva[i].Key {
			j++
		}
		values := []string{}
		for k := i; k < j; k++ {
			values = append(values, kva[k].Value)
		}
		output := reducef(kva[i].Key, values)

		// this is the correct format for each line of Reduce output.
		fmt.Fprintf(ofile, "%v %v\n", kva[i].Key, output)

		i = j
	}

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
