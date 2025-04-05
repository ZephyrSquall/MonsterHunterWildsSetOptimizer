use crate::decoration::Decoration;
use crate::skill::SkillAmount;
use crate::skill::armor_skill::{
    ADAPTABILITY, ADRENALINE_RUSH, AGITATOR, AMBUSH, ANTIVIRUS, AQUATIC_OILSILT_MOBILITY,
    BIND_RESISTANCE, BLAST_RESISTANCE, BLEEDING_RESISTANCE, BLINDSIDER, BOMBARDIER, BOTANIST,
    BURST, CLIFFHANGER, COALESCENCE, CONSTITUTION, COUNTERSTRIKE, DEFENSE_BOOST, DIVINE_BLESSING,
    DRAGON_RESISTANCE, EARPLUGS, ENTOMOLOGIST, EVADE_EXTENDER, EVADE_WINDOW, FIRE_RESISTANCE,
    FLAYER, FLINCH_FREE, FORAY, FREE_MEAL, GEOLOGIST, HEROICS, HUNGER_RESISTANCE, ICE_RESISTANCE,
    INTIMIDATOR, IRON_SKIN, ITEM_PROLONGER, JUMP_MASTER, LATENT_POWER, LEAP_OF_FAITH,
    MARATHON_RUNNER, MAXIMUM_MIGHT, MUSHROOMANCER, OUTDOORSMAN, PARALYSIS_RESISTANCE, PARTBREAKER,
    PEAK_PERFORMANCE, POISON_RESISTANCE, QUICK_SHEATHE, RECOVERY_SPEED, RECOVERY_UP, RESENTMENT,
    SELF_IMPROVEMENT, SHOCK_ABSORBER, SLEEP_RESISTANCE, SPEED_EATING, STAMINA_SURGE,
    STENCH_RESISTANCE, STUN_RESISTANCE, SURVIVAL_EXPERT, THUNDER_RESISTANCE, TOOL_SPECIALIST,
    TREMOR_RESISTANCE, WATER_RESISTANCE, WEAKNESS_EXPLOIT, WIDE_RANGE, WINDPROOF,
};

pub const ALL_ARMOR_DECORATIONS: [&Decoration; 66] = [
    &FUROR_JEWEL,
    &COUNTERATTACK_JEWEL,
    &FLAWLESS_JEWEL,
    &COUNTER_JEWEL,
    &TENDERIZER_JEWEL,
    &MIGHTY_JEWEL,
    &DEFENSE_JEWEL,
    &PROTECTION_JEWEL,
    &FIRE_RES_JEWEL,
    &WATER_RES_JEWEL,
    &THUNDER_RES_JEWEL,
    &ICE_RES_JEWEL,
    &DRAGON_RES_JEWEL,
    &ANTIDOTE_JEWEL,
    &ANTIPARA_JEWEL,
    &PEP_JEWEL,
    &DEF_LOCK_JEWEL,
    &STEADFAST_JEWEL,
    &SUTURE_JEWEL,
    &ESCAPE_JEWEL,
    &ANTIBLAST_JEWEL,
    &PERFUME_JEWEL,
    &ADAPT_JEWEL,
    &MEDICINE_JEWEL,
    &RECOVERY_JEWEL,
    &SURVIVAL_JEWEL,
    &PHYSIQUE_JEWEL,
    &REFRESH_JEWEL,
    &SPRINTER_JEWEL,
    &HUNGERLESS_JEWEL,
    &CHALLENGER_JEWEL,
    &THROTTLE_JEWEL,
    &POTENTIAL_JEWEL,
    &CHAIN_JEWEL,
    &FORAY_JEWEL,
    &FLAYER_JEWEL,
    &DESTROYER_JEWEL,
    &PHOENIX_JEWEL,
    &AMBUSH_JEWEL,
    &SANE_JEWEL,
    &GROWTH_JEWEL,
    &EVASION_JEWEL,
    &JUMPING_JEWEL,
    &EARPLUGS_JEWEL,
    &SHEATH_JEWEL,
    &WIND_RESIST_JEWEL,
    &FOOTING_JEWEL,
    &BRACE_JEWEL,
    &INTIMIDATOR_JEWEL,
    &GOBBLER_JEWEL,
    &MIREWALKER_JEWEL,
    &SHOCKPROOF_JEWEL,
    &DIVE_JEWEL,
    &LEAP_JEWEL,
    &CLIMBER_JEWEL,
    &FRIENDSHIP_JEWEL,
    &MAINTENANCE_JEWEL,
    &BOMBER_JEWEL,
    &FUNGIFORM_JEWEL,
    &ENDURING_JEWEL,
    &SATIATED_JEWEL,
    &FLASH_JEWEL,
    &BOTANY_JEWEL,
    &GEOLOGY_JEWEL,
    &SPECIMEN_JEWEL,
    &RANGER_JEWEL,
];

