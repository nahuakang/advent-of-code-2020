package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
	"strings"
)

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
	passwords := processPasswordAndPolicy(input)
	count := countValidPasswords(passwords)
	fmt.Println(count)
}

// Password represents a given policy and the password
type Password struct {
	min      int
	max      int
	letter   string
	password string
}

// readFile reads a given input file and returns a slice of integers
func readFile(file io.Reader) []string {
	result := []string{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		result = append(result, line)
	}

	return result
}

// processPasswordAndPolicy cleans up the raw file input and returns a slice of Password structs
func processPasswordAndPolicy(rawInput []string) []Password {
	results := []Password{}

	for _, raw := range rawInput {
		minStr, maxStr, policyLetter, password := splitPolicyFields(raw, " ")
		min, err := strconv.Atoi(minStr)
		if err != nil {
			panic(err)
		}
		max, err := strconv.Atoi(maxStr)
		if err != nil {
			panic(err)
		}

		results = append(
			results,
			Password{
				min:      min,
				max:      max,
				letter:   policyLetter,
				password: password,
			},
		)
	}

	return results
}

// splitPolicyFields is a helper function to extract password and password policies
func splitPolicyFields(input, sep string) (string, string, string, string) {
	rawResults := strings.Split(input, " ")
	minMax := strings.Split(rawResults[0], "-")
	min := minMax[0]
	max := minMax[1]
	letter := rawResults[1][0]
	password := rawResults[2]
	return string(min), string(max), string(letter), password
}

// countValidPasswords counts the number of valid passwords
func countValidPasswords(passwords []Password) int {
	count := 0
	for _, password := range passwords {
		if isValidPassword(password) {
			count++
		}
	}

	return count
}

// isValidPassword checks if a given password is valid according to its policy
func isValidPassword(password Password) bool {
	re := regexp.MustCompile(password.letter)
	matches := re.FindAllString(password.password, -1)
	count := len(matches)
	if count >= password.min && count <= password.max {
		return true
	}

	return false
}
