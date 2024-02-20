use std::ffi::CString;

use raylib::{prelude::*, RaylibHandle, RaylibThread};

pub struct Ui<'a> {
    pub rl: &'a mut RaylibHandle,
    thread: &'a RaylibThread,
}

impl<'a> Ui<'a> {
    pub fn new(rl: &'a mut RaylibHandle, thread: &'a RaylibThread) -> Self {
        Ui { rl, thread }
    }

    pub fn run(&mut self) {
        while !self.rl.window_should_close() {
            let mut d = self.rl.begin_drawing(&self.thread);
            d.clear_background(Color::BLACK);

            // Create UI
            d.gui_enable();

            let title = "Maze crawler";
            let font_size = d.gui_get_font().baseSize;
            let title_width = raylib::text::measure_text(title, font_size);
            let _title_label = d.gui_label(
                Rectangle::new(
                    (d.get_screen_width() / 2) as f32 - (title_width / 2) as f32,
                    0.0,
                    0.0,
                    30.0,
                ),
                Some(CString::new(title).unwrap().as_c_str()),
            );
        }
    }
}
