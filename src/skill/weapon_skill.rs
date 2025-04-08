use crate::config::{
    AIRBORNE_COVERAGE, CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE, OFFENSIVE_GUARD_UPTIME,
};
use crate::skill::{Skill, SkillId, SkillType};
use crate::weapon::{Element, WeaponType};

pub const AIRBORNE: Skill = Skill {
    name: "Airborne",
    alt_name: None,
    id: SkillId::Airborne,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |modifier, level, _weapon| {
        if level == 1 {
            modifier.attack_multiplier *= 1.1 * AIRBORNE_COVERAGE;
        }
    },
};

pub const ARTILLERY: Skill = Skill {
    name: "Artillery",
    alt_name: None,
    id: SkillId::Artillery,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ATTACK_BOOST: Skill = Skill {
    name: "Attack Boost",
    alt_name: None,
    id: SkillId::AttackBoost,
    skill_type: SkillType::Weapon,
    max: 5,
    apply: |modifier, level, _weapon| match level {
        0 => {}
        1 => modifier.bonus_attack += 3.0,
        2 => modifier.bonus_attack += 5.0,
        3 => modifier.bonus_attack += 7.0,
        4 => {
            modifier.attack_multiplier *= 1.02;
            modifier.bonus_attack += 8.0;
        }
        5 => {
            modifier.attack_multiplier *= 1.04;
            modifier.bonus_attack += 9.0;
        }
        _ => unreachable!("Attack Boost above maximum level"),
    },
};

pub const BALLISTICS: Skill = Skill {
    name: "Ballistics",
    alt_name: None,
    id: SkillId::Ballistics,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLAST_ATTACK: Skill = Skill {
    name: "Blast Attack",
    alt_name: None,
    id: SkillId::BlastAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Blast(_blast) = weapon.element {
            match level {
                0 => {}
                1 => {
                    modifier.element_multiplier *= 1.05;
                    modifier.bonus_element += 1.0;
                }
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 2.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 5.0;
                }
                _ => unreachable!("Blast Attack above maximum level"),
            }
        }
    },
};

