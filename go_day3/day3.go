package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
)

// Slopes represents all the type of slopes
var Slopes = [][]int{
	{1, 1},
	{3, 1},
	{5, 1},
	{7, 1},
	{1, 2},
}

func main() {
	tobogganMap := readMap("./input.txt")

	part1Count := findTrees(tobogganMap, 3, 1)
	fmt.Println(part1Count)

	part2Product := 1
	for _, slope := range Slopes {
		part2Product = part2Product * findTrees(tobogganMap, slope[0], slope[1])
	}
	fmt.Println(part2Product)
}

// readMap reads the input file and returns a slice of string
func readMap(filePath string) []string {
	file, err := ioutil.ReadFile(filePath)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(file), "\n")
	return lines
}

// findTrees finds the number of trees on the map given the map, slopes in the x and y coordinates
func findTrees(tobogganMap []string, slopeX int, slopeY int) int {
	treesCount, xPos, yPos := 0, 0, 0
	mapWidth := len(tobogganMap[0])

	for yPos < len(tobogganMap) {
		currentPos := tobogganMap[yPos][xPos]

		if currentPos == '#' {
			treesCount++
		}

		xPos = (xPos + slopeX) % mapWidth
		yPos += slopeY
	}

	return treesCount
}
