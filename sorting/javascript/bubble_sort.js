function bubble_sort(arr) {
  for (let i = 0; i < arr.length; ++i) {
    for (let j = 0; j < arr.length - i - 1; ++j) {
      const left = arr[j];
      const right = arr[j+1];
      if (left > right) {
        arr[j] = right;
        arr[j+1] = left;
      }
    }
  }
  console.log(arr);
}

bubble_sort([1, 0, 9, 3, 4]);
bubble_sort([]);
bubble_sort([1, 0, 9, -3, 4]);
