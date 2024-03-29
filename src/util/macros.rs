#[macro_export]
macro_rules! cfn {
    ($r:ty,$($t:ty),*) => {
        unsafe extern "C-unwind" fn($($t), *) -> $r
    }
}

#[macro_export]
macro_rules! o {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Oxide, OXIDE};
            &mut *(OXIDE.unwrap() as *mut _ as *mut Oxide)
        }
    };
}

#[macro_export]
macro_rules! d {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Draw, DRAW};
            &mut *(DRAW.unwrap() as *mut _ as *mut Draw)
        }
    };
}

#[macro_export]
macro_rules! s {
    () => {
        #[allow(unused_unsafe)]
        unsafe {
            use crate::{Settings, SETTINGS};
            &mut *(SETTINGS.unwrap() as *mut _ as *mut Settings)
        }
    };
}

#[macro_export]
macro_rules! interface_vmt {
    ($n:ident) => {
        (*o!().interfaces.$n.get_vmt())
    };
}

#[macro_export]
macro_rules! i {
    ($n:ident) => {{
        use crate::o;
        o!().interfaces.$n.interface_ref()
    }};
}
#[macro_export]
macro_rules! c {
    ($i:expr,$f:ident $(,$args: expr)*) => {
        #[allow(unused_unsafe)]
        unsafe{
            ((*$i.vmt).$f)($i,$($args),*)
        }
    };
}
#[macro_export]
macro_rules! call_interface {
    ($i:ident,$f:ident $(,$args: expr)*) => {
        ((*interface_ref!($i)).vmt.$f)(interface_ref!($i),$($args),*)
    };
}

#[macro_export]
macro_rules! impl_has_vmt {
    ($t:tt,$tv:tt) => {
        use crate::sdk::HasVmt;
        impl HasVmt<$tv> for $t {
            fn get_vmt(&self) -> &'static $tv {
                unsafe { &*self.vmt }
            }

            fn set_vmt(&mut self, vmt: *mut $tv) {
                self.vmt = vmt
            }
        }
    };
}

#[macro_export]
macro_rules! hex_to_rgb {
    ($h:expr) => {
        (($h >> 16) as u8, ($h >> 8) as u8, $h as u8)
    };
}
#[macro_export]
macro_rules! rgb_to_hex {
    ($r:expr,$g:expr, $b:expr) => {
        (($r as usize) << 16) + (($g as usize) << 8) + $b as usize
    };
}

#[macro_export]
macro_rules! amt {
    ($t:ty) => {
        Arc<Mutex<$t>>
    };
}

#[macro_export]
macro_rules! am {
    ($v:expr) => {
        Arc::new(Mutex::new($v))
    };
}
