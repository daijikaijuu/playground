mod bubble_sort;

fn main() {
    let mut arr = [0, 8, 2, -1, 5];
    bubble_sort::bubble_sort(&mut arr);
    println!("{:?}", arr);
}
