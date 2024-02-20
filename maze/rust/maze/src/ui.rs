use std::ffi::{CStr, CString};

use raylib::{prelude::*, rgui::IntoCStr, RaylibHandle, RaylibThread};

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

            // let title = CString::new("Maze crawler").unwrap().as_c_str();
            // let _title_label = d.gui_label(
            //     Rectangle::new(0.0, 0.0, d.get_screen_width() as f32, 30.0),
            //     title,
            // );
        }
    }
}
