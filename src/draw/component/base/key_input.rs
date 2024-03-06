use std::{
    sync::{Arc, Mutex},
    usize,
};

use sdl2_sys::*;

use crate::*;

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct KeyInput {
    x: isize,
    y: isize,
    w: isize,
    rooted_x: isize,
    rooted_y: isize,
    cursor: (isize, isize),
    val: Arc<Mutex<SDL_Scancode>>,
    focussed: bool,
}

impl KeyInput {
    pub fn new(x: isize, y: isize, w: isize, val: Arc<Mutex<SDL_Scancode>>) -> KeyInput {
        KeyInput {
            x,
            y,
            w,
            rooted_x: 0,
            rooted_y: 0,
            cursor: (0, 0),
            val,
            focussed: false,
        }
    }
}

impl RawComponent for KeyInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = self.x + root_x;
        let y = self.y + root_y;
        self.rooted_x = x;
        self.rooted_y = y;
        frame.filled_rect(x, y, self.w, SIZE, BACKGROUND, 255);
        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.w, SIZE, outline, 255);

        let val = *self.val.lock().unwrap();

        frame.text(
            &sdl_scancode_name_to_string(val),
            x + self.w / 2,
            y + SIZE / 2,
            FontSize::Small,
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, mut event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if !self.focussed {
                    if point_in_bounds(
                        draw!().cursor.0,
                        draw!().cursor.1,
                        self.rooted_x,
                        self.rooted_y,
                        self.w,
                        SIZE,
                    ) {
                        self.focussed = true;
                        event.handled = true;
                    }
                } else {
                    self.focussed = false;
                    event.handled = true;
                }
            }
            EventType::KeyDown(key) => {
                if !self.focussed {
                    return;
                }
                *self.val.lock().unwrap() = key;
                event.handled = true;
                self.focussed = false;
            }
            _ => (),
        }
    }
}

impl Component for KeyInput {}