pub fn selection_sort(arr: &mut [i32]) {
    for i in 0..arr.len() - 1 {
        let mut minIndex = i;

        for j in i + 1..arr.len() {
            if (arr[j] < arr[minIndex]) {
                minIndex = j;
            }
        }

        if (minIndex != i) {
            arr.swap(i, minIndex);
        }
    }
}
