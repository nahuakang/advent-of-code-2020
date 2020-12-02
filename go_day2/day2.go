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

	// Get raw string from file and convert each line to an integer.
	input := readFile(file)
	passwords := processPasswordAndPolicy(input)

	sledRentalValidPasswordCount := countValidPasswords(passwords, isValidSledRentalPassword)
	fmt.Println(sledRentalValidPasswordCount)

	tobogganValidPasswordCount := countValidPasswords(passwords, isValidTobogganCorporatePassword)
	fmt.Println(tobogganValidPasswordCount)
}

// Password represents a given policy and the password.
type Password struct {
	min      int
	max      int
	letter   string
	password string
}

// readFile reads a given input file and returns a slice of integers.
func readFile(file io.Reader) []string {
	result := []string{}

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		result = append(result, line)
	}

	return result
}

// processPasswordAndPolicy cleans up the raw file input and returns a slice of Password structs.
func processPasswordAndPolicy(rawInputs []string) []Password {
	results := []Password{}

	for _, rawInput := range rawInputs {
		passwordInfo := splitPolicyFields(rawInput)

		min, err := strconv.Atoi(passwordInfo[0])
		if err != nil {
			panic(err)
		}

		max, err := strconv.Atoi(passwordInfo[1])
		if err != nil {
			panic(err)
		}

		results = append(
			results,
			Password{min, max, passwordInfo[2], passwordInfo[3]},
		)
	}

	return results
}

// splitPolicyFields is a helper function to extract password and password policies.
func splitPolicyFields(input string) []string {
	re := regexp.MustCompile(`[- :]+`)
	return re.Split(input, -1)
}

// countValidPasswords counts the number of valid passwords.
func countValidPasswords(passwords []Password, fn func(Password) bool) int {
	count := 0
	for _, password := range passwords {
		if fn(password) {
			count++
		}
	}
	return count
}

// isValidPassword checks if a given password is valid according to North Pole Sled Rental Company policy.
func isValidSledRentalPassword(password Password) bool {
	re := regexp.MustCompile(password.letter)
	matches := re.FindAllString(password.password, -1)
	count := len(matches)
	if count >= password.min && count <= password.max {
		return true
	}
	return false
}

// isValidTobogganCorporatePassword checks if a given password is valid according to Toboggan Corporate Authentication System's policy.
func isValidTobogganCorporatePassword(password Password) bool {
	firstPosValid := string(password.password[password.min-1]) == password.letter
	secondPosValid := string(password.password[password.max-1]) == password.letter
	if (!firstPosValid && !secondPosValid) || (firstPosValid && secondPosValid) {
		return false
	}
	return true
}
