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
