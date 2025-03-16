use crate::skill::{DEFAULT_MODIFIER, Modifier, Skill, SkillId, SkillType};

pub const WEAPON_SKILLS: [Skill; 1] = [Skill {
    name: "Critical Eye",
    alt_name: None,
    id: SkillId::CriticalEye,
    skill_type: SkillType::Weapon,
    max: 5,
    modifier: Modifier {
        // Critical modifiers affinity by adding 4% times the level of Critical Eye to affinity. So
        // for the affinity modifier, provide a closure that adds 4 times level to the current
        // amount of bonus affinity.
        affinity: |level, bonus_affinity| *bonus_affinity += i16::from(level * 4),
        // Critical Eye does not affect any other stats, so use the default modifiers for everything
        // else.
        ..DEFAULT_MODIFIER
    },
}];
