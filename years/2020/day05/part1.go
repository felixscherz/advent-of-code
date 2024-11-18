package main

import (
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	filename := "input.txt"
	data, err := os.ReadFile(filename)
	if err != nil {
		log.Fatalf("failed reading file: %s", err)
	}
	lines := strings.Split(string(data), "\n")

    max_seat_id := -1
	for _, line := range lines {
		row, col := find_row_col(line)
        seat_id := row * 8 + col
        if  seat_id > max_seat_id {
            max_seat_id = seat_id
        }
	}

    fmt.Println(max_seat_id)

}

func find_row_col(seq string) (int, int) {
	rows := make([]int, 128)
	cols := make([]int, 8)
	for i := 0; i < 128; i++ {
		rows[i] = i
	}
	for i := 0; i < 8; i++ {
		cols[i] = i
	}

	for _, part := range seq {
		switch part {
		case 'F':
			rows = rows[:len(rows)/2]
		case 'B':
			rows = rows[len(rows)/2:]
		case 'L':
			cols = cols[:len(cols)/2] // Keep the first half
		case 'R':
			cols = cols[len(cols)/2:] // Keep the second half

		}
	}
	return rows[0], cols[0]

}
