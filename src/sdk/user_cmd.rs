use crate::*;

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserCmd {
    pub vmt: *const c_void,
    pub command_number: c_int,
    pub tick_count: c_int,
    pub viewangles: Angles,
    pub forwardmove: c_float,
    pub sidemove: c_float,
    pub upmove: c_float,
    pub buttons: Buttons,
    pub impulse: u8,
    pub weaponselect: isize,
    pub weaponsubtype: isize,
    pub random_seed: isize,
    pub mousedx: i16,
    pub mousedy: i16,
    pub hasbeenpredicted: bool,
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Buttons(u32);

impl Buttons {
    pub fn get(&self, flag: ButtonFlags) -> bool {
        let flag = flag as u8;
        let shifted = 1 << flag;
        ButtonFlags::IN_ATTACK as u8;
        let Buttons(s) = *self;
        s & shifted == shifted
    }
    pub fn set(&mut self, flag: ButtonFlags, val: bool)  {
        let flag = flag as u8;
        unsafe{
            let s = self as  *mut _ as *mut u32;
            let val = if val {1} else {0};
            *s |= val << flag
        }

    }
}

#[allow(non_snake_case, non_camel_case_types, dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ButtonFlags {
    IN_ATTACK,
    IN_JUMP,
    IN_DUCK,
    IN_FORWARD,
    IN_BACK,
    IN_USE,
    IN_CANCEL,
    IN_LEFT,
    IN_RIGHT,
    IN_MOVELEFT,
    IN_MOVERIGHT,
    IN_ATTACK2,
    IN_RUN,
    IN_RELOAD,
    IN_ALT1,
    IN_ALT2,
    IN_SCORE,
    IN_SPEED,
    IN_WALK,
    IN_ZOOM,
    IN_WEAPON1,
    IN_WEAPON2,
    IN_BULLRUSH,
    IN_GRENADE1,
    IN_GRENADE2,
    IN_ATTACK3,
}
