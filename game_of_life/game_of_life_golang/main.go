package main

import (
	"fmt"
	"math/rand"
	"os"
	"os/exec"
	"time"
)

const (
	rows    = 20
	columns = 20
)

func main() {
	rand.Seed(time.Now().UnixNano())

	board := initializeBoard()
	printBoard(board)

	for i := 0; i < 10; i++ {
		board = updateBoard(board)
		clearScreen()
		printBoard(board)
		time.Sleep(1 * time.Second)
	}
}

func initializeBoard() [][]bool {
	board := make([][]bool, rows)
	for i := range board {
		board[i] = make([]bool, columns)
		for j := range board[i] {
			// Initialize cells randomly
			board[i][j] = rand.Intn(2) == 1
		}
	}
	return board
}

func printBoard(board [][]bool) {
	fmt.Println("\rCurrent board:")
	for _, row := range board {
		for _, cell := range row {
			if cell {
				fmt.Print("■ ")
			} else {
				fmt.Print("□ ")
			}
		}
		fmt.Println()
	}
}

func updateBoard(board [][]bool) [][]bool {
	newBoard := make([][]bool, rows)
	for i := range newBoard {
		newBoard[i] = make([]bool, columns)
	}

	for i := range board {
		for j := range board[i] {
			neighbors := countNeighbors(board, i, j)
			if board[i][j] {
				// Cell is alive
				newBoard[i][j] = neighbors == 2 || neighbors == 3
			} else {
				// Cell is dead
				newBoard[i][j] = neighbors == 3
			}
		}
	}

	return newBoard
}

func countNeighbors(board [][]bool, x, y int) int {
	count := 0

	for i := -1; i <= 1; i++ {
		for j := -1; j <= 1; j++ {
			// Skip the cell itself
			if i == 0 && j == 0 {
				continue
			}

			// Check boundaries
			newX, newY := x+i, y+j
			if newX >= 0 && newX < rows && newY >= 0 && newY < columns {
				if board[newX][newY] {
					count++
				}
			}
		}
	}

	return count
}

func clearScreen() {
	cmd := exec.Command("clear")
	if isWindows() {
		cmd = exec.Command("cmd", "/c", "cls")
	}
	cmd.Stdout = os.Stdout
	cmd.Run()
}

func isWindows() bool {
	return os.Getenv("OS") == "Windows_NT"
}