pub const BLAST_FUNCTIONALITY: Skill = Skill {
    name: "Blast Functionality",
    alt_name: None,
    id: SkillId::BlastFunctionality,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLUDGEONER: Skill = Skill {
    name: "Bludgeoner",
    alt_name: None,
    id: SkillId::Bludgeoner,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const CHARGE_MASTER: Skill = Skill {
    name: "Charge Master",
    alt_name: None,
    id: SkillId::ChargeMaster,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const CHARGE_UP: Skill = Skill {
    name: "Charge Up",
    alt_name: None,
    id: SkillId::ChargeUp,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const CRITICAL_BOOST: Skill = Skill {
    name: "Critical Boost",
    alt_name: None,
    id: SkillId::CriticalBoost,
    skill_type: SkillType::Weapon,
    max: 5,
    apply: |modifier, level, _weapon| {
        modifier.raw_crit_multiplier += 0.03 * f64::from(level);
    },
};

pub const CRITICAL_DRAW: Skill = Skill {
    name: "Critical Draw",
    alt_name: None,
    id: SkillId::CriticalDraw,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, _weapon| {
        modifier.bonus_affinity +=
            f64::from((level + 1) * 25) * CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE;
    },
};

pub const CRITICAL_ELEMENT: Skill = Skill {
    name: "Critical Element",
    alt_name: None,
    id: SkillId::CriticalElement,
    skill_type: SkillType::Weapon,
    max: 3,
    // Source on these numbers: https://game8.co/games/Monster-Hunter-Wilds/archives/501573
    // I don't find this a particularly reliable source since it doesn't seem to be datamined info,
    // but I am unable to find exact numbers elsewhere, so I'm trusting it for now. This will be
    // updated if I find corrected numbers or other sources that backs this up.
    apply: |modifier, level, weapon| match weapon.weapon_type {
        WeaponType::GreatSword
        | WeaponType::Hammer
        | WeaponType::HuntingHorn
        | WeaponType::Gunlance
        | WeaponType::SwitchAxe
        | WeaponType::ChargeBlade => {
            modifier.element_crit_multiplier += 0.2 / 3.0 * f64::from(level);
        }
        WeaponType::LongSword
        | WeaponType::SwordAndShield
        | WeaponType::DualBlades
        | WeaponType::Lance
        | WeaponType::InsectGlaive
        | WeaponType::LightBowgun
        | WeaponType::HeavyBowgun
        | WeaponType::Bow => modifier.element_crit_multiplier += 0.05 * f64::from(level),
    },
};

pub const CRITICAL_EYE: Skill = Skill {
    name: "Critical Eye",
    alt_name: None,
    id: SkillId::CriticalEye,
    skill_type: SkillType::Weapon,
    max: 5,
    apply: |modifier, level, _weapon| modifier.bonus_affinity += f64::from(level * 4),
};

pub const CRITICAL_STATUS: Skill = Skill {
    name: "Critical Status",
    alt_name: None,
    id: SkillId::CriticalStatus,
    skill_type: SkillType::Weapon,
    max: 3,
    // I cannot find any numbers for Critical Status anywhere. Until I find hard numbers for it, I'm
    // ignoring it by returning nothing for the bonus status critical multiplier.
    apply: |_modifier, _level, _weapon| {},
};

pub const DRAGON_ATTACK: Skill = Skill {
    name: "Dragon Attack",
    alt_name: None,
    id: SkillId::DragonAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Dragon(_dragon) = weapon.element {
            match level {
                0 => {}
                1 => modifier.bonus_element += 4.0,
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 5.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 6.0;
                }
                _ => unreachable!("Dragon Attack above maximum level"),
            }
        }
    },
};

pub const EXHAUST_FUNCTIONALITY: Skill = Skill {
    name: "Exhaust Functionality",
    alt_name: None,
    id: SkillId::ExhaustFunctionality,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const FIRE_ATTACK: Skill = Skill {
    name: "Fire Attack",
    alt_name: None,
    id: SkillId::FireAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Also handle Fire Attack's effect on the Rathalos's Flare (Scorcher) skill.
    apply: |modifier, level, weapon| {
        if let Element::Fire(_fire) = weapon.element {
            match level {
                0 => {}
                1 => modifier.bonus_element += 4.0,
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 5.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 6.0;
                }
                _ => unreachable!("Fire Attack above maximum level"),
            }
        }
    },
};

pub const FOCUS: Skill = Skill {
    name: "Focus",
    alt_name: None,
    id: SkillId::Focus,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const GUARD: Skill = Skill {
    name: "Guard",
    alt_name: None,
    id: SkillId::Guard,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const GUARD_UP: Skill = Skill {
    name: "Guard Up",
    alt_name: None,
    id: SkillId::GuardUp,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const HANDICRAFT: Skill = Skill {
    name: "Handicraft",
    alt_name: None,
    id: SkillId::Handicraft,
    skill_type: SkillType::Weapon,
    max: 5,
    // TODO: Handle Handicraft's boost to Sharpness.
    apply: |_modifier, _level, _weapon| {},
};

pub const HORN_MAESTRO: Skill = Skill {
    name: "Horn Maestro",
    alt_name: None,
    id: SkillId::HornMaestro,
    skill_type: SkillType::Weapon,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const ICE_ATTACK: Skill = Skill {
    name: "Ice Attack",
    alt_name: None,
    id: SkillId::IceAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Ice(_ice) = weapon.element {
            match level {
                0 => {}
                1 => modifier.bonus_element += 4.0,
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 5.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 6.0;
                }
                _ => unreachable!("Ice Attack above maximum level"),
            }
        }
    },
};

pub const LOAD_SHELLS: Skill = Skill {
    name: "Load Shells",
    alt_name: None,
    id: SkillId::LoadShells,
    skill_type: SkillType::Weapon,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
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
    apply: |_modifier, _level, _weapon| {},
};

pub const MINDS_EYE: Skill = Skill {
    name: "Mind's Eye",
    alt_name: None,
    id: SkillId::MindsEye,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const NORMAL_SHOTS: Skill = Skill {
    name: "Normal Shots",
    alt_name: None,
    id: SkillId::NormalShots,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const OFFENSIVE_GUARD: Skill = Skill {
    name: "Offensive Guard",
    alt_name: None,
    id: SkillId::OffensiveGuard,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, _weapon| {
        modifier.attack_multiplier *= (1.0 + f64::from(level) * 0.05) * OFFENSIVE_GUARD_UPTIME;
    },
};

pub const OPENING_SHOT: Skill = Skill {
    name: "Opening Shot",
    alt_name: None,
    id: SkillId::OpeningShot,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Implement once the data structure for Bow coatings and Bowgun ammo is complete.
    apply: |_modifier, _level, _weapon| {},
};

pub const PARA_FUNCTIONALITY: Skill = Skill {
    name: "Para Functionality",
    alt_name: None,
    id: SkillId::ParaFunctionality,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const PARALYSIS_ATTACK: Skill = Skill {
    name: "Paralysis Attack",
    alt_name: None,
    id: SkillId::ParalysisAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Paralysis(_paralysis) = weapon.element {
            match level {
                0 => {}
                1 => {
                    modifier.element_multiplier *= 1.05;
                    modifier.bonus_element += 1.0;
                }
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 2.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 5.0;
                }
                _ => unreachable!("Paralysis Attack above maximum level"),
            }
        }
    },
};

pub const PIERCING_SHOTS: Skill = Skill {
    name: "Piercing Shots",
    alt_name: None,
    id: SkillId::PiercingShots,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const POISON_ATTACK: Skill = Skill {
    name: "Poison Attack",
    alt_name: None,
    id: SkillId::PoisonAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Poison(_poison) = weapon.element {
            match level {
                0 => {}
                1 => {
                    modifier.element_multiplier *= 1.05;
                    modifier.bonus_element += 1.0;
                }
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 2.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 5.0;
                }
                _ => unreachable!("Poison Attack above maximum level"),
            }
        }
    },
};

pub const POISON_DURATION_UP: Skill = Skill {
    name: "Poison Duration Up",
    alt_name: None,
    id: SkillId::PoisonDurationUp,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const POISON_FUNCTIONALITY: Skill = Skill {
    name: "Poison Functionality",
    alt_name: None,
    id: SkillId::PoisonFunctionality,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const POWER_PROLONGER: Skill = Skill {
    name: "Power Prolonger",
    alt_name: None,
    id: SkillId::PowerProlonger,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const PROTECTIVE_POLISH: Skill = Skill {
    name: "Protective Polish",
    alt_name: None,
    id: SkillId::ProtectivePolish,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const PUNISHING_DRAW: Skill = Skill {
    name: "Punishing Draw",
    alt_name: None,
    id: SkillId::PunishingDraw,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Punishing Draw also adds a stun effect to draw attacks, which could potentially
    // increase damage indirectly even further. Find a way to represent this.
    apply: |modifier, level, _weapon| {
        modifier.bonus_attack += CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE
            * match level {
                0 => 0.0,
                1 => 3.0,
                2 => 5.0,
                3 => 7.0,
                _ => unreachable!("Punishing Draw above maximum level"),
            };
    },
};

pub const RAPID_FIRE_UP: Skill = Skill {
    name: "Rapid Fire Up",
    alt_name: None,
    id: SkillId::RapidFireUp,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const RAPID_MORPH: Skill = Skill {
    name: "Rapid Morph",
    alt_name: None,
    id: SkillId::RapidMorph,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const RAZOR_SHARP: Skill = Skill {
    name: "Razor Sharp",
    alt_name: None,
    id: SkillId::RazorSharp,
    skill_type: SkillType::Weapon,
    max: 5,
    // TODO: Handle Razor Sharp's reduction to Sharpness loss.
    apply: |_modifier, _level, _weapon| {},
};

pub const SLEEP_ATTACK: Skill = Skill {
    name: "Sleep Attack",
    alt_name: None,
    id: SkillId::SleepAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Sleep(_sleep) = weapon.element {
            match level {
                0 => {}
                1 => {
                    modifier.element_multiplier *= 1.05;
                    modifier.bonus_element += 1.0;
                }
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 2.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 5.0;
                }
                _ => unreachable!("Sleep Attack above maximum level"),
            }
        }
    },
};

pub const SLEEP_FUNCTIONALITY: Skill = Skill {
    name: "Sleep Functionality",
    alt_name: None,
    id: SkillId::SleepFunctionality,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const SLICKED_BLADE: Skill = Skill {
    name: "Slicked Blade",
    alt_name: None,
    id: SkillId::SlickedBlade,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SLUGGER: Skill = Skill {
    name: "Slugger",
    alt_name: None,
    id: SkillId::Slugger,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SPECIAL_AMMO_BOOST: Skill = Skill {
    name: "Special Ammo Boost",
    alt_name: None,
    id: SkillId::SpecialAmmoBoost,
    skill_type: SkillType::Weapon,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const SPEED_SHARPENING: Skill = Skill {
    name: "Speed Sharpening",
    alt_name: None,
    id: SkillId::SpeedSharpening,
    skill_type: SkillType::Weapon,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const SPREAD_POWER_SHOTS: Skill = Skill {
    name: "Spread/Power Shots",
    alt_name: None,
    id: SkillId::SpreadPowerShots,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const STAMINA_THIEF: Skill = Skill {
    name: "Stamina Thief",
    alt_name: None,
    id: SkillId::StaminaThief,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const TETRAD_SHOT: Skill = Skill {
    name: "Tetrad Shot",
    alt_name: None,
    id: SkillId::TetradShot,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Implement once the data structure for Bow coatings and Bowgun ammo is complete.
    apply: |_modifier, _level, _weapon| {},
};

pub const THUNDER_ATTACK: Skill = Skill {
    name: "Thunder Attack",
    alt_name: None,
    id: SkillId::ThunderAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Thunder(_thunder) = weapon.element {
            match level {
                0 => {}
                1 => modifier.bonus_element += 4.0,
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 5.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 6.0;
                }
                _ => unreachable!("Thunder Attack above maximum level"),
            }
        }
    },
};

pub const WATER_ATTACK: Skill = Skill {
    name: "Water Attack",
    alt_name: None,
    id: SkillId::WaterAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    apply: |modifier, level, weapon| {
        if let Element::Water(_water) = weapon.element {
            match level {
                0 => {}
                1 => modifier.bonus_element += 4.0,
                2 => {
                    modifier.element_multiplier *= 1.1;
                    modifier.bonus_element += 5.0;
                }
                3 => {
                    modifier.element_multiplier *= 1.2;
                    modifier.bonus_element += 6.0;
                }
                _ => unreachable!("Water Attack above maximum level"),
            }
        }
    },
};

pub const WHITEFLAME_TORRENT: Skill = Skill {
    name: "Whiteflame Torrent",
    alt_name: None,
    id: SkillId::WhiteflameTorrent,
    skill_type: SkillType::Weapon,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};
