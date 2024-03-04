use crate::*;
use freetype_sys::FT_Bitmap;
use sdl2_sys::*;
use sdl2_sys::*;
use std::{f32::consts::PI, intrinsics::offset, isize, mem::MaybeUninit, ptr::null, usize};

module_export!(component);
module_export!(colors);
module_export!(fonts);
module_export!(sdl_wrappers);
module_export!(frame);

pub struct Draw {
    pub fonts: Fonts,
    pub renderer: *mut SDL_Renderer,
    pub old_ctx: *mut c_void,
    pub ctx: *mut c_void,
    pub components: Components,
}

impl Draw {
    pub unsafe fn init(window: *mut SDL_Window) -> Result<Draw, std::boxed::Box<dyn Error>> {
        println!("loading menu");
        let old_ctx = SDL_GL_GetCurrentContext();
        let ctx = SDL_GL_CreateContext(window);
        let mut renderer = SDL_CreateRenderer(window, -1, 0);

        //STUPID WORKOAROUND
        if renderer.is_null() {
            renderer = SDL_GetRenderer(window);
        }

        let title = CString::new(format!(
            "Team Fortress 2 - [{}] v{} by {}",
            NAME, VERSION, AUTHOR
        ))
        .unwrap();

        SDL_SetWindowTitle(window, title.as_ptr());

        SDL_GL_MakeCurrent(window, old_ctx);

        let mut components = Components::new();

        components.add(AimbotFov {});
        components.add(Overlay::new());

        println!("loaded menu");
        Ok(Draw {
            components,
            fonts: Fonts::init(),
            old_ctx,
            ctx,
            renderer,
        })
    }

    pub unsafe fn restore(&self) {
        SDL_GL_DeleteContext(self.ctx);
        self.fonts.restore();
    }

    pub fn run(&'static mut self, window: *mut SDL_Window) {
        unsafe {
            SDL_GL_MakeCurrent(window, self.ctx);
        }

        let mut frame = Frame::new(window, self.renderer, &mut self.fonts);
        self.components.draw(&mut frame);

        unsafe {
            SDL_RenderPresent(self.renderer);
            SDL_GL_MakeCurrent(window, self.old_ctx);
        }
    }

    pub fn handle_event(&mut self, event: *mut SDL_Event) {
        self.components.handle_event(event);
    }
}