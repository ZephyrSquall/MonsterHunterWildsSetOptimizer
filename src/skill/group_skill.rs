use crate::skill::{Skill, SkillId, SkillType};

pub const ALLURING_PELT: Skill = Skill {
    name: "Alluring Pelt",
    alt_name: Some("Diversion"),
    id: SkillId::AlluringPelt,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const FORTIFYING_PELT: Skill = Skill {
    name: "Fortifying Pelt",
    alt_name: Some("Fortify"),
    id: SkillId::FortifyingPelt,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const GUARDIANS_PROTECTION: Skill = Skill {
    name: "Guardian's Protection",
    alt_name: Some("Ward of Wyveria"),
    id: SkillId::GuardiansProtection,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const GUARDIANS_PULSE: Skill = Skill {
    name: "Guardian's Pulse",
    alt_name: Some("Wylk Burst"),
    id: SkillId::GuardiansPulse,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SCALE_LAYERING: Skill = Skill {
    name: "Scale Layering",
    alt_name: Some("Adrenaline"),
    id: SkillId::ScaleLayering,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SCALING_PROWESS: Skill = Skill {
    name: "Scaling Prowess",
    alt_name: Some("Master Mounter"),
    id: SkillId::ScalingProwess,
    skill_type: SkillType::Group,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};
