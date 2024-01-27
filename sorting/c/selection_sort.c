#include "utils.h"
#include <stdio.h>

void selection_sort(int arr[], int n) {
  for (int i = 0; i < n - 1; ++i) {
    int minIndex = i;

    for (int j = i + 1; j < n; ++j) {
      if (arr[j] < arr[minIndex]) {
        minIndex = j;
      }
    }

    if (minIndex != i) {
      swap(&arr[i], &arr[minIndex]);
    }
  }
}

int main() {
  int arr[] = {98, 2, -10, 9, 0, 7};
  int n = sizeof(arr) / sizeof(arr[0]);

  printf("Original array: ");
  printArray(arr, n);
  selection_sort(arr, n);

  printf("Selection sorted array: ");
  printArray(arr, n);
  return 0;
}
