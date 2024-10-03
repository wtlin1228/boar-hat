package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}
func main() {
	fmt.Printf("What's your input?\n")
	input := ""
	fmt.Scanf("%s", &input)
	f_in, err := os.Open(input)
	check(err)
	defer f_in.Close()

	fmt.Printf("What's your output?\n")
	output := ""
	fmt.Scanf("%s", &output)
	f_out, err := os.Create(output)
	check(err)
	defer f_out.Close()

	scanner := bufio.NewScanner(f_in)
	writer := bufio.NewWriter(f_out)
	line_count := 0
	for scanner.Scan() {
		writer.WriteString(strconv.Itoa(line_count) + " " + scanner.Text() + "\n")
		line_count++
	}
	writer.Flush()
}
