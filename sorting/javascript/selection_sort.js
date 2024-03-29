function selectionSort(arr) {
  let len = arr.length;

  for (let i = 0; i < len - 1; i++) {
    let minIndex = i;

    // Find the index of the minimum element in the remaining unsorted part
    for (let j = i + 1; j < len; j++) {
      if (arr[j] < arr[minIndex]) {
        minIndex = j;
      }
    }

    // Swap the found minimum element with the first element
    if (minIndex !== i) {
      let temp = arr[i];
      arr[i] = arr[minIndex];
      arr[minIndex] = temp;
    }
  }

  return arr;
}

let unsortedArray = [64, 25, 12, 22, 11];
let sortedArray = selectionSort(unsortedArray);
console.log("Sorted Array:", sortedArray);
