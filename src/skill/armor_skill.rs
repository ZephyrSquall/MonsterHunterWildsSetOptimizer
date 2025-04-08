use crate::{
    config::{
        AGITATOR_UPTIME, BURST_FIVE_HIT_UPTIME, BURST_ONE_HIT_UPTIME, MAXIMUM_MIGHT_UPTIME,
        WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME, WEAKNESS_EXPLOIT_WOUND_UPTIME,
    },
    skill::{Skill, SkillId, SkillType},
    weapon::WeaponType,
};

pub const ADAPTABILITY: Skill = Skill {
    name: "Adaptability",
    alt_name: None,
    id: SkillId::Adaptability,
    skill_type: SkillType::Armor,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const ADRENALINE_RUSH: Skill = Skill {
    name: "Adrenaline Rush",
    alt_name: None,
    id: SkillId::AdrenalineRush,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const AGITATOR: Skill = Skill {
    name: "Agitator",
    alt_name: None,
    id: SkillId::Agitator,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |modifier, level, _weapon| match level {
        0 => {}
        1 => {
            modifier.bonus_attack += 4.0 * AGITATOR_UPTIME;
            modifier.bonus_affinity += 3.0 * AGITATOR_UPTIME;
        }
        2 => {
            modifier.bonus_attack += 8.0 * AGITATOR_UPTIME;
            modifier.bonus_affinity += 5.0 * AGITATOR_UPTIME;
        }
        3 => {
            modifier.bonus_attack += 12.0 * AGITATOR_UPTIME;
            modifier.bonus_affinity += 7.0 * AGITATOR_UPTIME;
        }
        4 => {
            modifier.bonus_attack += 16.0 * AGITATOR_UPTIME;
            modifier.bonus_affinity += 10.0 * AGITATOR_UPTIME;
        }
        5 => {
            modifier.bonus_attack += 20.0 * AGITATOR_UPTIME;
            modifier.bonus_affinity += 15.0 * AGITATOR_UPTIME;
        }
        _ => unreachable!("Agitator above maximum level"),
    },
};

pub const AMBUSH: Skill = Skill {
    name: "Ambush",
    alt_name: None,
    id: SkillId::Ambush,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ANTIVIRUS: Skill = Skill {
    name: "Antivirus",
    alt_name: None,
    id: SkillId::Antivirus,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const AQUATIC_OILSILT_MOBILITY: Skill = Skill {
    name: "Aquatic/Oilsilt Mobility",
    alt_name: None,
    id: SkillId::AquaticOilsiltMobility,
    skill_type: SkillType::Armor,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const BIND_RESISTANCE: Skill = Skill {
    name: "Bind Resistance",
    alt_name: None,
    id: SkillId::BindResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLAST_RESISTANCE: Skill = Skill {
    name: "Blast Resistance",
    alt_name: None,
    id: SkillId::BlastResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLEEDING_RESISTANCE: Skill = Skill {
    name: "Bleeding Resistance",
    alt_name: None,
    id: SkillId::BleedingResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLIGHT_RESISTANCE: Skill = Skill {
    name: "Blight Resistance",
    alt_name: None,
    id: SkillId::BlightResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BLINDSIDER: Skill = Skill {
    name: "Blindsider",
    alt_name: None,
    id: SkillId::Blindsider,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const BOMBARDIER: Skill = Skill {
    name: "Bombardier",
    alt_name: None,
    id: SkillId::Bombardier,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const BOTANIST: Skill = Skill {
    name: "Botanist",
    alt_name: None,
    id: SkillId::Botanist,
    skill_type: SkillType::Armor,
    max: 4,
    apply: |_modifier, _level, _weapon| {},
};

pub const BURST: Skill = Skill {
    name: "Burst",
    alt_name: None,
    id: SkillId::Burst,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |modifier, level, weapon| {
        match level {
            0 => {}
            1 => match weapon.weapon_type {
                WeaponType::GreatSword | WeaponType::HuntingHorn => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LongSword
                | WeaponType::SwordAndShield
                | WeaponType::Hammer
                | WeaponType::Lance
                | WeaponType::Gunlance
                | WeaponType::SwitchAxe
                | WeaponType::ChargeBlade
                | WeaponType::InsectGlaive => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 6.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::DualBlades => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 4.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LightBowgun | WeaponType::HeavyBowgun => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 6.0 * BURST_FIVE_HIT_UPTIME;
                }
                WeaponType::Bow => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 6.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 4.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
            },
            2 => match weapon.weapon_type {
                WeaponType::GreatSword | WeaponType::HuntingHorn => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LongSword
                | WeaponType::SwordAndShield
                | WeaponType::Hammer
                | WeaponType::Lance
                | WeaponType::Gunlance
                | WeaponType::SwitchAxe
                | WeaponType::ChargeBlade
                | WeaponType::InsectGlaive => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::DualBlades => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 6.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LightBowgun | WeaponType::HeavyBowgun => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 7.0 * BURST_FIVE_HIT_UPTIME;
                }
                WeaponType::Bow => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 7.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 6.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
            },
            3 => match weapon.weapon_type {
                WeaponType::GreatSword | WeaponType::HuntingHorn => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 14.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LongSword
                | WeaponType::SwordAndShield
                | WeaponType::Hammer
                | WeaponType::Lance
                | WeaponType::Gunlance
                | WeaponType::SwitchAxe
                | WeaponType::ChargeBlade
                | WeaponType::InsectGlaive => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::DualBlades => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LightBowgun | WeaponType::HeavyBowgun => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                }
                WeaponType::Bow => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 8.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
            },
            4 => match weapon.weapon_type {
                WeaponType::GreatSword | WeaponType::HuntingHorn => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 16.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 16.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LongSword
                | WeaponType::SwordAndShield
                | WeaponType::Hammer
                | WeaponType::Lance
                | WeaponType::Gunlance
                | WeaponType::SwitchAxe
                | WeaponType::ChargeBlade
                | WeaponType::InsectGlaive => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 15.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::DualBlades => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 15.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LightBowgun | WeaponType::HeavyBowgun => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 9.0 * BURST_FIVE_HIT_UPTIME;
                }
                WeaponType::Bow => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 9.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
            },
            5 => match weapon.weapon_type {
                WeaponType::GreatSword | WeaponType::HuntingHorn => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 20.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 20.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LongSword
                | WeaponType::SwordAndShield
                | WeaponType::Hammer
                | WeaponType::Lance
                | WeaponType::Gunlance
                | WeaponType::SwitchAxe
                | WeaponType::ChargeBlade
                | WeaponType::InsectGlaive => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 18.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            5.0 * BURST_ONE_HIT_UPTIME + 14.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::DualBlades => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 18.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
                WeaponType::LightBowgun | WeaponType::HeavyBowgun => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                }
                WeaponType::Bow => {
                    modifier.bonus_attack +=
                        5.0 * BURST_ONE_HIT_UPTIME + 10.0 * BURST_FIVE_HIT_UPTIME;
                    if weapon.is_elemental() {
                        modifier.bonus_element +=
                            3.0 * BURST_ONE_HIT_UPTIME + 12.0 * BURST_FIVE_HIT_UPTIME;
                    }
                }
            },
            _ => unreachable!("Burst above maximum level"),
        };
    },
};

pub const CLIFFHANGER: Skill = Skill {
    name: "Cliffhanger",
    alt_name: None,
    id: SkillId::Cliffhanger,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const COALESCENCE: Skill = Skill {
    name: "Coalescence",
    alt_name: None,
    id: SkillId::Coalescence,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const CONSTITUTION: Skill = Skill {
    name: "Constitution",
    alt_name: None,
    id: SkillId::Constitution,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const CONVERT_ELEMENT: Skill = Skill {
    name: "Convert Element",
    alt_name: None,
    id: SkillId::ConvertElement,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const COUNTERSTRIKE: Skill = Skill {
    name: "Counterstrike",
    alt_name: None,
    id: SkillId::Counterstrike,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const DEFENSE_BOOST: Skill = Skill {
    name: "Defense Boost",
    alt_name: None,
    id: SkillId::DefenseBoost,
    skill_type: SkillType::Armor,
    max: 7,
    apply: |_modifier, _level, _weapon| {},
};

pub const DIVINE_BLESSING: Skill = Skill {
    name: "Divine Blessing",
    alt_name: None,
    id: SkillId::DivineBlessing,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const DRAGON_RESISTANCE: Skill = Skill {
    name: "Dragon Resistance",
    alt_name: None,
    id: SkillId::DragonResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const EARPLUGS: Skill = Skill {
    name: "Earplugs",
    alt_name: None,
    id: SkillId::Earplugs,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ELEMENTAL_ABSORPTION: Skill = Skill {
    name: "Elemental Absorption",
    alt_name: None,
    id: SkillId::ElementalAbsorption,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ENTOMOLOGIST: Skill = Skill {
    name: "Entomologist",
    alt_name: None,
    id: SkillId::Entomologist,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const EVADE_EXTENDER: Skill = Skill {
    name: "Evade Extender",
    alt_name: None,
    id: SkillId::EvadeExtender,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const EVADE_WINDOW: Skill = Skill {
    name: "Evade Window",
    alt_name: None,
    id: SkillId::EvadeWindow,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const FIRE_RESISTANCE: Skill = Skill {
    name: "Fire Resistance",
    alt_name: None,
    id: SkillId::FireResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const FLAYER: Skill = Skill {
    name: "Flayer",
    alt_name: None,
    id: SkillId::Flayer,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const FLINCH_FREE: Skill = Skill {
    name: "Flinch Free",
    alt_name: None,
    id: SkillId::FlinchFree,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const FORAY: Skill = Skill {
    name: "Foray",
    alt_name: None,
    id: SkillId::Foray,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const FREE_MEAL: Skill = Skill {
    name: "Free Meal",
    alt_name: None,
    id: SkillId::FreeMeal,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const GEOLOGIST: Skill = Skill {
    name: "Geologist",
    alt_name: None,
    id: SkillId::Geologist,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const HEROICS: Skill = Skill {
    name: "Heroics",
    alt_name: None,
    id: SkillId::Heroics,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const HUNGER_RESISTANCE: Skill = Skill {
    name: "Hunger Resistance",
    alt_name: None,
    id: SkillId::HungerResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ICE_RESISTANCE: Skill = Skill {
    name: "Ice Resistance",
    alt_name: None,
    id: SkillId::IceResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const INTIMIDATOR: Skill = Skill {
    name: "Intimidator",
    alt_name: None,
    id: SkillId::Intimidator,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const IRON_SKIN: Skill = Skill {
    name: "Iron Skin",
    alt_name: None,
    id: SkillId::IronSkin,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const ITEM_PROLONGER: Skill = Skill {
    name: "Item Prolonger",
    alt_name: None,
    id: SkillId::ItemProlonger,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const JUMP_MASTER: Skill = Skill {
    name: "Jump Master",
    alt_name: None,
    id: SkillId::JumpMaster,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const LATENT_POWER: Skill = Skill {
    name: "Latent Power",
    alt_name: None,
    id: SkillId::LatentPower,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const LEAP_OF_FAITH: Skill = Skill {
    name: "Leap of Faith",
    alt_name: None,
    id: SkillId::LeapOfFaith,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const MARATHON_RUNNER: Skill = Skill {
    name: "Marathon Runner",
    alt_name: None,
    id: SkillId::MarathonRunner,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const MAXIMUM_MIGHT: Skill = Skill {
    name: "Maximum Might",
    alt_name: None,
    id: SkillId::MaximumMight,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |modifier, level, _weapon| match level {
        0 => {}
        1 => modifier.bonus_affinity += 10.0 * MAXIMUM_MIGHT_UPTIME,
        2 => modifier.bonus_affinity += 20.0 * MAXIMUM_MIGHT_UPTIME,
        3 => modifier.bonus_affinity += 30.0 * MAXIMUM_MIGHT_UPTIME,
        _ => unreachable!("Maximum Might above maximum level"),
    },
};

pub const MUSHROOMANCER: Skill = Skill {
    name: "Mushroomancer",
    alt_name: None,
    id: SkillId::Mushroomancer,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const OUTDOORSMAN: Skill = Skill {
    name: "Outdoorsman",
    alt_name: None,
    id: SkillId::Outdoorsman,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const PARALYSIS_RESISTANCE: Skill = Skill {
    name: "Paralysis Resistance",
    alt_name: None,
    id: SkillId::ParalysisResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const PARTBREAKER: Skill = Skill {
    name: "Partbreaker",
    alt_name: None,
    id: SkillId::Partbreaker,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const PEAK_PERFORMANCE: Skill = Skill {
    name: "Peak Performance",
    alt_name: None,
    id: SkillId::PeakPerformance,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const POISON_RESISTANCE: Skill = Skill {
    name: "Poison Resistance",
    alt_name: None,
    id: SkillId::PoisonResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const QUICK_SHEATHE: Skill = Skill {
    name: "Quick Sheathe",
    alt_name: None,
    id: SkillId::QuickSheathe,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const RECOVERY_SPEED: Skill = Skill {
    name: "Recovery Speed",
    alt_name: None,
    id: SkillId::RecoverySpeed,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const RECOVERY_UP: Skill = Skill {
    name: "Recovery Up",
    alt_name: None,
    id: SkillId::RecoveryUp,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const RESENTMENT: Skill = Skill {
    name: "Resentment",
    alt_name: None,
    id: SkillId::Resentment,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const SELF_IMPROVEMENT: Skill = Skill {
    name: "Self-Improvement",
    alt_name: None,
    id: SkillId::SelfImprovement,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const SHOCK_ABSORBER: Skill = Skill {
    name: "Shock Absorber",
    alt_name: None,
    id: SkillId::ShockAbsorber,
    skill_type: SkillType::Armor,
    max: 1,
    apply: |_modifier, _level, _weapon| {},
};

pub const SLEEP_RESISTANCE: Skill = Skill {
    name: "Sleep Resistance",
    alt_name: None,
    id: SkillId::SleepResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SPEED_EATING: Skill = Skill {
    name: "Speed Eating",
    alt_name: None,
    id: SkillId::SpeedEating,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const STAMINA_SURGE: Skill = Skill {
    name: "Stamina Surge",
    alt_name: None,
    id: SkillId::StaminaSurge,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const STENCH_RESISTANCE: Skill = Skill {
    name: "Stench Resistance",
    alt_name: None,
    id: SkillId::StenchResistance,
    skill_type: SkillType::Armor,
    max: 2,
    apply: |_modifier, _level, _weapon| {},
};

pub const STUN_RESISTANCE: Skill = Skill {
    name: "Stun Resistance",
    alt_name: None,
    id: SkillId::StunResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const SURVIVAL_EXPERT: Skill = Skill {
    name: "Survival Expert",
    alt_name: None,
    id: SkillId::SurvivalExpert,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const THUNDER_RESISTANCE: Skill = Skill {
    name: "Thunder Resistance",
    alt_name: None,
    id: SkillId::ThunderResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const TOOL_SPECIALIST: Skill = Skill {
    name: "Tool Specialist",
    alt_name: None,
    id: SkillId::ToolSpecialist,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const TREMOR_RESISTANCE: Skill = Skill {
    name: "Tremor Resistance",
    alt_name: None,
    id: SkillId::TremorResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const WATER_RESISTANCE: Skill = Skill {
    name: "Water Resistance",
    alt_name: None,
    id: SkillId::WaterResistance,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};

pub const WEAKNESS_EXPLOIT: Skill = Skill {
    name: "Weakness Exploit",
    alt_name: None,
    id: SkillId::WeaknessExploit,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |modifier, level, _weapon| match level {
        0 => {}
        1 => {
            modifier.bonus_affinity +=
                5.0 * WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME + 8.0 * WEAKNESS_EXPLOIT_WOUND_UPTIME;
        }
        2 => {
            modifier.bonus_affinity +=
                10.0 * WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME + 15.0 * WEAKNESS_EXPLOIT_WOUND_UPTIME;
        }
        3 => {
            modifier.bonus_affinity +=
                15.0 * WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME + 25.0 * WEAKNESS_EXPLOIT_WOUND_UPTIME;
        }
        4 => {
            modifier.bonus_affinity +=
                20.0 * WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME + 35.0 * WEAKNESS_EXPLOIT_WOUND_UPTIME;
        }
        5 => {
            modifier.bonus_affinity +=
                30.0 * WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME + 50.0 * WEAKNESS_EXPLOIT_WOUND_UPTIME;
        }
        _ => unreachable!("Weakness Exploit above maximum level"),
    },
};

pub const WIDE_RANGE: Skill = Skill {
    name: "Wide-Range",
    alt_name: None,
    id: SkillId::WideRange,
    skill_type: SkillType::Armor,
    max: 5,
    apply: |_modifier, _level, _weapon| {},
};

pub const WINDPROOF: Skill = Skill {
    name: "Windproof",
    alt_name: None,
    id: SkillId::Windproof,
    skill_type: SkillType::Armor,
    max: 3,
    apply: |_modifier, _level, _weapon| {},
};
