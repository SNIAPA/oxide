use std::sync::{Arc, Mutex};

use sdl2_sys::*;


use crate::{
    d, draw::{
        colors::{BACKGROUND, BLUE, FOREGROUND},
        component::{Component, RawComponent},
        event::{Event, EventType},
        fonts::FontSize,
        frame::Frame,
    }, util::{point_in_bounds, sdl_scancode_to_char}
};

const SIZE: isize = FontSize::Small as isize + 4;

#[derive(Debug)]
pub struct TextInput {
    label: &'static str,
    x: isize,
    y: isize,
    w: isize,
    rooted_x: isize,
    rooted_y: isize,
    val: Arc<Mutex<String>>,
    focussed: bool,
}

impl TextInput {
    pub fn new(
        label: &'static str,
        x: isize,
        y: isize,
        w: isize,
        val: Arc<Mutex<String>>,
    ) -> TextInput {
        TextInput {
            label,
            x,
            y,
            w,
            rooted_x: 0,
            rooted_y: 0,
            val,
            focussed: false,
        }
    }
}

impl RawComponent for TextInput {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = self.x + root_x;
        let y = self.y + root_y;
        self.rooted_x = x;
        self.rooted_y = y;
        let label = format!("{}:", self.label);

        let label_size = frame.fonts.get_text_size(&label, FontSize::Small);

        frame.text(
            &label,
            x,
            y + SIZE / 2,
            FontSize::Small,
            false,
            FOREGROUND,
            255,
        );
        let x = x + label_size.0 + 10;

        frame.filled_rect(x, y, self.w, SIZE, BACKGROUND, 255);
        let outline = if self.focussed { BLUE } else { FOREGROUND };
        frame.outlined_rect(x, y, self.w, SIZE, outline, 255);

        let val = self.val.lock().unwrap();

        frame.text(
            &val,
            x + self.w / 2,
            y + SIZE / 2,
            FontSize::Small,
            true,
            FOREGROUND,
            255,
        );
    }

    fn handle_event(&mut self, event: &mut Event) {
        match event.r#type {
            EventType::MouseButtonDown => {
                if !self.focussed {
                    if point_in_bounds(
                        d!().cursor.0,
                        d!().cursor.1,
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
                if let Some(letter) = sdl_scancode_to_char(key) {
                    let mut val = self.val.lock().unwrap();
                    val.push(letter);
                }
                match key {
                    SDL_Scancode::SDL_SCANCODE_DELETE => {}
                    SDL_Scancode::SDL_SCANCODE_BACKSPACE => {
                        let mut val = self.val.lock().unwrap();
                        val.pop();
                    }
                    SDL_Scancode::SDL_SCANCODE_RETURN | SDL_Scancode::SDL_SCANCODE_ESCAPE => {
                        self.focussed = false
                    }
                    _ => {}
                }
                event.handled = true
            }
            _ => (),
        }
    }
}

impl Component for TextInput {}
