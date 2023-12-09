package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	maxCalories := 0
	currentCalories := 0

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			if currentCalories > maxCalories {
				maxCalories = currentCalories
			}
			currentCalories = 0
		} else {
			v, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			currentCalories += v
		}
	}

	log.Print(maxCalories)
}
