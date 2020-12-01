package main

import (
	"bufio"
	"errors"
	"flag"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"
)

// NewYear is the sum that two numbers should have
const NewYear = 2020

// ErrNumNotFound is returned if an input list has no two numbers that sum up to 2020
var ErrNumNotFound = errors.New("cannot find two numbers from input that sum to 2020")

func main() {
	filename := flag.String("filename", "input.txt", "a file to read text-based Python dictionary output from.")
	flag.Parse()

	file, err := os.Open(*filename)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// Get raw string from file and convert each line to an integer
	input := readFile(file)

	resultOfTwo, err := findTwoNumsToSum(input, NewYear)
	if err != nil {
		panic(err)
	}
	fmt.Println(resultOfTwo)

	resultOfThree, err := findThreeNumsToSum(input, NewYear)
	if err != nil {
		panic(err)
	}
	fmt.Println(resultOfThree)
}

// readFile reads a given input file and returns a slice of integers
func readFile(file io.Reader) []int {
	result := []int{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		num, err := strconv.Atoi(strings.TrimSpace(scanner.Text()))
		if err != nil {
			panic(err)
		}
		result = append(result, num)
	}

	return result
}

// findTwoNumsToSum finds two numbers in an array that sums to the given sum
func findTwoNumsToSum(input []int, sum int) (int, error) {
	hasSeen := make(map[int]bool)

	for _, num := range input {
		need := sum - num
		if hasSeen[need] {
			return num * need, nil
		}
		hasSeen[num] = true
	}

	return 0, ErrNumNotFound
}

// findThreeNumsToSum finds three numbers in an array that sums to the given sum
func findThreeNumsToSum(input []int, sum int) (int, error) {
	for i, num := range input {
		need := sum - num
		mult, err := findTwoNumsToSum(input[i:], need)
		if err != nil {
			continue
		} else {
			return num * mult, nil
		}
	}

	return 0, ErrNumNotFound
}
