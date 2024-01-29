mod bubble_sort;
mod selection_sort;

fn main() {
    let arr = [0, 8, 2, -1, 5];
    println!("Bubble sort: {:?}", bubble_sort::bubble_sort(&arr));
    println!("Selection sort: {:?}", selection_sort::selection_sort(&arr));
}
