use std::{error::Error, mem::transmute};

use derivative::Derivative;

use crate::{
    c,
    draw::colors::{BLUE, RED},
    error::OxideError,
    i,
    math::{angles::Angles, vector::Vector3},
    o,
};

use self::{
    flags::Flags,
    model_info::{Hitbox, HitboxId},
};

use super::*;

use super::{
    collideable::Collideable, condition::Condition, model_render::Renderable,
    networkable::Networkable, player_class::PlayerClass, user_cmd::UserCmd, weapon::Weapon,
    CBaseHandle,
};

pub mod flags;

pub const MAX_WEAPONS: usize = 48;
pub const MAX_STUDIO_BONES: usize = 128;
pub const HITBOX_SET: usize = 0;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum BoneMask {
    BoneUsedByAnything = 0x0007FF00,
    BoneUsedByHitbox = 0x00000100,
    BoneUsedByAttachment = 0x00000200,
    BoneUsedByVertexMask = 0x0003FC00,
    BoneUsedByVertexLod0 = 0x00000400,
    BoneUsedByVertexLod1 = 0x00000800,
    BoneUsedByVertexLod2 = 0x00001000,
    BoneUsedByVertexLod3 = 0x00002000,
    BoneUsedByVertexLod4 = 0x00004000,
    BoneUsedByVertexLod5 = 0x00008000,
    BoneUsedByVertexLod6 = 0x00010000,
    BoneUsedByVertexLod7 = 0x00020000,
    BoneUsedByBoneMerge = 0x00040000,
}

