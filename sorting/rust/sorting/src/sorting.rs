pub fn bubble_sort(arr: &[i32]) -> Vec<i32> {
    let mut sorted_arr = arr.to_vec();

    let len = sorted_arr.len();

    for i in 0..len {
        for j in 0..(len - i - 1) {
            if sorted_arr[j] > sorted_arr[j + 1] {
                sorted_arr.swap(j, j + 1);
            }
        }
    }

    sorted_arr
}

pub fn selection_sort(arr: &[i32]) -> Vec<i32> {
    let mut sorted_arr = arr.to_vec();

    let len = sorted_arr.len();

    for i in 0..arr.len() - 1 {
        let mut min_index = i;

        for j in i + 1..len {
            if sorted_arr[j] < sorted_arr[min_index] {
                min_index = j;
            }
        }

        if min_index != i {
            sorted_arr.swap(i, min_index);
        }
    }

    sorted_arr
}

pub fn insertion_sort(arr: &[i32]) -> Vec<i32> {
    let mut sorted_arr = arr.to_vec();
    let len = sorted_arr.len();

    for i in 1..len {
        let key = sorted_arr[i];
        let mut j = i;

        while j > 0 && sorted_arr[j - 1] > key {
            sorted_arr[j] = sorted_arr[j - 1];
            j -= 1;
        }
        sorted_arr[j] = key;
    }

    sorted_arr
}

// Merge sort
pub fn merge_sort(arr: &[i32]) -> Vec<i32> {
    let len = arr.len();

    if len <= 1 {
        return arr.to_vec();
    }

    let mid = len / 2;
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);

    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}

pub fn quick_sort(arr: &[i32]) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr.to_vec();
    }

    let pivot = arr[0];
    let mut left_partition = Vec::new();
    let mut right_partition = Vec::new();

    for i in 1..arr.len() {
        if arr[i] < pivot {
            left_partition.push(arr[i]);
        } else {
            right_partition.push(arr[i]);
        }
    }

    let mut left_sorted = quick_sort(&left_partition);
    left_sorted.push(pivot);
    left_sorted.extend(quick_sort(&right_partition));

    left_sorted
}

// Heap sort
pub fn heap_sort(arr: &[i32]) -> Vec<i32> {
    let mut sorted_arr = arr.to_vec();
    let len = sorted_arr.len();

    // Build the max heap
    for i in (0..len / 2).rev() {
        heapify(&mut sorted_arr, i, len);
    }

    // Extract elements from the heap one by one
    for i in (1..len).rev() {
        sorted_arr.swap(0, i);
        heapify(&mut sorted_arr, 0, i);
    }

    sorted_arr
}

fn heapify(arr: &mut Vec<i32>, root: usize, len: usize) {
    let mut largest = root;
    let left_child = 2 * root + 1;
    let right_child = 2 * root + 2;

    if left_child < len && arr[left_child] > arr[largest] {
        largest = left_child;
    }

    if right_child < len && arr[right_child] > arr[largest] {
        largest = right_child;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, largest, len);
    }
}
