package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

var validFields = []string{
	"byr",
	"iyr",
	"eyr",
	"hgt",
	"hcl",
	"ecl",
	"pid",
}

func main() {
	// re := regexp.MustCompile(`\s\n|\s|\n`)
	part1Count := 0

	passports := readFile("./input.txt")
	for _, passport := range passports {
		if hasRequiredFields(passport, validFields) {
			part1Count++
		}
		// fields := re.Split(passport, -1)
		// fmt.Println(fields)
	}

	fmt.Println("Part 1: Valid passport count is: ", part1Count)
}

func readFile(filePath string) []string {
	contentBytes, err := ioutil.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(contentBytes), "\n\n")
	for i := range lines {
		lines[i] = strings.TrimSuffix(lines[i], "\n")
	}

	return lines
}

func hasRequiredFields(passport string, requiredFields []string) bool {
	for _, field := range validFields {
		if !strings.Contains(passport, field) {
			return false
		}
	}
	return true
}
