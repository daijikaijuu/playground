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
