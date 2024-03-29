use std::process;

use raylib::prelude::*;

use crate::sorting_bar::SortingBar;

pub fn draw_bars(
    arr: &[SortingBar],
    d: &mut RaylibDrawHandle,
    highlight_index1: Option<i32>,
    highlight_index2: Option<i32>,
) {
    d.clear_background(Color::RAYWHITE);

    let bar_width = d.get_screen_width() / arr.len() as i32;

    for (i, bar) in arr.iter().enumerate() {
        if d.window_should_close() {
            process::exit(0);
        }
        let bar_height = bar.value * (d.get_screen_height() - 20) / 100;
        let x = i as i32 * bar_width;
        let y = d.get_screen_height() - bar_height - 10;

        let color = match i as i32 == highlight_index1.unwrap_or(-1)
            || i as i32 == highlight_index2.unwrap_or(-1)
        {
            true => Color::RED,
            false => bar.color,
        };

        d.draw_rectangle(
            x as i32,
            y as i32,
            bar_width as i32,
            bar_height as i32,
            color,
        );
    }
}
