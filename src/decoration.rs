use crate::skill::{SkillId, SkillType};

pub struct Decoration {
    name: &'static str,
    size: u8,
    skills: &'static [(SkillId, u8)],
    skill_type: SkillType,
}