#[repr(C)]
#[derive(Derivative, Clone)]
#[derivative(Debug)]
pub struct VMTEntity {
    #[derivative(Debug = "ignore")]
    _pad1: [u32; 4],
    pub get_collideable: cfn!(&Collideable, &Entity),
    #[derivative(Debug = "ignore")]
    _pad2: [u32; 6],
    pub get_abs_origin: cfn!(*const Vector3, *const Entity),
    pub get_abs_angles: cfn!(&'static Angles, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad3: [u32; 66],
    pub get_index: cfn!(&isize, &Entity),
    #[derivative(Debug = "ignore")]
    _pad4: [u32; 26],
    pub world_space_center: cfn!(&Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad5: [u32; 10],
    pub get_team_number: cfn!(Team, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad6: [u32; 34],
    pub get_health: cfn!(isize, &Entity),
    pub get_max_health: cfn!(isize, &Entity),
    #[derivative(Debug = "ignore")]
    _pad7: [u32; 29],
    pub is_alive: cfn!(bool, *const Entity),
    pub is_player: cfn!(bool, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad8: [u32; 2],
    pub is_npc: cfn!(bool, &Entity),
    #[derivative(Debug = "ignore")]
    _pad9: [u32; 2],
    pub is_weapon: cfn!(bool, &Entity),
    pub get_weapon2: cfn!(*mut u8, &Entity),
    #[derivative(Debug = "ignore")]
    _pad10: [u32; 2],
    pub eye_position: cfn!(Vector3, *const Entity),
    pub eye_angles: cfn!(Angles, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad11: [u32; 12],
    pub third_person_switch: cfn!((), &Entity, bool),
    #[derivative(Debug = "ignore")]
    _pad12: [u32; 82],
    pub get_weapon: cfn!(&'static mut Weapon, *const Entity),
    #[derivative(Debug = "ignore")]
    _pad13: [u32; 10],
    pub get_shoot_pos: cfn!(Vector3, &Entity),
    #[derivative(Debug = "ignore")]
    _pad14: [u32; 6],
    pub get_observer_mode: cfn!(isize, &Entity),
    pub get_observer_target: cfn!(&Entity, &Entity),
}

#[repr(C)]
#[derive(Derivative, Clone, Copy)]
#[derivative(Debug)]
pub struct Entity {
    pub vmt: *mut VMTEntity,
    #[derivative(Debug = "ignore")]
    _pad1: [u8; 0x7c],
    pub model_idx: isize,
    #[derivative(Debug = "ignore")]
    _pad2: [u8; 0x8C],
    pub velocity: Vector3,
    #[derivative(Debug = "ignore")]
    _pad3: [u8; 0x7C],
    pub water_level: usize,
    #[derivative(Debug = "ignore")]
    _pad4: [u8; 0x1B8],
    pub vec_origin: Vector3,
    #[derivative(Debug = "ignore")]
    _pad5: [u8; 0xC],
    pub flags: Flags,
    #[derivative(Debug = "ignore")]
    _pad6: [u8; 0x8E4],
    pub next_attack: f32,
    #[derivative(Debug = "ignore")]
    _pad7: [u8; 0x84],
    pub my_weapons: [CBaseHandle; MAX_WEAPONS],
    #[derivative(Debug = "ignore")]
    _pad8: [u8; 0xD0],
    pub vec_punch_angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad9: [u8; 0xD0],
    pub object_mode: isize,
    #[derivative(Debug = "ignore")]
    _pad10: [u8; 0x1C4],
    pub angle: Angles,
    #[derivative(Debug = "ignore")]
    _pad11: [u8; 0x48],
    pub current_command: *const UserCmd,
    #[derivative(Debug = "ignore")]
    _pad12: [u8; 0xCC],
    pub tick_base: isize,
    #[derivative(Debug = "ignore")]
    _pad13: [u8; 0x3F8],
    pub player_class: PlayerClass,
    #[derivative(Debug = "ignore")]
    _pad14: [u8; 0x36C],
    pub player_cond: Condition,
    #[derivative(Debug = "ignore")]
    _pad15: [u8; 0x18],
    pub condition_bits: isize,
    #[derivative(Debug = "ignore")]
    _pad16: [u8; 0x418],
    pub allow_move_during_taunt: bool,
    #[derivative(Debug = "ignore")]
    _pad17: [u8; 0x18],
    pub force_taunt_cam: isize,
}

impl_has_vmt!(Entity, VMTEntity);

impl Entity {
    pub fn as_renderable(&self) -> &mut Renderable {
        unsafe { transmute(transmute::<&Self, usize>(self) + 4) }
    }
    pub fn as_networkable(&mut self) -> &mut Networkable {
        unsafe { transmute(transmute::<&mut Self, usize>(self) + 8) }
    }

    pub unsafe fn can_attack(&self) -> bool {
        let now = o!().global_vars.now();
        self.next_attack <= now
    }

    //todo make this shit a result type
    pub fn get_hitbox(&self, hitbox_id: HitboxId) -> Option<Hitbox> {
        unsafe {
            let rend = self.as_renderable();

            let model = c!(rend, get_model);
            let studio_model = &*c!(i!(model_info), get_studio_model, model);

            let Some(hitbox_set) = studio_model.get_hitbox_set(HITBOX_SET) else {
                return None;
            };
            let Some(hitbox) = hitbox_set.get_hitbox(hitbox_id) else {
                return None;
            };
            Some(hitbox)
        }
    }
}

impl Entity {
    pub fn get_local() -> Result<&'static mut Entity, Box<dyn Error>> {
        let id = c!(i!(base_engine), get_local_player);
        Self::get_player(id)
    }
    pub fn get_player(id: isize) -> Result<&'static mut Entity, Box<dyn Error>> {
        unsafe {
            let ent = c!(i!(entity_list), get_client_entity, id);
            if ent.is_null() {
                return Err(OxideError::new("entity is null"));
            }
            let ent = &mut *ent;
            if !c!(ent, is_player) {
                return Err(OxideError::new("entity is not a player"));
            }

            Ok(ent)
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Team {
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn color(&self) -> usize {
        match self {
            Team::Red => RED,
            Team::Blue => BLUE,
        }
    }
}
