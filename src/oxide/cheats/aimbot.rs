use std::isize;

use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct Aimbot {}

impl Aimbot {
    pub fn init() -> Aimbot {
        Aimbot {}
    }
    pub fn ent_priority(&self, p_local: &Entity, ent: &Entity) -> Option<isize> {
        let me = p_local.vec_origin;
        let target = ent.vec_origin;
        unsafe {
            if call!(*ent, get_team_number) == call!(*p_local, get_team_number) {
                return None;
            }
            return Some(-(me - target).dist3d() as isize);
        }
    }

    pub fn remove_punch(p_local: &Entity) {
        let mut my_angles = unsafe { *call!(*p_local, get_abs_angles) };
        my_angles.pitch += p_local.vec_punch_angle.pitch;
        my_angles.yaw += p_local.vec_punch_angle.yaw;
        my_angles.roll += p_local.vec_punch_angle.roll;
    }

    pub unsafe fn find_target(&self, p_local: &Entity) -> Result<Option<Angles>, OxideError> {
        let entity_count = call!(interface!(entity_list), get_max_entities);

        let mut target: Option<(Vector3, isize)> = None;
        let my_eyes = call!(*p_local, eye_position);

        for i in 0..entity_count {
            let Some(ent) = Entity::get_player(i) else {
                continue;
            };

            let Some(prio) = self.ent_priority(p_local, ent) else {
                continue;
            };

            let Some((hitbox, bone)) = ent.get_hitbox(self.hitbox(p_local)) else {
                return Err(OxideError::new("could not get hitbox").into());
            };

            let target_point = hitbox.center(bone);

            let trace = trace(my_eyes, target_point, MASK_SHOT, ent);
            if trace.entity != ent as *const _ {
                continue;
            }

            let Some((_, target_prio)) = &target else {
                target = Some((target_point, prio));
                continue;
            };

            if prio > *target_prio {
                target = Some((target_point, prio))
            }
        }

        let Some((target_point, prio)) = target else {
            return Ok(None);
        };
        let diff = target_point - my_eyes;

        return Ok(Some(diff.angle()));
        Ok(None)
    }
    pub fn hitbox(&self, p_local: &Entity) -> HitboxId {
        match p_local.player_class {
            PlayerClass::Sniper => HitboxId::HitboxHead,
            _ => HitboxId::HitboxPelvis,
        }
    }
    pub fn should_run(&mut self) -> bool {
        if !menu!().aimbot_checkbox.checked {
            return false;
        }

        let Some(p_local) = Entity::local() else {
            return false;
        };

        if !unsafe { call!(*p_local, is_alive) } {
            return false;
        }
        true
    }

    pub unsafe fn pre_create_move(&mut self, cmd: &mut UserCmd) -> Result<(), OxideError> {
        if !self.should_run() {
            return Ok(());
        }

        let p_local = Entity::local().unwrap();

        let start = std::time::SystemTime::now();
        if let Some(new_angle) = self.find_target(p_local)? {
            if self.shoot(p_local, cmd) {
                cmd.viewangles = new_angle;
            }
        }
        let end = std::time::SystemTime::now();

        Ok(())
    }
    pub fn shoot(&mut self, p_local: &Entity, cmd: &mut UserCmd) -> bool {
        match p_local.player_class {
            PlayerClass::Sniper => {
                let weapon = unsafe { *call!(*p_local, get_weapon) };
                if !p_local.player_cond.get(ConditionFlags::Zoomed) {
                    cmd.buttons.set(ButtonFlags::InAttack2, true);
                    return false;
                }
                unsafe {
                    if !p_local.can_attack() || !call!(weapon, can_fire_critical_shot, true) {
                        return false;
                    }
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    true
                }
            }
            PlayerClass::Hwguy => {
                cmd.buttons.set(ButtonFlags::InAttack, true);
                true
            }
            _ => unsafe {
                if p_local.can_attack() {
                    cmd.buttons.set(ButtonFlags::InAttack, true);
                    return true;
                }
                return false;
            },
        }
    }
}
