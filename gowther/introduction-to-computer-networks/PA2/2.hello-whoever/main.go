package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Printf("Who's there?\n")
	text := ""
	fmt.Scanf("%s", &text)

	fmt.Printf("Hello, %s\n", text)
	fmt.Println("Hello,", text)
	fmt.Fprintf(os.Stdout, "Hello, %s\n", text)
}
