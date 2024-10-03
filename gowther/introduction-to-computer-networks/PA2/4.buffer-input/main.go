package main

import (
	"bufio"
	"fmt"
	"os"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}
func main() {
	f, err := os.Open("PA2/1.hello-world/main.go")
	check(err)
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		fmt.Println(scanner.Text())
	}

	f.Close()
}
