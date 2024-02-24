#[macro_export]
macro_rules! cfn {
    ($r:ty,$($t:ty),*) => {unsafe extern "C-unwind" fn($($t), *) -> $r}
}
#[macro_export]
macro_rules! module_export {
    ($m:ident) => {
        pub mod $m;
        pub use $m::*;
    };
}

#[macro_export]
macro_rules! oxide {
    () => {
        unsafe {
        *(OXIDE as *mut _ as *mut Oxide) 
        }
    };
}

#[macro_export]
macro_rules! menu {
    () => {
        *(MENU as *mut _ as *mut Menu)
    };
}

#[macro_export]
macro_rules! interface_vmt {
    ($n:ident) => {
        (*oxide!().interfaces.$n.get_vmt())
    };
}

#[macro_export]
macro_rules! interface {
    ($n:ident) => {
        *oxide!().interfaces.$n.interface_ref()
    };
}
#[macro_export]
macro_rules! call {
    ($i:expr,$f:ident $(,$args: expr)*) => {
        ($i.vmt.$f)(addr_of!($i),$($args),*)
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
        impl HasVmt<$tv> for $t {
            fn get_vmt(&self) -> &'static $tv {
                self.vmt
            }

            fn set_vmt(&mut self, vmt: &'static $tv) {
                self.vmt = vmt
            }

            unsafe fn c(&mut self) -> $tv {
                *self.vmt
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
macro_rules! hex_to_rgba {
    ($h:expr) => {
        (
            ($h >> 32) as u8,
            ($h >> 16) as u8,
            ($h >> 8) as u8,
            $h as u8,
        )
    };
}
