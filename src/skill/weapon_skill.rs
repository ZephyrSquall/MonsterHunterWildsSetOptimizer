use crate::config::{
    AIRBORNE_COVERAGE, CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE, OFFENSIVE_GUARD_UPTIME,
};
use crate::skill::{DEFAULT_MODIFIER, Modifier, Skill, SkillId, SkillType};
use crate::weapon::{Element, WeaponType};

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

pub const BALLISTICS: Skill = Skill {
    name: "Ballistics",
    alt_name: None,
    id: SkillId::Ballistics,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const BLAST_ATTACK: Skill = Skill {
    name: "Blast Attack",
    alt_name: None,
    id: SkillId::BlastAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Blast(blast) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += f64::from(blast) * 0.05 + 1.0,
                    2 => *bonus_element += f64::from(blast) * 0.1 + 2.0,
                    3 => *bonus_element += f64::from(blast) * 0.2 + 5.0,
                    _ => panic!("Blast Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const BLUDGEONER: Skill = Skill {
    name: "Bludgeoner",
    alt_name: None,
    id: SkillId::Bludgeoner,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const CHARGE_MASTER: Skill = Skill {
    name: "Charge Master",
    alt_name: None,
    id: SkillId::ChargeMaster,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
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

pub const DRAGON_ATTACK: Skill = Skill {
    name: "Dragon Attack",
    alt_name: None,
    id: SkillId::DragonAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Dragon(dragon) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += 4.0,
                    2 => *bonus_element += f64::from(dragon) * 0.1 + 5.0,
                    3 => *bonus_element += f64::from(dragon) * 0.2 + 6.0,
                    _ => panic!("Dragon Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const FIRE_ATTACK: Skill = Skill {
    name: "Fire Attack",
    alt_name: None,
    id: SkillId::FireAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Also handle Fire Attack's effect on the Rathalos's Flare (Scorcher) skill.
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Fire(fire) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += 4.0,
                    2 => *bonus_element += f64::from(fire) * 0.1 + 5.0,
                    3 => *bonus_element += f64::from(fire) * 0.2 + 6.0,
                    _ => panic!("Fire Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const FOCUS: Skill = Skill {
    name: "Focus",
    alt_name: None,
    id: SkillId::Focus,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
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

pub const HANDICRAFT: Skill = Skill {
    name: "Handicraft",
    alt_name: None,
    id: SkillId::Handicraft,
    skill_type: SkillType::Weapon,
    max: 5,
    // TODO: Handle Handicraft's boost to Sharpness.
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const ICE_ATTACK: Skill = Skill {
    name: "Ice Attack",
    alt_name: None,
    id: SkillId::IceAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Ice(ice) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += 4.0,
                    2 => *bonus_element += f64::from(ice) * 0.1 + 5.0,
                    3 => *bonus_element += f64::from(ice) * 0.2 + 6.0,
                    _ => panic!("Ice Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
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

pub const MINDS_EYE: Skill = Skill {
    name: "Mind's Eye",
    alt_name: None,
    id: SkillId::MindsEye,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const NORMAL_SHOTS: Skill = Skill {
    name: "Normal Shots",
    alt_name: None,
    id: SkillId::NormalShots,
    skill_type: SkillType::Weapon,
    max: 1,
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

pub const OPENING_SHOT: Skill = Skill {
    name: "Opening Shot",
    alt_name: None,
    id: SkillId::OpeningShot,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Implement once the data structure for Bow coatings and Bowgun ammo is complete.
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const PARALYSIS_ATTACK: Skill = Skill {
    name: "Paralysis Attack",
    alt_name: None,
    id: SkillId::ParalysisAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Paralysis(paralysis) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += f64::from(paralysis) * 0.05 + 1.0,
                    2 => *bonus_element += f64::from(paralysis) * 0.1 + 2.0,
                    3 => *bonus_element += f64::from(paralysis) * 0.2 + 5.0,
                    _ => panic!("Paralysis Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const PIERCING_SHOTS: Skill = Skill {
    name: "Piercing Shots",
    alt_name: None,
    id: SkillId::PiercingShots,
    skill_type: SkillType::Weapon,
    max: 1,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const POISON_ATTACK: Skill = Skill {
    name: "Poison Attack",
    alt_name: None,
    id: SkillId::PoisonAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Poison(poison) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += f64::from(poison) * 0.05 + 1.0,
                    2 => *bonus_element += f64::from(poison) * 0.1 + 2.0,
                    3 => *bonus_element += f64::from(poison) * 0.2 + 5.0,
                    _ => panic!("Poison Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const POWER_PROLONGER: Skill = Skill {
    name: "Power Prolonger",
    alt_name: None,
    id: SkillId::PowerProlonger,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const PROTECTIVE_POLISH: Skill = Skill {
    name: "Protective Polish",
    alt_name: None,
    id: SkillId::ProtectivePolish,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
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

pub const RAPID_FIRE_UP: Skill = Skill {
    name: "Rapid Fire Up",
    alt_name: None,
    id: SkillId::RapidFireUp,
    skill_type: SkillType::Weapon,
    max: 1,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const RAPID_MORPH: Skill = Skill {
    name: "Rapid Morph",
    alt_name: None,
    id: SkillId::RapidMorph,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const RAZOR_SHARP: Skill = Skill {
    name: "Razor Sharp",
    alt_name: None,
    id: SkillId::RazorSharp,
    skill_type: SkillType::Weapon,
    max: 5,
    // TODO: Handle Razor Sharp's reduction to Sharpness loss.
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const SLEEP_ATTACK: Skill = Skill {
    name: "Sleep Attack",
    alt_name: None,
    id: SkillId::SleepAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Sleep(sleep) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += f64::from(sleep) * 0.05 + 1.0,
                    2 => *bonus_element += f64::from(sleep) * 0.1 + 2.0,
                    3 => *bonus_element += f64::from(sleep) * 0.2 + 5.0,
                    _ => panic!("Sleep Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const SLUGGER: Skill = Skill {
    name: "Slugger",
    alt_name: None,
    id: SkillId::Slugger,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const SPEED_SHARPENING: Skill = Skill {
    name: "Speed Sharpening",
    alt_name: None,
    id: SkillId::SpeedSharpening,
    skill_type: SkillType::Weapon,
    max: 2,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const SPREAD_POWER_SHOTS: Skill = Skill {
    name: "Spread/Power Shots",
    alt_name: None,
    id: SkillId::SpreadPowerShots,
    skill_type: SkillType::Weapon,
    max: 1,
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const TETRAD_SHOT: Skill = Skill {
    name: "Tetrad Shot",
    alt_name: None,
    id: SkillId::TetradShot,
    skill_type: SkillType::Weapon,
    max: 3,
    // TODO: Implement once the data structure for Bow coatings and Bowgun ammo is complete.
    modifier: Modifier { ..DEFAULT_MODIFIER },
};

pub const THUNDER_ATTACK: Skill = Skill {
    name: "Thunder Attack",
    alt_name: None,
    id: SkillId::ThunderAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Thunder(thunder) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += 4.0,
                    2 => *bonus_element += f64::from(thunder) * 0.1 + 5.0,
                    3 => *bonus_element += f64::from(thunder) * 0.2 + 6.0,
                    _ => panic!("Thunder Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};

pub const WATER_ATTACK: Skill = Skill {
    name: "Water Attack",
    alt_name: None,
    id: SkillId::WaterAttack,
    skill_type: SkillType::Weapon,
    max: 3,
    modifier: Modifier {
        element: |level, bonus_element, weapon| {
            if let Element::Water(water) = weapon.element {
                match level {
                    0 => {}
                    1 => *bonus_element += 4.0,
                    2 => *bonus_element += f64::from(water) * 0.1 + 5.0,
                    3 => *bonus_element += f64::from(water) * 0.2 + 6.0,
                    _ => panic!("Water Attack above maximum level"),
                }
            }
        },
        ..DEFAULT_MODIFIER
    },
};
