package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func minValueIndex(arr []int) int {
	min := 0
	for i := 1; i < len(arr); i++ {
		if arr[i] < arr[min] {
			min = i
		}
	}
	return min
}

func arraySum(arr []int) int {
	sum := 0
	for _, v := range arr {
		sum += v
	}
	return sum
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	currentCalories := 0
	maxCalories := [3]int{0, 0, 0}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			min := minValueIndex(maxCalories[:])
			if currentCalories > maxCalories[min] {
				maxCalories[min] = currentCalories
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

	log.Print(arraySum(maxCalories[:]))
}
