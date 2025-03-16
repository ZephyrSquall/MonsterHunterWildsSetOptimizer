use crate::weapon::Element;

pub mod weapon_skill;

pub struct Skill {
    name: &'static str,
    // alt_name is used for Set Bonus and Group skills to indicate the name of the skill that is
    // granted if enough of the Set Bonus or Group pieces are equipped. e.g. "Gore Magala's Tyranny"
    // is the name, "Black Eclipse" (without the "I" or "II") is the alt_name. For Weapon and Armor
    // skills, alt_name is None.
    alt_name: Option<&'static str>,
    pub id: SkillId,
    skill_type: SkillType,
    max: u8,
    pub modifier: Modifier,
}

pub struct Modifier {
    pub attack: fn(level: u8, bonus_attack: &mut u16, raw_attack: u16),
    pub affinity: fn(level: u8, bonus_affinity: &mut i16),
    raw_crit_multiplier: fn(level: u8, bonus_raw_crit_multiplier: &mut u16),
    element_crit_multiplier: fn(level: u8, bonus_element_crit_multiplier: &mut u16),
    status_crit_multiplier: fn(level: u8, bonus_status_crit_multiplier: &mut u16),
    element: fn(level: u8, bonus_element: &mut u16, raw_element: Element),
}

// The default modifiers are simply all no-op functions. Most skills don't affect any given stat, so
// this simplifies skill definitions by only requiring a function to be explicitly provided for
// stats that the skill actually modifies. It is assumed that every skill only performs addition or
// subtraction to the bonus to their stat, and hence modifiers can be applied in any order; as I
// know is true for all skills in the game currently. For each modifier, the first parameter is the
// skill level, the second paramater is the bonus to the stat so far and is mutable so it can be
// modified further, and all additional parameters provide information that at least one skill needs
// to calculate the bonus it provides.
const DEFAULT_MODIFIER: Modifier = Modifier {
    attack: |_level, _bonus_attack, _raw_attack| {},
    affinity: |_level, _bonus_affinity| {},
    raw_crit_multiplier: |_level, _bonus_raw_crit_multiplier| {},
    element_crit_multiplier: |_level, _bonus_element_crit_multiplier| {},
    status_crit_multiplier: |_level, _bonus_status_crit_multiplier| {},
    element: |_level, _bonus_element, _raw_element| {},
};

pub enum SkillType {
    Weapon,
    Armor,
    SetBonus,
    Group,
}

#[derive(PartialEq)]
pub enum SkillId {
    // Weapon skills
    CriticalElement,
    CriticalEye,
    MastersTouch,
    OffensiveGuard,
    SpeedSharpening,

    // Armor skills
    Coalescence,
    EvadeWindow,

    // Set Bonus skills
    GoreMagalasTyranny,

    // Group skills
    ScaleLayering,
    ScalingProwess,
}
