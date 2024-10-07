package main

import (
	"bufio"
	"fmt"
	"io"
	"net"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}
func main() {
	// read file
	fmt.Printf("What's your input?\n")
	input := ""
	fmt.Scanf("%s", &input)
	f_in, err := os.Open(input)
	check(err)
	defer f_in.Close()
	file_reader := bufio.NewReader(f_in)

	// connect to server
	conn, errc := net.Dial("tcp", "127.0.0.1:11999")
	check(errc)
	defer conn.Close()
	tcpConn, ok := conn.(*net.TCPConn)
	if !ok {
		fmt.Println("Error: Not a TCP connection")
		return
	}

	// write to server
	writer := bufio.NewWriter(conn)
	len, errw := io.Copy(writer, file_reader)
	check(errw)
	fmt.Printf("Send a string of %d bytes\n", len)
	writer.Flush()
	tcpConn.CloseWrite()

	// read the server reply
	reader := bufio.NewReader(conn)
	message, errr := reader.ReadString('\n')
	check(errr)
	fmt.Printf("Server replies: %s", message)
}
