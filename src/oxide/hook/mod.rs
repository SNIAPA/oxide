pub mod hooks;

pub mod create_move;
pub mod frame_stage_notify;
pub mod override_view;
pub mod paint;
pub mod paint_traverse;
pub mod poll_event;
pub mod swap_window;

pub trait Hook: std::fmt::Debug {
    fn restore(&mut self);
}

#[macro_export]
macro_rules! define_hook{
    ($name:ident,$stringName:expr,$return:ty,$default:ident,$($argName:ident,$argType:ty),*) => {
        use crate::{cfn,o,OXIDE};
        use core::intrinsics::transmute_unchecked;
        use crate::oxide::hook::Hook;
        use core::intrinsics::breakpoint;


        type RawHookFn = cfn!($return,$($argType),*);
        type BeforeHookFn =  fn ($($argType),*) -> ();
        type AfterHookFn = fn ($($argType),*,&mut $return) -> ();

        #[derive(Debug)]
        pub struct $name
        {
            pub org: RawHookFn,
            pub target: &'static mut RawHookFn,
            pub before: Vec<BeforeHookFn>,
            pub after: Vec<AfterHookFn>,
        }

        impl $name {
            pub fn name() -> &'static str{
                $stringName
            }
            pub fn init(target: &RawHookFn) -> Self {
                let target = unsafe {transmute_unchecked::<_,&'static mut RawHookFn>(target)};
                let org = (*target).clone();
                let before = Vec::new();
                let after = Vec::new();
                let hook = $name { org, target, before, after };
                *hook.target = $name::hook_fn;
                hook
            }
            fn restore(&mut self) {
        *self.target = self.org
        }
            #[allow(unused)]
            unsafe extern "C-unwind" fn hook_fn($($argName:$argType),*) -> $return{
                if OXIDE.is_none() {
                    return $default;
                }
                let mut hook: Box<CreateMoveHook> = transmute_unchecked(o!().hooks.hooks.get($stringName).unwrap());

                breakpoint();
                for fun in hook.before {
                    (fun)($($argName),*);
                }

                let mut res = (hook.org)($($argName),*);

                for fun in hook.after {
                    (fun)($($argName),*,&mut res);
                }
                res
            }
        }
        impl Hook for $name {
            fn restore(&mut self) {
                self.restore()

            }

        }
    }
}
