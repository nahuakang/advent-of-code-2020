package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"regexp"
	"strconv"
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
	part1Count := 0
	part2Count := 0

	passports := readFile("./input.txt")
	for _, passport := range passports {
		if hasRequiredFields(passport, validFields) {
			part1Count++
		}
	}

	for _, passport := range passports {
		if hasRequiredFields(passport, validFields) && hasValidFields(passport) {
			part2Count++
		}
	}

	fmt.Println("Part 1: Valid passport count is: ", part1Count)
	fmt.Println("Part 2: Valid passport count is: ", part2Count)
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

func hasValidFields(passport string) bool {
	fieldSplitter := regexp.MustCompile(`\s\n|\s|\n`)
	keyValueSplitter := regexp.MustCompile(`:`)
	fields := fieldSplitter.Split(passport, -1)

	for _, field := range fields {
		keyValuePair := keyValueSplitter.Split(field, 2)
		if !isValidField(keyValuePair[0], keyValuePair[1]) {
			return false
		}
	}

	return true
}

func isValidField(key, value string) bool {
	heightMatcher := regexp.MustCompile(`^(\d{2,3})(\w{2})$`)
	hairColorMatcher := regexp.MustCompile(`^#[0-9a-f]{6}$`)
	eyeColorMatcher := map[string]bool{"amb": true, "blu": true, "brn": true, "gry": true, "grn": true, "hzl": true, "oth": true}
	passIDMatcher := regexp.MustCompile(`^\d{9}$`)

	switch key {
	case "byr":
		year, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		return year >= 1920 && year <= 2002
	case "iyr":
		year, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		return year >= 2010 && year <= 2020
	case "eyr":
		year, err := strconv.Atoi(value)
		if err != nil {
			return false
		}
		return year >= 2020 && year <= 2030
	case "hgt":
		matches := heightMatcher.FindStringSubmatch(value)
		if matches == nil || len(matches) != 3 {
			return false
		}
		height, err := strconv.Atoi(matches[1])
		if err != nil {
			return false
		}
		if matches[2] == "cm" {
			return height >= 150 && height <= 193
		} else if matches[2] == "in" {
			return height >= 59 && height <= 76
		} else {
			return false
		}
	case "hcl":
		return hairColorMatcher.MatchString(value)
	case "ecl":
		return eyeColorMatcher[value]
	case "pid":
		return passIDMatcher.MatchString(value)
	case "cid":
		return true
	default:
		return false
	}
}
