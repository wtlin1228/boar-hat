package main

import (
	"bufio"
	"fmt"
	"net"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}
func main() {
	// init server
	fmt.Println("Launching server...")
	ln, _ := net.Listen("tcp", ":11999")
	conn, _ := ln.Accept()
	defer ln.Close()
	defer conn.Close()

	// prepare writer
	f_out, err := os.Create("./whatever.txt")
	check(err)
	defer f_out.Close()
	file_writer := bufio.NewWriter(f_out)

	// read line by line
	reader := bufio.NewReader(conn)
	receive_bytes := 0
	line_count := 0
	for {
		line_count += 1
		message, errr := reader.ReadString('\n')
		if errr != nil {
			break
		}
		receive_bytes += len(message)
		file_writer.WriteString(strconv.Itoa(line_count) + " " + message)
	}
	file_writer.Flush()

	// reply to client
	writer := bufio.NewWriter(conn)
	newline := fmt.Sprintf("%d bytes received\n", receive_bytes)
	_, errw := writer.WriteString(newline)
	check(errw)
	writer.Flush()
}