const FUROR_JEWEL: Decoration = Decoration {
    name: "Furor Jewel",
    size: 2,
    skills: &[SkillAmount::new(&RESENTMENT, 1)],
};

const COUNTERATTACK_JEWEL: Decoration = Decoration {
    name: "Counterattack Jewel",
    size: 3,
    skills: &[SkillAmount::new(&ADRENALINE_RUSH, 1)],
};

const FLAWLESS_JEWEL: Decoration = Decoration {
    name: "Flawless Jewel",
    size: 2,
    skills: &[SkillAmount::new(&PEAK_PERFORMANCE, 1)],
};

const COUNTER_JEWEL: Decoration = Decoration {
    name: "Counter Jewel",
    size: 2,
    skills: &[SkillAmount::new(&COUNTERSTRIKE, 1)],
};

const TENDERIZER_JEWEL: Decoration = Decoration {
    name: "Tenderizer Jewel",
    size: 3,
    skills: &[SkillAmount::new(&WEAKNESS_EXPLOIT, 1)],
};

const MIGHTY_JEWEL: Decoration = Decoration {
    name: "Mighty Jewel",
    size: 2,
    skills: &[SkillAmount::new(&MAXIMUM_MIGHT, 1)],
};

const DEFENSE_JEWEL: Decoration = Decoration {
    name: "Defense Jewel",
    size: 1,
    skills: &[SkillAmount::new(&DEFENSE_BOOST, 1)],
};

const PROTECTION_JEWEL: Decoration = Decoration {
    name: "Protection Jewel",
    size: 1,
    skills: &[SkillAmount::new(&DIVINE_BLESSING, 1)],
};

const FIRE_RES_JEWEL: Decoration = Decoration {
    name: "Fire Res Jewel",
    size: 1,
    skills: &[SkillAmount::new(&FIRE_RESISTANCE, 1)],
};

const WATER_RES_JEWEL: Decoration = Decoration {
    name: "Water Res Jewel",
    size: 1,
    skills: &[SkillAmount::new(&WATER_RESISTANCE, 1)],
};

const THUNDER_RES_JEWEL: Decoration = Decoration {
    name: "Thunder Res Jewel",
    size: 1,
    skills: &[SkillAmount::new(&THUNDER_RESISTANCE, 1)],
};

const ICE_RES_JEWEL: Decoration = Decoration {
    name: "Ice Res Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ICE_RESISTANCE, 1)],
};

const DRAGON_RES_JEWEL: Decoration = Decoration {
    name: "Dragon Res Jewel",
    size: 1,
    skills: &[SkillAmount::new(&DRAGON_RESISTANCE, 1)],
};

const ANTIDOTE_JEWEL: Decoration = Decoration {
    name: "Antidote Jewel",
    size: 1,
    skills: &[SkillAmount::new(&POISON_RESISTANCE, 1)],
};

const ANTIPARA_JEWEL: Decoration = Decoration {
    name: "Antipara Jewel",
    size: 1,
    skills: &[SkillAmount::new(&PARALYSIS_RESISTANCE, 1)],
};

const PEP_JEWEL: Decoration = Decoration {
    name: "Pep Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SLEEP_RESISTANCE, 1)],
};

const DEF_LOCK_JEWEL: Decoration = Decoration {
    name: "Def Lock Jewel",
    size: 1,
    skills: &[SkillAmount::new(&IRON_SKIN, 1)],
};

const STEADFAST_JEWEL: Decoration = Decoration {
    name: "Steadfast Jewel",
    size: 1,
    skills: &[SkillAmount::new(&STUN_RESISTANCE, 1)],
};

