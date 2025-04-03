use crate::skill::SkillAmount;

pub mod arms;
pub mod chest;
pub mod head;
pub mod legs;
pub mod talisman;
pub mod waist;

pub struct Armor {
    pub name: &'static str,
    pub set: &'static str,
    pub defense: u16,
    pub three_slots: u8,
    pub two_slots: u8,
    pub one_slots: u8,
    pub fire_res: i16,
    pub water_res: i16,
    pub thunder_res: i16,
    pub ice_res: i16,
    pub dragon_res: i16,
    pub skills: &'static [SkillAmount],
}

pub struct Talisman {
    pub name: &'static str,
    pub skills: &'static [SkillAmount],
}
