#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>

#define ROWS 20
#define COLS 40

void initializeGrid(int grid[ROWS][COLS]) {
  for (int i = 0; i < ROWS; ++i) {
    for (int j = 0; j < COLS; ++j) {
      grid[i][j] = rand() % 2;
    }
  }
}

void printGrid(const int grid[ROWS][COLS]) {
  system("clear");

  for (int i = 0; i < ROWS; ++i) {
    for (int j = 0; j < COLS; ++j) {
      printf("%c ", grid[i][j] ? '*' : ' ');
    }
    printf("\n");
  }
}

void updateGrid(int grid[ROWS][COLS]) {
  int newGrid[ROWS][COLS];

  for (int i = 0; i < ROWS; ++i) {
    for (int j = 0; j < COLS; ++j) {
      int liveNeighbors = 0;

      for (int x = -1; x <= 1; ++x) {
        for (int y = -1; y <= 1; ++y) {
          if (x == 0 && y == 0)
            continue;

          int ni = i + x;
          int nj = j + y;

          if (ni >= 0 && ni < ROWS && nj >= 0 && nj < COLS && grid[ni][nj]) {
            liveNeighbors++;
          }
        }
      }

      newGrid[i][j] =
          (liveNeighbors == 3) || (grid[i][j] && liveNeighbors == 2);
    }
  }

  for (int i = 0; i < ROWS; ++i) {
    for (int j = 0; j < COLS; ++j) {
      grid[i][j] = newGrid[i][j];
    }
  }
}

int main() {
  int grid[ROWS][COLS];

  initializeGrid(grid);

  while (1) {
    printGrid(grid);
    updateGrid(grid);
    sleep(1);
  }

  return 0;
}