const SUTURE_JEWEL: Decoration = Decoration {
    name: "Suture Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BLEEDING_RESISTANCE, 1)],
};

const ESCAPE_JEWEL: Decoration = Decoration {
    name: "Escape Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BIND_RESISTANCE, 1)],
};

const ANTIBLAST_JEWEL: Decoration = Decoration {
    name: "Antiblast Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BLAST_RESISTANCE, 1)],
};

const PERFUME_JEWEL: Decoration = Decoration {
    name: "Perfume Jewel",
    size: 1,
    skills: &[SkillAmount::new(&STENCH_RESISTANCE, 1)],
};

const ADAPT_JEWEL: Decoration = Decoration {
    name: "Adapt Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ADAPTABILITY, 1)],
};

const MEDICINE_JEWEL: Decoration = Decoration {
    name: "Medicine Jewel",
    size: 1,
    skills: &[SkillAmount::new(&RECOVERY_UP, 1)],
};

const RECOVERY_JEWEL: Decoration = Decoration {
    name: "Recovery Jewel",
    size: 1,
    skills: &[SkillAmount::new(&RECOVERY_SPEED, 1)],
};

const SURVIVAL_JEWEL: Decoration = Decoration {
    name: "Survival Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SURVIVAL_EXPERT, 1)],
};

const PHYSIQUE_JEWEL: Decoration = Decoration {
    name: "Physique Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CONSTITUTION, 1)],
};

const REFRESH_JEWEL: Decoration = Decoration {
    name: "Refresh Jewel",
    size: 2,
    skills: &[SkillAmount::new(&STAMINA_SURGE, 1)],
};

const SPRINTER_JEWEL: Decoration = Decoration {
    name: "Sprinter Jewel",
    size: 1,
    skills: &[SkillAmount::new(&MARATHON_RUNNER, 1)],
};

const HUNGERLESS_JEWEL: Decoration = Decoration {
    name: "Hungerless Jewel",
    size: 1,
    skills: &[SkillAmount::new(&HUNGER_RESISTANCE, 1)],
};

const CHALLENGER_JEWEL: Decoration = Decoration {
    name: "Challenger Jewel",
    size: 3,
    skills: &[SkillAmount::new(&AGITATOR, 1)],
};

const THROTTLE_JEWEL: Decoration = Decoration {
    name: "Throttle Jewel",
    size: 3,
    skills: &[SkillAmount::new(&LATENT_POWER, 1)],
};

const POTENTIAL_JEWEL: Decoration = Decoration {
    name: "Potential Jewel",
    size: 2,
    skills: &[SkillAmount::new(&HEROICS, 1)],
};

const CHAIN_JEWEL: Decoration = Decoration {
    name: "Chain Jewel",
    size: 3,
    skills: &[SkillAmount::new(&BURST, 1)],
};

const FORAY_JEWEL: Decoration = Decoration {
    name: "Foray Jewel",
    size: 3,
    skills: &[SkillAmount::new(&FORAY, 1)],
};

const FLAYER_JEWEL: Decoration = Decoration {
    name: "Flayer Jewel",
    size: 3,
    skills: &[SkillAmount::new(&FLAYER, 1)],
};

const DESTROYER_JEWEL: Decoration = Decoration {
    name: "Destroyer Jewel",
    size: 2,
    skills: &[SkillAmount::new(&PARTBREAKER, 1)],
};

const PHOENIX_JEWEL: Decoration = Decoration {
    name: "Phoenix Jewel",
    size: 2,
    skills: &[SkillAmount::new(&COALESCENCE, 1)],
};

const AMBUSH_JEWEL: Decoration = Decoration {
    name: "Ambush Jewel",
    size: 2,
    skills: &[SkillAmount::new(&AMBUSH, 1)],
};

const SANE_JEWEL: Decoration = Decoration {
    name: "Sane Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ANTIVIRUS, 1)],
};

const GROWTH_JEWEL: Decoration = Decoration {
    name: "Growth Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SELF_IMPROVEMENT, 1)],
};

const EVASION_JEWEL: Decoration = Decoration {
    name: "Evasion Jewel",
    size: 2,
    skills: &[SkillAmount::new(&EVADE_WINDOW, 1)],
};

