use raylib::prelude::*;
use std::time::Duration;

use crate::sorting_bar::SortingBar;

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
                std::thread::sleep(Duration::from_millis(20));
            }
        }
    }
}

fn draw_bars(arr: &[SortingBar], d: &mut RaylibDrawHandle) {
    d.clear_background(Color::RAYWHITE);

    let bar_width = d.get_screen_width() / arr.len() as i32;

    for (i, bar) in arr.iter().enumerate() {
        let bar_height = bar.value * (d.get_screen_height() - 20) / 100;
        let x = i as i32 * bar_width;
        let y = d.get_screen_height() - bar_height - 10;

        d.draw_rectangle(
            x as i32,
            y as i32,
            bar_width as i32,
            bar_height as i32,
            bar.color,
        );
    }
}
