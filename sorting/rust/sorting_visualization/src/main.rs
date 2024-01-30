use std::thread;
use std::time::Duration;

use rand;
use sorting_visualization::bubble_sort::bubble_sort;
use sorting_visualization::insertion_sort::insertion_sort;
use sorting_visualization::selection_sort::selection_sort;
use sorting_visualization::sorting_bar::SortingBar;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Sorting Visualization")
        .build();

    while !rl.window_should_close() {
        // Initialize array of bars with random values
        let values: Vec<SortingBar> = (1..=50)
            .map(|_| SortingBar::new(rand::random::<i32>() % 100))
            .collect();

        bubble_sort(&values, &mut rl, &thread);
        thread::sleep(Duration::from_millis(500));
        selection_sort(&values, &mut rl, &thread);
        thread::sleep(Duration::from_millis(500));
        insertion_sort(&values, &mut rl, &thread);
    }
}
