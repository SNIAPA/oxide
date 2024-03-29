use std::sync::{Arc, Mutex};

use crate::{
    d, draw::{
        colors::{BACKGROUND, CURSOR, FOREGROUND}, component::{Components, Component}, event::{Event, EventType}, fonts::FontSize, frame::Frame
    }, util::point_in_bounds
};

use super::button::Button;

const HEADER_HEIGHT: isize = 50;

#[derive(Debug)]
pub struct Window {
    x: isize,
    y: isize,
    w: isize,
    h: isize,
    rooted_x: isize,
    rooted_y: isize,
    title: String,
    last_cursor: (isize, isize),
    pub visible: Arc<Mutex<bool>>,
    dragging: bool,
    components: Components,
    close_button: Button,
}

impl Window {
    pub fn new(title: String, visible: Arc<Mutex<bool>>, components: Components) -> Window {
        let w = 500;
        let h = 500;

        let close_button_size = FontSize::Small as isize + 2;
        let close_button_pad = HEADER_HEIGHT / 2 - close_button_size / 2;
        let close_button = Button::new(
            "x",
            w - close_button_pad - close_button_size,
            close_button_pad,
            close_button_size,
            close_button_size,
            visible.clone(),
            FontSize::Small,
        );
        Window {
            x: 100,
            y: 100,
            rooted_x: 0,
            rooted_y: 0,
            w,
            h,
            title,
            last_cursor: (0, 0),
            visible,
            dragging: false,
            components,
            close_button,
        }
    }
}

impl Component for Window {
    fn draw(&mut self, frame: &mut Frame, root_x: isize, root_y: isize) {
        let x = root_x + self.x;
        let y = root_y + self.y;
        self.rooted_x = x;
        self.rooted_y = y;
        if !*self.visible.lock().unwrap() {
            return;
        }
        frame.filled_rect(x, y, self.w, HEADER_HEIGHT, BACKGROUND, 255);
        frame.filled_rect(
            x,
            y + HEADER_HEIGHT,
            self.w,
            self.h - HEADER_HEIGHT,
            BACKGROUND,
            220,
        );

        frame.text(
            &self.title,
            x + self.w / 2,
            y + HEADER_HEIGHT / 2,
            FontSize::Medium,
            true,
            FOREGROUND,
            255,
        );

        frame.filled_rect(x, y + HEADER_HEIGHT, self.w, 1, CURSOR, 100);
        frame.outlined_rect(x, y, self.w, self.h, CURSOR, 255);

        self.components.draw(frame, x, y + HEADER_HEIGHT);
        self.close_button.draw(frame, x, y)
    }

    fn handle_event(&mut self, event: &mut Event) {
        if !*self.visible.lock().unwrap() {
            return;
        }
        self.components.handle_event(event);
        if event.handled {
            return;
        }
        self.close_button.handle_event(event);
        if event.handled {
            return;
        }
        match event.r#type {
            EventType::CursorMove(pos) => {
                if self.dragging {
                    self.x += pos.0 as isize - self.last_cursor.0;
                    self.y += pos.1 as isize - self.last_cursor.1;
                }
            }
            EventType::MouseButtonDown => {
                if point_in_bounds(
                    d!().cursor.0,
                    d!().cursor.1,
                    self.rooted_x,
                    self.rooted_y,
                    self.w,
                    HEADER_HEIGHT,
                ) {
                    self.dragging = true;
                }
                if point_in_bounds(
                    d!().cursor.0,
                    d!().cursor.1,
                    self.rooted_x,
                    self.rooted_y,
                    self.w,
                    self.h,
                ) {
                    event.handled = true;
                }
            }
            EventType::MouseButtonUp => {
                self.dragging = false;
            }
            _ => (),
        }
        self.last_cursor = d!().cursor;
    }
}

