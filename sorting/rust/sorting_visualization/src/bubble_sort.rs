use raylib::prelude::*;
use std::time::Duration;

use crate::{drawing::draw_bars, sorting_bar::SortingBar};

pub fn bubble_sort(arr: &[SortingBar], rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut sorted_arr = arr.to_vec();
    let n = sorted_arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if sorted_arr[j].value > sorted_arr[j + 1].value {
                sorted_arr.swap(j, j + 1);

                // Visualization: Draw bars after each swap
                let mut d = rl.begin_drawing(thread);
                draw_bars(&sorted_arr, &mut d);
                d.draw_text("Bubble Sort Visualization", 10, 10, 20, Color::BLACK);

                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }
}
