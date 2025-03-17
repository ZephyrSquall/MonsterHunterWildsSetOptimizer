use crate::skill::SkillId;

mod arms;
mod chest;
pub mod head;
mod legs;
mod talisman;
mod waist;

pub struct Armor {
    pub name: &'static str,
    set: &'static str,
    defense: u16,
    slots: &'static [u8],
    fire_res: i16,
    water_res: i16,
    thunder_res: i16,
    ice_res: i16,
    dragon_res: i16,
    pub skills: &'static [(SkillId, u8)],
}
