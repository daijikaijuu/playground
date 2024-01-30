use std::time::Duration;

use raylib::prelude::*;

use crate::{drawing::draw_bars, sorting_bar::SortingBar};

pub fn selection_sort(arr: &[SortingBar], rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut sorted_arr = arr.to_vec();
    let len = sorted_arr.len();

    for i in 0..len - 1 {
        let mut min_index = i;

        for j in i + 1..len {
            if sorted_arr[j].value < sorted_arr[min_index].value {
                min_index = j;
            }
        }

        if min_index != i {
            sorted_arr.swap(i, min_index);

            // Visualization: Draw bars after each swap
            let mut d = rl.begin_drawing(thread);
            draw_bars(&sorted_arr, &mut d);
            d.draw_text("Selection Sort Visualization", 10, 10, 20, Color::BLACK);

            std::thread::sleep(Duration::from_millis(120));
        }
    }
}
