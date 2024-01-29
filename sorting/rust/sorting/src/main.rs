mod sorting;

fn main() {
    let arr = [0, 8, 2, -1, 5];
    println!("Bubble sort: {:?}", sorting::bubble_sort(&arr));
    println!("Selection sort: {:?}", sorting::selection_sort(&arr));
    println!("Insertion sort: {:?}", sorting::insertion_sort(&arr));
    println!("Merge sort: {:?}", sorting::merge_sort(&arr));
}
