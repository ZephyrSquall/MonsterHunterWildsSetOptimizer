use crate::skill::{Skill, SkillId, SkillType};

pub const ARKVELDS_HUNGER: Skill = Skill {
    name: "Arkveld's Hunger",
    alt_name: Some("Hasten Recovery"),
    id: SkillId::ArkveldsHunger,
    skill_type: SkillType::SetBonus,
    max: 4,
    apply: |_modifier, _level, _weapon| {},
};

pub const GORE_MAGALAS_TYRANNY: Skill = Skill {
    name: "Gore Magala's Tyranny",
    alt_name: Some("Black Eclipse"),
    id: SkillId::GoreMagalasTyranny,
    skill_type: SkillType::SetBonus,
    max: 4,
    apply: |_modifier, _level, _weapon| {},
};

pub const GUARDIAN_ARKVELDS_VITALITY: Skill = Skill {
    name: "Guardian Arkveld's Vitality",
    alt_name: Some("Decimator"),
    id: SkillId::GuardianArkveldsVitality,
    skill_type: SkillType::SetBonus,
    max: 4,
    apply: |_modifier, _level, _weapon| {},
};
