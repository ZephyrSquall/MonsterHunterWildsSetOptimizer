use crate::skill::SkillAmount;

mod weapon_decoration;

pub struct Decoration {
    name: &'static str,
    size: u8,
    skills: &'static [SkillAmount],
}
