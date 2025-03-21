use crate::config::{
    AIRBORNE_COVERAGE, CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE, OFFENSIVE_GUARD_UPTIME,
};
use crate::skill::{DEFAULT_MODIFIER, Modifier, Skill, SkillId, SkillType};
use crate::weapon::WeaponType;

pub const AIRBORNE: Skill = Skill {
    name: "Airborne",
    alt_name: None,
    id: SkillId::Airborne,
    skill_type: SkillType::Weapon,
    max: 1,
    modifier: Modifier {
        // Airborne modifies attack by increasing raw damage by 10% on jump attacks, so provide a
        // closure that describes this behaviour.
        attack: |level, bonus_attack, weapon| {
            *bonus_attack += f64::from(weapon.attack) * 0.1 * f64::from(level) * AIRBORNE_COVERAGE;
        },
        // Airborne does not affect any other stats, so use the default modifiers for everything
        // else.
        ..DEFAULT_MODIFIER
    },
};

pub const ATTACK_BOOST: Skill = Skill {
    name: "Attack Boost",
    alt_name: None,
    id: SkillId::AttackBoost,
    skill_type: SkillType::Weapon,
    max: 5,
    modifier: Modifier {
        attack: |level, bonus_attack, weapon| match level {
            0 => {}
            1 => *bonus_attack += 3.0,
            2 => *bonus_attack += 5.0,
            3 => *bonus_attack += 7.0,
            4 => *bonus_attack += f64::from(weapon.attack) * 0.02 + 8.0,
            5 => *bonus_attack += f64::from(weapon.attack) * 0.04 + 9.0,
            _ => panic!("Attack Boost above maximum level"),
        },
        ..DEFAULT_MODIFIER
    },
};

pub const CRITICAL_BOOST: Skill = Skill {
    name: "Critical Boost",
    alt_name: None,
    id: SkillId::CriticalBoost,
    skill_type: SkillType::Weapon,
    max: 5,
    modifier: Modifier {
        raw_crit_multiplier: |level, bonus_raw_crit_multiplier| {
            *bonus_raw_crit_multiplier += 0.03 * f64::from(level);
        },
        ..DEFAULT_MODIFIER
    },
};

pub const CRITICAL_DRAW: Skill = Skill {
    name: "Critical Draw",
    alt_name: None,
    id: SkillId::CriticalDraw,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        affinity: |level, bonus_affinity| {
            *bonus_affinity +=
                f64::from((level + 1) * 25) * CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE;
        },
        ..DEFAULT_MODIFIER
    },
};

pub const CRITICAL_ELEMENT: Skill = Skill {
    name: "Critical Element",
    alt_name: None,
    id: SkillId::CriticalElement,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        // Source on these numbers: https://game8.co/games/Monster-Hunter-Wilds/archives/501573 I
        // don't find this a particularly reliable source since it doesn't seem to be datamined
        // info, but I am unable to find exact numbers elsewhere, so I'm trusting it for now. This
        // will be updated if I find corrected numbers or other sources that backs this up.
        element_crit_multiplier: |level, bonus_element_crit_multiplier, weapon| match weapon
            .weapon_type
        {
            WeaponType::GreatSword
            | WeaponType::Hammer
            | WeaponType::HuntingHorn
            | WeaponType::Gunlance
            | WeaponType::SwitchAxe
            | WeaponType::ChargeBlade => {
                *bonus_element_crit_multiplier += 0.2 / 3.0 * f64::from(level);
            }
            WeaponType::LongSword
            | WeaponType::SwordAndShield
            | WeaponType::DualBlades
            | WeaponType::Lance
            | WeaponType::InsectGlaive
            | WeaponType::LightBowgun
            | WeaponType::HeavyBowgun
            | WeaponType::Bow => *bonus_element_crit_multiplier += 0.05 * f64::from(level),
        },
        ..DEFAULT_MODIFIER
    },
};

pub const CRITICAL_EYE: Skill = Skill {
    name: "Critical Eye",
    alt_name: None,
    id: SkillId::CriticalEye,
    skill_type: SkillType::Weapon,
    max: 5,
    modifier: Modifier {
        affinity: |level, bonus_affinity| *bonus_affinity += f64::from(level * 4),
        ..DEFAULT_MODIFIER
    },
};

pub const CRITICAL_STATUS: Skill = Skill {
    name: "Critical Status",
    alt_name: None,
    id: SkillId::CriticalStatus,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        // I cannot find any numbers for Critical Status anywhere. Until I find hard numbers for it,
        // I'm ignoring it by returning nothing for the bonus status critical multiplier.
        status_crit_multiplier: |_level, _bonus_status_crit_multiplier| {},
        ..DEFAULT_MODIFIER
    },
};

pub const GUARD: Skill = Skill {
    name: "Guard",
    alt_name: None,
    id: SkillId::Guard,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const GUARD_UP: Skill = Skill {
    name: "Guard Up",
    alt_name: None,
    id: SkillId::GuardUp,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const MASTERS_TOUCH: Skill = Skill {
    name: "Master's Touch",
    alt_name: None,
    id: SkillId::MastersTouch,
    skill_type: SkillType::Weapon,
    max: 1,
    // TODO: Find a way to implement Master's Touch. This is not a simple skill to implement, as it
    // depends on the final affinity value, which isn't known at the time these modifiers are run.
    // For now I'm simply ignoring it.
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const OFFENSIVE_GUARD: Skill = Skill {
    name: "Offensive Guard",
    alt_name: None,
    id: SkillId::OffensiveGuard,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        attack: |level, bonus_attack, weapon| {
            *bonus_attack +=
                f64::from(weapon.attack) * 0.05 * f64::from(level) * OFFENSIVE_GUARD_UPTIME;
        },
        ..DEFAULT_MODIFIER
    },
};

pub const PUNISHING_DRAW: Skill = Skill {
    name: "Punishing Draw",
    alt_name: None,
    id: SkillId::PunishingDraw,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Punishing Draw also adds a stun effect to draw attacks, which could potentially
    // increase damage indirectly even further. Find a way to represent this.
    modifier: Modifier {
        attack: |level, bonus_attack, _weapon| {
            *bonus_attack += CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE
                * match level {
                    0 => 0.0,
                    1 => 3.0,
                    2 => 5.0,
                    3 => 7.0,
                    _ => panic!("Punishing Draw above maximum level"),
                };
        },
        ..DEFAULT_MODIFIER
    },
};

pub const SPEED_SHARPENING: Skill = Skill {
    name: "Speed Sharpening",
    alt_name: None,
    id: SkillId::SpeedSharpening,
    skill_type: SkillType::Weapon,
    max: 2,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};
