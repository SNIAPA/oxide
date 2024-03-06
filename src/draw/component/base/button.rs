use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

#[derive(Debug)]
pub struct Button {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    val: Arc<Mutex<bool>>,
    text: String 
}

impl Button {
    pub fn new(text: &str,x: isize, y: isize, w: isize, h: isize, val: Arc<Mutex<bool>>) -> Button {
        Button {
            x,
            y,
            w,
            h,
            val,
            text: text.to_owned()
        }
    }
}

impl RawComponent for Button {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        frame.filled_rect(self.x, self.y, self.w, self.h, CURSOR_TEXT, 255);
        frame.outlined_rect(self.x, self.y, self.w, self.h, CURSOR, 255);
        frame.text(
            &self.text,
            self.x + self.w / 2,
            self.y + self.h / 2,
            FontSize::Medium,
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, mut event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    draw!().cursor.0,
                    draw!().cursor.1,
                    self.x,
                    self.y,
                    self.w,
                    self.h,
                ) {
                    let mut val = self.val.lock().unwrap();
                    *val = !*val;
                    event.handled = true;
                }
            }
            _ => (),
        }
    }
}

impl Component for Button {}