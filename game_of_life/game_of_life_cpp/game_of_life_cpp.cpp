#include <cstdlib>
#include <ctime>
#include <iostream>
#include <unistd.h>

const int rows = 20;
const int cols = 40;

void initializeGrid(bool grid[rows][cols]) {
  srand(time(0));

  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      grid[i][j] = rand() % 2 == 1;
    }
  }
}

void printGrid(const bool grid[rows][cols]) {
  system("clear");

  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      std::cout << (grid[i][j] ? '*' : ' ') << ' ';
    }
    std::cout << '\n';
  }
}

void updateGrid(bool grid[rows][cols]) {
  bool newGrid[rows][cols];

  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      int liveNeighbors = 0;

      for (int x = -1; x <= 1; ++x) {
        for (int y = -1; y <= 1; ++y) {
          if (x == 0 && y == 0)
            continue;

          int ni = i + x;
          int nj = j + y;

          if (ni >= 0 && ni < rows && nj >= 0 && nj < cols && grid[ni][nj]) {
            liveNeighbors++;
          }
        }
      }

      newGrid[i][j] =
          (liveNeighbors == 3) || (grid[i][j] && liveNeighbors == 2);
    }
  }

  for (int i = 0; i < rows; ++i) {
    for (int j = 0; j < cols; ++j) {
      grid[i][j] = newGrid[i][j];
    }
  }
}

int main() {
  bool grid[rows][cols];

  initializeGrid(grid);

  while (true) {
    printGrid(grid);
    updateGrid(grid);
    sleep(1);
  }

  return 0;
}