const JUMPING_JEWEL: Decoration = Decoration {
    name: "Jumping Jewel",
    size: 2,
    skills: &[SkillAmount::new(&EVADE_EXTENDER, 1)],
};

const EARPLUGS_JEWEL: Decoration = Decoration {
    name: "Earplugs Jewel",
    size: 2,
    skills: &[SkillAmount::new(&EARPLUGS, 1)],
};

const SHEATH_JEWEL: Decoration = Decoration {
    name: "Sheath Jewel",
    size: 1,
    skills: &[SkillAmount::new(&QUICK_SHEATHE, 1)],
};

const WIND_RESIST_JEWEL: Decoration = Decoration {
    name: "Wind Resist Jewel",
    size: 1,
    skills: &[SkillAmount::new(&WINDPROOF, 1)],
};

const FOOTING_JEWEL: Decoration = Decoration {
    name: "Footing Jewel",
    size: 1,
    skills: &[SkillAmount::new(&TREMOR_RESISTANCE, 1)],
};

const BRACE_JEWEL: Decoration = Decoration {
    name: "Brace Jewel",
    size: 1,
    skills: &[SkillAmount::new(&FLINCH_FREE, 1)],
};

const INTIMIDATOR_JEWEL: Decoration = Decoration {
    name: "Intimidator Jewel",
    size: 1,
    skills: &[SkillAmount::new(&INTIMIDATOR, 1)],
};

const GOBBLER_JEWEL: Decoration = Decoration {
    name: "Gobbler Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SPEED_EATING, 1)],
};

const MIREWALKER_JEWEL: Decoration = Decoration {
    name: "Mirewalker Jewel",
    size: 1,
    skills: &[SkillAmount::new(&AQUATIC_OILSILT_MOBILITY, 1)],
};

const SHOCKPROOF_JEWEL: Decoration = Decoration {
    name: "Shockproof Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SHOCK_ABSORBER, 1)],
};

const DIVE_JEWEL: Decoration = Decoration {
    name: "Dive Jewel",
    size: 1,
    skills: &[SkillAmount::new(&LEAP_OF_FAITH, 1)],
};

const LEAP_JEWEL: Decoration = Decoration {
    name: "Leap Jewel",
    size: 1,
    skills: &[SkillAmount::new(&JUMP_MASTER, 1)],
};

const CLIMBER_JEWEL: Decoration = Decoration {
    name: "Climber Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CLIFFHANGER, 1)],
};

const FRIENDSHIP_JEWEL: Decoration = Decoration {
    name: "Friendship Jewel",
    size: 1,
    skills: &[SkillAmount::new(&WIDE_RANGE, 1)],
};

const MAINTENANCE_JEWEL: Decoration = Decoration {
    name: "Maintenance Jewel",
    size: 2,
    skills: &[SkillAmount::new(&TOOL_SPECIALIST, 1)],
};

const BOMBER_JEWEL: Decoration = Decoration {
    name: "Bomber Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BOMBARDIER, 1)],
};

const FUNGIFORM_JEWEL: Decoration = Decoration {
    name: "Fungiform Jewel",
    size: 2,
    skills: &[SkillAmount::new(&MUSHROOMANCER, 1)],
};

const ENDURING_JEWEL: Decoration = Decoration {
    name: "Enduring Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ITEM_PROLONGER, 1)],
};

const SATIATED_JEWEL: Decoration = Decoration {
    name: "Satiated Jewel",
    size: 1,
    skills: &[SkillAmount::new(&FREE_MEAL, 1)],
};

const FLASH_JEWEL: Decoration = Decoration {
    name: "Flash Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BLINDSIDER, 1)],
};

const BOTANY_JEWEL: Decoration = Decoration {
    name: "Botany Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BOTANIST, 1)],
};

const GEOLOGY_JEWEL: Decoration = Decoration {
    name: "Geology Jewel",
    size: 1,
    skills: &[SkillAmount::new(&GEOLOGIST, 1)],
};

const SPECIMEN_JEWEL: Decoration = Decoration {
    name: "Specimen Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ENTOMOLOGIST, 1)],
};

const RANGER_JEWEL: Decoration = Decoration {
    name: "Ranger Jewel",
    size: 1,
    skills: &[SkillAmount::new(&OUTDOORSMAN, 1)],
};
