use rand;
use sorting_visualization::bubble_sort::bubble_sort;
use sorting_visualization::sorting_bar::SortingBar;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Buble Sort Visualization")
        .build();

    // Initialize array of bars with random values
    let values: Vec<SortingBar> = (1..=50)
        .map(|_| SortingBar::new(rand::random::<i32>() % 100))
        .collect();

    while !rl.window_should_close() {
        if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
            break;
        }

        bubble_sort(&values, &mut rl, &thread);
    }
}
