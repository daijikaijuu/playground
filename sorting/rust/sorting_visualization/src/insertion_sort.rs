use std::time::Duration;

use raylib::prelude::*;

use crate::{drawing::draw_bars, sorting_bar::SortingBar};

pub fn insertion_sort(arr: &[SortingBar], rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut sorted_arr = arr.to_vec();
    let len = sorted_arr.len();

    for i in 1..len {
        let key = sorted_arr[i];

        let mut j = i;

        while j > 0 && sorted_arr[j - 1].value > key.value {
            sorted_arr[j] = sorted_arr[j - 1];
            j -= 1;

            // Visualization: Draw bars after each swap
            let mut d = rl.begin_drawing(thread);
            draw_bars(&sorted_arr, &mut d);
            d.draw_text("Insertion Sort Visualization", 10, 10, 20, Color::BLACK);

            std::thread::sleep(Duration::from_millis(20));
        }
        sorted_arr[j] = key;
    }
}
