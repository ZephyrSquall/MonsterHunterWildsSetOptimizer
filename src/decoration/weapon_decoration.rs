use crate::decoration::Decoration;
use crate::skill::SkillAmount;
use crate::skill::weapon_skill::{
    AIRBORNE, ARTILLERY, ATTACK_BOOST, BALLISTICS, BLAST_ATTACK, BLAST_FUNCTIONALITY, BLUDGEONER,
    CHARGE_MASTER, CHARGE_UP, CRITICAL_BOOST, CRITICAL_DRAW, CRITICAL_ELEMENT, CRITICAL_EYE,
    CRITICAL_STATUS, DRAGON_ATTACK, EXHAUST_FUNCTIONALITY, FIRE_ATTACK, FOCUS, GUARD, GUARD_UP,
    HANDICRAFT, HORN_MAESTRO, ICE_ATTACK, LOAD_SHELLS, MASTERS_TOUCH, MINDS_EYE, NORMAL_SHOTS,
    OFFENSIVE_GUARD, OPENING_SHOT, PARA_FUNCTIONALITY, PARALYSIS_ATTACK, PIERCING_SHOTS,
    POISON_ATTACK, POISON_DURATION_UP, POISON_FUNCTIONALITY, POWER_PROLONGER, PROTECTIVE_POLISH,
    PUNISHING_DRAW, RAPID_FIRE_UP, RAPID_MORPH, RAZOR_SHARP, SLEEP_ATTACK, SLEEP_FUNCTIONALITY,
    SLUGGER, SPECIAL_AMMO_BOOST, SPEED_SHARPENING, SPREAD_POWER_SHOTS, STAMINA_THIEF, TETRAD_SHOT,
    THUNDER_ATTACK, WATER_ATTACK,
};

const ATTACK_JEWEL: Decoration = Decoration {
    name: "Attack Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ATTACK_BOOST, 1)],
};

const ATTACK_JEWEL_II: Decoration = Decoration {
    name: "Attack Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&ATTACK_BOOST, 2)],
};

const ATTACK_JEWEL_III: Decoration = Decoration {
    name: "Attack Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&ATTACK_BOOST, 3)],
};

const GUARDIAN_JEWEL: Decoration = Decoration {
    name: "Guardian Jewel",
    size: 1,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 1)],
};

const GUARDIAN_JEWEL_II: Decoration = Decoration {
    name: "Guardian Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 2)],
};

const GUARDIAN_JEWEL_III: Decoration = Decoration {
    name: "Guardian Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 3)],
};

const GUARDIAN_BLAZE_JWL: Decoration = Decoration {
    name: "Guardian/Blaze Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const GUARDIAN_STREAM_JWL: Decoration = Decoration {
    name: "Guardian/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const GUARDIAN_FROST_JWL: Decoration = Decoration {
    name: "Guardian/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const GUARDIAN_BOLT_JWL: Decoration = Decoration {
    name: "Guardian/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const GUARDIAN_DRAGON_JWL: Decoration = Decoration {
    name: "Guardian/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const GUARDIAN_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Guardian/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const GUARDIAN_IRONWALL_JWL: Decoration = Decoration {
    name: "Guardian/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OFFENSIVE_GUARD, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const EXPERT_JEWEL: Decoration = Decoration {
    name: "Expert Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CRITICAL_EYE, 1)],
};

const EXPERT_JEWEL_II: Decoration = Decoration {
    name: "Expert Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CRITICAL_EYE, 2)],
};

const EXPERT_JEWEL_III: Decoration = Decoration {
    name: "Expert Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&CRITICAL_EYE, 3)],
};

const CRITICAL_JEWEL: Decoration = Decoration {
    name: "Critical Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CRITICAL_BOOST, 1)],
};

const CRITICAL_JEWEL_II: Decoration = Decoration {
    name: "Critical Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CRITICAL_BOOST, 2)],
};

const CRITICAL_JEWEL_III: Decoration = Decoration {
    name: "Critical Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&CRITICAL_BOOST, 3)],
};

const DRAW_JEWEL: Decoration = Decoration {
    name: "Draw Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CRITICAL_DRAW, 1)],
};

const DRAW_JEWEL_II: Decoration = Decoration {
    name: "Draw Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CRITICAL_DRAW, 2)],
};

const DRAW_JEWEL_III: Decoration = Decoration {
    name: "Draw Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&CRITICAL_DRAW, 3)],
};

const BLAZE_JEWEL: Decoration = Decoration {
    name: "Blaze Jewel",
    size: 1,
    skills: &[SkillAmount::new(&FIRE_ATTACK, 1)],
};

const BLAZE_JEWEL_II: Decoration = Decoration {
    name: "Blaze Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&FIRE_ATTACK, 2)],
};

const BLAZE_JEWEL_III: Decoration = Decoration {
    name: "Blaze Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&FIRE_ATTACK, 3)],
};

const BLAZE_GUARDIAN_JEWEL: Decoration = Decoration {
    name: "Blaze/Guardian Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
};

const BLAZE_CRIT_ELEM_JWL: Decoration = Decoration {
    name: "Blaze/Crit Elem Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
};

const BLAZE_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Blaze/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const BLAZE_RZR_SHARP_JWL: Decoration = Decoration {
    name: "Blaze/Rzr Sharp Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&RAZOR_SHARP, 1),
    ],
};

const BLAZE_PRECISE_JEWEL: Decoration = Decoration {
    name: "Blaze/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const BLAZE_OPENER_JEWEL: Decoration = Decoration {
    name: "Blaze/Opener Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&OPENING_SHOT, 1),
    ],
};

const BLAZE_BANDOLIER_JWL: Decoration = Decoration {
    name: "Blaze/Bandolier Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&TETRAD_SHOT, 1),
    ],
};

const BLAZE_FOCUS_JEWEL: Decoration = Decoration {
    name: "Blaze/Focus Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&FOCUS, 1),
    ],
};

const BLAZE_ENHANCER_JWL: Decoration = Decoration {
    name: "Blaze/Enhancer Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&POWER_PROLONGER, 1),
    ],
};

const BLAZE_KO_JEWEL: Decoration = Decoration {
    name: "Blaze/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const BLAZE_QUICKSWITCH_JWL: Decoration = Decoration {
    name: "Blaze/Quickswitch Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&RAPID_MORPH, 1),
    ],
};

const BLAZE_IRONWALL_JWL: Decoration = Decoration {
    name: "Blaze/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const BLAZE_SHIELD_JEWEL: Decoration = Decoration {
    name: "Blaze/Shield Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FIRE_ATTACK, 3),
        SkillAmount::new(&GUARD_UP, 1),
    ],
};

const STREAM_JEWEL: Decoration = Decoration {
    name: "Stream Jewel",
    size: 1,
    skills: &[SkillAmount::new(&WATER_ATTACK, 1)],
};

const STREAM_JEWEL_II: Decoration = Decoration {
    name: "Stream Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&WATER_ATTACK, 2)],
};

const STREAM_JEWEL_III: Decoration = Decoration {
    name: "Stream Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&WATER_ATTACK, 3)],
};

const STREAM_GUARDIAN_JWL: Decoration = Decoration {
    name: "Stream/Guardian Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
};

const STREAM_CRIT_ELEM_JWL: Decoration = Decoration {
    name: "Stream/Crit Elem Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
};

const STREAM_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Stream/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const STREAM_RZR_SHARP_JWL: Decoration = Decoration {
    name: "Stream/Rzr Sharp Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&RAZOR_SHARP, 1),
    ],
};

const STREAM_PRECISE_JEWEL: Decoration = Decoration {
    name: "Stream/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const STREAM_OPENER_JWL: Decoration = Decoration {
    name: "Stream/Opener Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&OPENING_SHOT, 1),
    ],
};

const STREAM_BANDOLIER_JWL: Decoration = Decoration {
    name: "Stream/Bandolier Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&TETRAD_SHOT, 1),
    ],
};

const STREAM_FOCUS_JEWEL: Decoration = Decoration {
    name: "Stream/Focus Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&FOCUS, 1),
    ],
};

const STREAM_ENHANCER_JWL: Decoration = Decoration {
    name: "Stream/Enhancer Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&POWER_PROLONGER, 1),
    ],
};

const STREAM_KO_JEWEL: Decoration = Decoration {
    name: "Stream/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const STREAM_QUICKSWITCH_JWL: Decoration = Decoration {
    name: "Stream/Quickswitch Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&RAPID_MORPH, 1),
    ],
};

const STREAM_IRONWALL_JWL: Decoration = Decoration {
    name: "Stream/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const STREAM_SHIELD_JEWEL: Decoration = Decoration {
    name: "Stream/Shield Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&WATER_ATTACK, 3),
        SkillAmount::new(&GUARD_UP, 1),
    ],
};

const BOLT_JEWEL: Decoration = Decoration {
    name: "Bolt Jewel",
    size: 1,
    skills: &[SkillAmount::new(&THUNDER_ATTACK, 1)],
};

const BOLT_JEWEL_II: Decoration = Decoration {
    name: "Bolt Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&THUNDER_ATTACK, 2)],
};

const BOLT_JEWEL_III: Decoration = Decoration {
    name: "Bolt Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&THUNDER_ATTACK, 3)],
};

const BOLT_GUARDIAN_JWL: Decoration = Decoration {
    name: "Bolt/Guardian Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
};

const BOLT_CRIT_ELEM_JWL: Decoration = Decoration {
    name: "Bolt/Crit Elem Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
};

const BOLT_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Bolt/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const BOLT_RZR_SHARP_JWL: Decoration = Decoration {
    name: "Bolt/Rzr Sharp Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&RAZOR_SHARP, 1),
    ],
};

const BOLT_PRECISE_JEWEL: Decoration = Decoration {
    name: "Bolt/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const BOLT_OPENER_JEWEL: Decoration = Decoration {
    name: "Bolt/Opener Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&OPENING_SHOT, 1),
    ],
};

const BOLT_BANDOLIER_JWL: Decoration = Decoration {
    name: "Bolt/Bandolier Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&TETRAD_SHOT, 1),
    ],
};

const BOLT_FOCUS_JEWEL: Decoration = Decoration {
    name: "Bolt/Focus Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&FOCUS, 1),
    ],
};

const BOLT_ENHANCER_JWL: Decoration = Decoration {
    name: "Bolt/Enhancer Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&POWER_PROLONGER, 1),
    ],
};

const BOLT_KO_JEWEL: Decoration = Decoration {
    name: "Bolt/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const BOLT_QUICKSWITCH_JWL: Decoration = Decoration {
    name: "Bolt/Quickswitch Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&RAPID_MORPH, 1),
    ],
};

const BOLT_IRONWALL_JEWEL: Decoration = Decoration {
    name: "Bolt/Ironwall Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const BOLT_SHIELD_JEWEL: Decoration = Decoration {
    name: "Bolt/Shield Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&THUNDER_ATTACK, 3),
        SkillAmount::new(&GUARD_UP, 1),
    ],
};

const FROST_JEWEL: Decoration = Decoration {
    name: "Frost Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ICE_ATTACK, 1)],
};

const FROST_JEWEL_II: Decoration = Decoration {
    name: "Frost Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&ICE_ATTACK, 2)],
};

const FROST_JEWEL_III: Decoration = Decoration {
    name: "Frost Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&ICE_ATTACK, 3)],
};

const FROST_GUARDIAN_JWL: Decoration = Decoration {
    name: "Frost/Guardian Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
};

const FROST_CRIT_ELEM_JWL: Decoration = Decoration {
    name: "Frost/Crit Elem Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
};

const FROST_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Frost/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const FROST_RZR_SHARP_JWL: Decoration = Decoration {
    name: "Frost/Rzr Sharp Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&RAZOR_SHARP, 1),
    ],
};

const FROST_PRECISE_JEWEL: Decoration = Decoration {
    name: "Frost/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const FROST_OPENER_JEWEL: Decoration = Decoration {
    name: "Frost/Opener Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&OPENING_SHOT, 1),
    ],
};

const FROST_BANDOLIER_JWL: Decoration = Decoration {
    name: "Frost/Bandolier Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&TETRAD_SHOT, 1),
    ],
};

const FROST_FOCUS_JEWEL: Decoration = Decoration {
    name: "Frost/Focus Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&FOCUS, 1),
    ],
};

const FROST_ENHANCER_JWL: Decoration = Decoration {
    name: "Frost/Enhancer Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&POWER_PROLONGER, 1),
    ],
};

const FROST_KO_JEWEL: Decoration = Decoration {
    name: "Frost/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const FROST_QUICKSWITCH_JWL: Decoration = Decoration {
    name: "Frost/Quickswitch Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&RAPID_MORPH, 1),
    ],
};

const FROST_IRONWALL_JWL: Decoration = Decoration {
    name: "Frost/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const FROST_SHIELD_JEWEL: Decoration = Decoration {
    name: "Frost/Shield Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&ICE_ATTACK, 3),
        SkillAmount::new(&GUARD_UP, 1),
    ],
};

const DRAGON_JEWEL: Decoration = Decoration {
    name: "Dragon Jewel",
    size: 1,
    skills: &[SkillAmount::new(&DRAGON_ATTACK, 1)],
};

const DRAGON_JEWEL_II: Decoration = Decoration {
    name: "Dragon Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&DRAGON_ATTACK, 2)],
};

const DRAGON_JEWEL_III: Decoration = Decoration {
    name: "Dragon Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&DRAGON_ATTACK, 3)],
};

const DRAGON_GUARDIAN_JWL: Decoration = Decoration {
    name: "Dragon/Guardian Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
};

const DRAGON_CRIT_ELEM_JWL: Decoration = Decoration {
    name: "Dragon/Crit Elem Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
};

const DRAGON_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Dragon/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const DRAGON_RZR_SHARP_JWL: Decoration = Decoration {
    name: "Dragon/Rzr Sharp Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&RAZOR_SHARP, 1),
    ],
};

const DRAGON_PRECISE_JEWEL: Decoration = Decoration {
    name: "Dragon/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const DRAGON_OPENER_JEWEL: Decoration = Decoration {
    name: "Dragon/Opener Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&OPENING_SHOT, 1),
    ],
};

const DRAGON_BANDOLIER_JWL: Decoration = Decoration {
    name: "Dragon/Bandolier Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&TETRAD_SHOT, 1),
    ],
};

const DRAGON_FOCUS_JEWEL: Decoration = Decoration {
    name: "Dragon/Focus Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&FOCUS, 1),
    ],
};

const DRAGON_ENHANCER_JWL: Decoration = Decoration {
    name: "Dragon/Enhancer Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&POWER_PROLONGER, 1),
    ],
};

const DRAGON_KO_JEWEL: Decoration = Decoration {
    name: "Dragon/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const DRAGON_QUICKSWITCH_JWL: Decoration = Decoration {
    name: "Dragon/Quickswitch Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&RAPID_MORPH, 1),
    ],
};

const DRAGON_IRONWALL_JWL: Decoration = Decoration {
    name: "Dragon/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const DRAGON_SHIELD_JEWEL: Decoration = Decoration {
    name: "Dragon/Shield Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&DRAGON_ATTACK, 3),
        SkillAmount::new(&GUARD_UP, 1),
    ],
};

const VENOM_JEWEL: Decoration = Decoration {
    name: "Venom Jewel",
    size: 1,
    skills: &[SkillAmount::new(&POISON_ATTACK, 1)],
};

const VENOM_JEWEL_II: Decoration = Decoration {
    name: "Venom Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&POISON_ATTACK, 2)],
};

const VENOM_JEWEL_III: Decoration = Decoration {
    name: "Venom Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&POISON_ATTACK, 3)],
};

const PARALYZER_JEWEL: Decoration = Decoration {
    name: "Paralyzer Jewel",
    size: 1,
    skills: &[SkillAmount::new(&PARALYSIS_ATTACK, 1)],
};

const PARALYZER_JEWEL_II: Decoration = Decoration {
    name: "Paralyzer Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&PARALYSIS_ATTACK, 2)],
};

const PARALYZER_JEWEL_III: Decoration = Decoration {
    name: "Paralyzer Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&PARALYSIS_ATTACK, 3)],
};

const SLEEP_JEWEL: Decoration = Decoration {
    name: "Sleep Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SLEEP_ATTACK, 1)],
};

const SLEEP_JEWEL_II: Decoration = Decoration {
    name: "Sleep Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&SLEEP_ATTACK, 2)],
};

const SLEEP_JEWEL_III: Decoration = Decoration {
    name: "Sleep Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&SLEEP_ATTACK, 3)],
};

const BLAST_JEWEL: Decoration = Decoration {
    name: "Blast Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BLAST_ATTACK, 1)],
};

const BLAST_JEWEL_II: Decoration = Decoration {
    name: "Blast Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&BLAST_ATTACK, 2)],
};

const BLAST_JEWEL_III: Decoration = Decoration {
    name: "Blast Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&BLAST_ATTACK, 3)],
};

const CRIT_ELEMENT_JEWEL: Decoration = Decoration {
    name: "Crit Element Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CRITICAL_ELEMENT, 1)],
};

const CRIT_ELEMENT_JEWEL_II: Decoration = Decoration {
    name: "Crit Element Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CRITICAL_ELEMENT, 2)],
};

const CRIT_ELEM_JWL_III: Decoration = Decoration {
    name: "Crit Elem Jwl III",
    size: 3,
    skills: &[SkillAmount::new(&CRITICAL_ELEMENT, 3)],
};

const CRIT_ELEM_BLAZE_JWL: Decoration = Decoration {
    name: "Crit Elem/Blaze Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const CRIT_ELEM_STREAM_JWL: Decoration = Decoration {
    name: "Crit Elem/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const CRIT_ELEM_FROST_JWL: Decoration = Decoration {
    name: "Crit Elem/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const CRIT_ELEM_BOLT_JWL: Decoration = Decoration {
    name: "Crit Elem/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const CRIT_ELEM_DRAGON_JWL: Decoration = Decoration {
    name: "Crit Elem/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const CRIT_ELEM_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Crit Elem/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const CRIT_ELEM_PRECISE_JWL: Decoration = Decoration {
    name: "Crit Elem/Precise Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const CRIT_ELEM_KO_JEWEL: Decoration = Decoration {
    name: "Crit Elem/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const CRIT_ELEM_IRONWALL_JWL: Decoration = Decoration {
    name: "Crit Elem/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const CRIT_STATUS_JEWEL: Decoration = Decoration {
    name: "Crit Status Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CRITICAL_STATUS, 1)],
};

const CRIT_STATUS_JEWEL_II: Decoration = Decoration {
    name: "Crit Status Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CRITICAL_STATUS, 2)],
};

const CRIT_STAT_JWL_III: Decoration = Decoration {
    name: "Crit Stat Jwl III",
    size: 3,
    skills: &[SkillAmount::new(&CRITICAL_STATUS, 3)],
};

const CRIT_STAT_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Crit Stat/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_STATUS, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const CRIT_STAT_PRECISE_JWL: Decoration = Decoration {
    name: "Crit Stat/Precise Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_STATUS, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const CRIT_STATUS_KO_JWL: Decoration = Decoration {
    name: "Crit Status/KO Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_STATUS, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const CRIT_STAT_IRONWALL_JWL: Decoration = Decoration {
    name: "Crit Stat/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CRITICAL_STATUS, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const CHARGE_JEWEL: Decoration = Decoration {
    name: "Charge Jewel",
    size: 1,
    skills: &[SkillAmount::new(&CHARGE_MASTER, 1)],
};

const CHARGE_JEWEL_II: Decoration = Decoration {
    name: "Charge Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&CHARGE_MASTER, 2)],
};

const CHARGE_JEWEL_III: Decoration = Decoration {
    name: "Charge Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&CHARGE_MASTER, 3)],
};

const CHARGE_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Charge/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_MASTER, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const CHARGE_KO_JEWEL: Decoration = Decoration {
    name: "Charge/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_MASTER, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const HANDICRAFT_JEWEL: Decoration = Decoration {
    name: "Handicraft Jewel",
    size: 1,
    skills: &[SkillAmount::new(&HANDICRAFT, 1)],
};

const HANDICRAFT_JEWEL_II: Decoration = Decoration {
    name: "Handicraft Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&HANDICRAFT, 2)],
};

const HANDICRAFT_JWL_III: Decoration = Decoration {
    name: "Handicraft Jwl III",
    size: 3,
    skills: &[SkillAmount::new(&HANDICRAFT, 3)],
};

const RAZOR_SHARP_JEWEL: Decoration = Decoration {
    name: "Razor Sharp Jewel",
    size: 1,
    skills: &[SkillAmount::new(&RAZOR_SHARP, 1)],
};

const RAZOR_SHARP_JEWEL_II: Decoration = Decoration {
    name: "Razor Sharp Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&RAZOR_SHARP, 2)],
};

const RZR_SHARP_JWL_III: Decoration = Decoration {
    name: "Rzr Sharp Jwl III",
    size: 3,
    skills: &[SkillAmount::new(&RAZOR_SHARP, 3)],
};

const RZR_SHARP_BLAZE_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Blaze Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const RZR_SHARP_STREAM_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const RZR_SHARP_FROST_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const RZR_SHARP_BOLT_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const RZR_SHARP_DRAGON_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const RZR_SHARP_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const RAZOR_SHARP_KO_JWL: Decoration = Decoration {
    name: "Razor Sharp/KO Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const RZR_SHARP_IRONWALL_JWL: Decoration = Decoration {
    name: "Rzr Sharp/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAZOR_SHARP, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const SHARP_JEWEL: Decoration = Decoration {
    name: "Sharp Jewel",
    size: 1,
    skills: &[SkillAmount::new(&PROTECTIVE_POLISH, 1)],
};

const SHARP_JEWEL_II: Decoration = Decoration {
    name: "Sharp Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&PROTECTIVE_POLISH, 2)],
};

const SHARP_JEWEL_III: Decoration = Decoration {
    name: "Sharp Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&PROTECTIVE_POLISH, 3)],
};

const MINDS_EYE_JEWEL: Decoration = Decoration {
    name: "Mind's Eye Jewel",
    size: 1,
    skills: &[SkillAmount::new(&MINDS_EYE, 1)],
};

const MINDS_EYE_JEWEL_II: Decoration = Decoration {
    name: "Mind's Eye Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&MINDS_EYE, 2)],
};

const MINDS_EYE_JWL_III: Decoration = Decoration {
    name: "Mind's Eye Jwl III",
    size: 3,
    skills: &[SkillAmount::new(&MINDS_EYE, 3)],
};

const BLUNT_JEWEL: Decoration = Decoration {
    name: "Blunt Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BLUDGEONER, 1)],
};

const BLUNT_JEWEL_II: Decoration = Decoration {
    name: "Blunt Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&BLUDGEONER, 2)],
};

const BLUNT_JEWEL_III: Decoration = Decoration {
    name: "Blunt Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&BLUDGEONER, 3)],
};

const MASTERY_JEWEL: Decoration = Decoration {
    name: "Mastery Jewel",
    size: 3,
    skills: &[SkillAmount::new(&MASTERS_TOUCH, 1)],
};

const FORCESHOT_JEWEL: Decoration = Decoration {
    name: "Forceshot Jewel",
    size: 3,
    skills: &[SkillAmount::new(&NORMAL_SHOTS, 1)],
};

const PIERCE_JEWEL: Decoration = Decoration {
    name: "Pierce Jewel",
    size: 3,
    skills: &[SkillAmount::new(&PIERCING_SHOTS, 1)],
};

const SPREAD_JEWEL: Decoration = Decoration {
    name: "Spread Jewel",
    size: 3,
    skills: &[SkillAmount::new(&SPREAD_POWER_SHOTS, 1)],
};

const PRECISE_JEWEL: Decoration = Decoration {
    name: "Precise Jewel",
    size: 1,
    skills: &[SkillAmount::new(&BALLISTICS, 1)],
};

const PRECISE_JEWEL_II: Decoration = Decoration {
    name: "Precise Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&BALLISTICS, 2)],
};

const PRECISE_JEWEL_III: Decoration = Decoration {
    name: "Precise Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&BALLISTICS, 3)],
};

const SALVO_JEWEL: Decoration = Decoration {
    name: "Salvo Jewel",
    size: 3,
    skills: &[SkillAmount::new(&RAPID_FIRE_UP, 1)],
};

const OPENER_JEWEL: Decoration = Decoration {
    name: "Opener Jewel",
    size: 1,
    skills: &[SkillAmount::new(&OPENING_SHOT, 1)],
};

const OPENER_JEWEL_II: Decoration = Decoration {
    name: "Opener Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&OPENING_SHOT, 2)],
};

const OPENER_JEWEL_III: Decoration = Decoration {
    name: "Opener Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&OPENING_SHOT, 3)],
};

const OPENER_BLAZE_JEWEL: Decoration = Decoration {
    name: "Opener/Blaze Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const OPENER_STREAM_JWL: Decoration = Decoration {
    name: "Opener/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const OPENER_FROST_JEWEL: Decoration = Decoration {
    name: "Opener/Frost Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const OPENER_BOLT_JEWEL: Decoration = Decoration {
    name: "Opener/Bolt Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const OPENER_DRAGON_JEWEL: Decoration = Decoration {
    name: "Opener/Dragon Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const OPENER_PRECISE_JEWEL: Decoration = Decoration {
    name: "Opener/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const OPENER_IRONWALL_JWL: Decoration = Decoration {
    name: "Opener/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&OPENING_SHOT, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const BANDOLIER_JEWEL: Decoration = Decoration {
    name: "Bandolier Jewel",
    size: 1,
    skills: &[SkillAmount::new(&TETRAD_SHOT, 1)],
};

const BANDOLIER_JEWEL_II: Decoration = Decoration {
    name: "Bandolier Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&TETRAD_SHOT, 2)],
};

const BANDOLIER_JEWEL_III: Decoration = Decoration {
    name: "Bandolier Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&TETRAD_SHOT, 3)],
};

const BANDOLIER_BLAZE_JWL: Decoration = Decoration {
    name: "Bandolier/Blaze Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const BANDOLIER_STREAM_JWL: Decoration = Decoration {
    name: "Bandolier/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const BANDOLIER_FROST_JWL: Decoration = Decoration {
    name: "Bandolier/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const BANDOLIER_BOLT_JWL: Decoration = Decoration {
    name: "Bandolier/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const BANDOLIER_DRAGON_JWL: Decoration = Decoration {
    name: "Bandolier/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const BANDOLIER_PRECISE_JWL: Decoration = Decoration {
    name: "Bandolier/Precise Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const BANDOLIER_IRONWALL_JWL: Decoration = Decoration {
    name: "Bandolier/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&TETRAD_SHOT, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const TRUESHOT_JEWEL: Decoration = Decoration {
    name: "Trueshot Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SPECIAL_AMMO_BOOST, 1)],
};

const TRUESHOT_JEWEL_II: Decoration = Decoration {
    name: "Trueshot Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&SPECIAL_AMMO_BOOST, 2)],
};

const POISONCOAT_JEWEL: Decoration = Decoration {
    name: "Poisoncoat Jewel",
    size: 2,
    skills: &[SkillAmount::new(&POISON_FUNCTIONALITY, 1)],
};

const PARACOAT_JEWEL: Decoration = Decoration {
    name: "Paracoat Jewel",
    size: 3,
    skills: &[SkillAmount::new(&PARA_FUNCTIONALITY, 1)],
};

const SLEEPCOAT_JEWEL: Decoration = Decoration {
    name: "Sleepcoat Jewel",
    size: 3,
    skills: &[SkillAmount::new(&SLEEP_FUNCTIONALITY, 1)],
};

const BLASTCOAT_JEWEL: Decoration = Decoration {
    name: "Blastcoat Jewel",
    size: 2,
    skills: &[SkillAmount::new(&BLAST_FUNCTIONALITY, 1)],
};

const DRAINCOAT_JEWEL: Decoration = Decoration {
    name: "Draincoat Jewel",
    size: 2,
    skills: &[SkillAmount::new(&EXHAUST_FUNCTIONALITY, 1)],
};

const FOCUS_JEWEL: Decoration = Decoration {
    name: "Focus Jewel",
    size: 1,
    skills: &[SkillAmount::new(&FOCUS, 1)],
};

const FOCUS_JEWEL_II: Decoration = Decoration {
    name: "Focus Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&FOCUS, 2)],
};

const FOCUS_JEWEL_III: Decoration = Decoration {
    name: "Focus Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&FOCUS, 3)],
};

const FOCUS_BLAZE_JEWEL: Decoration = Decoration {
    name: "Focus/Blaze Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const FOCUS_STREAM_JEWEL: Decoration = Decoration {
    name: "Focus/Stream Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const FOCUS_FROST_JEWEL: Decoration = Decoration {
    name: "Focus/Frost Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const FOCUS_BOLT_JWL: Decoration = Decoration {
    name: "Focus/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const FOCUS_DRAGON_JEWEL: Decoration = Decoration {
    name: "Focus/Dragon Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const FOCUS_HANDICRAFT_JEWEL: Decoration = Decoration {
    name: "Focus/Handicraft Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const FOCUS_PRECISE_JEWEL: Decoration = Decoration {
    name: "Focus/Precise Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&FOCUS, 3),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const FOCUS_KO_JEWEL: Decoration = Decoration {
    name: "Focus/KO Jewel",
    size: 3,
    skills: &[SkillAmount::new(&FOCUS, 3), SkillAmount::new(&SLUGGER, 1)],
};

const ENHANCER_JEWEL: Decoration = Decoration {
    name: "Enhancer Jewel",
    size: 1,
    skills: &[SkillAmount::new(&POWER_PROLONGER, 1)],
};

const ENHANCER_JEWEL_II: Decoration = Decoration {
    name: "Enhancer Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&POWER_PROLONGER, 2)],
};

const ENHANCER_JEWEL_III: Decoration = Decoration {
    name: "Enhancer Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&POWER_PROLONGER, 3)],
};

const ENHANCER_BLAZE_JEWEL: Decoration = Decoration {
    name: "Enhancer/Blaze Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const ENHANCER_STREAM_JWL: Decoration = Decoration {
    name: "Enhancer/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const ENHANCER_FROST_JWL: Decoration = Decoration {
    name: "Enhancer/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const ENHANCER_BOLT_JEWEL: Decoration = Decoration {
    name: "Enhancer/Bolt Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const ENHANCER_DRAGON_JWL: Decoration = Decoration {
    name: "Enhancer/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const ENHANCER_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Enhancer/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POWER_PROLONGER, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const GAMBIT_JEWEL: Decoration = Decoration {
    name: "Gambit Jewel",
    size: 1,
    skills: &[SkillAmount::new(&PUNISHING_DRAW, 1)],
};

const GAMBIT_JEWEL_II: Decoration = Decoration {
    name: "Gambit Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&PUNISHING_DRAW, 2)],
};

const GAMBIT_JEWEL_III: Decoration = Decoration {
    name: "Gambit Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&PUNISHING_DRAW, 3)],
};

const GAMBIT_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Gambit/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&PUNISHING_DRAW, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const GAMBIT_KO_JEWEL: Decoration = Decoration {
    name: "Gambit/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&PUNISHING_DRAW, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const GAMBIT_IRONWALL_JWL: Decoration = Decoration {
    name: "Gambit/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&PUNISHING_DRAW, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const KO_JEWEL: Decoration = Decoration {
    name: "KO Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SLUGGER, 1)],
};

const KO_JEWEL_II: Decoration = Decoration {
    name: "KO Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&SLUGGER, 2)],
};

const KO_JEWEL_III: Decoration = Decoration {
    name: "KO Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&SLUGGER, 3)],
};

const DRAIN_JEWEL: Decoration = Decoration {
    name: "Drain Jewel",
    size: 1,
    skills: &[SkillAmount::new(&STAMINA_THIEF, 1)],
};

const DRAIN_JEWEL_II: Decoration = Decoration {
    name: "Drain Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&STAMINA_THIEF, 2)],
};

const DRAIN_JEWEL_III: Decoration = Decoration {
    name: "Drain Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&STAMINA_THIEF, 3)],
};

const ARTILLERY_JEWEL: Decoration = Decoration {
    name: "Artillery Jewel",
    size: 1,
    skills: &[SkillAmount::new(&ARTILLERY, 1)],
};

const ARTILLERY_JEWEL_II: Decoration = Decoration {
    name: "Artillery Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&ARTILLERY, 2)],
};

const ARTILLERY_JEWEL_III: Decoration = Decoration {
    name: "Artillery Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&ARTILLERY, 3)],
};

const QUICKSWITCH_JEWEL: Decoration = Decoration {
    name: "Quickswitch Jewel",
    size: 1,
    skills: &[SkillAmount::new(&RAPID_MORPH, 1)],
};

const QUICKSWITCH_JEWEL_II: Decoration = Decoration {
    name: "Quickswitch Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&RAPID_MORPH, 2)],
};

const QUICKSWITCH_JEWEL_III: Decoration = Decoration {
    name: "Quickswitch Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&RAPID_MORPH, 3)],
};

const QUICKSWITCH_BLAZE_JWL: Decoration = Decoration {
    name: "Quickswitch/Blaze Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const QUICKSWITCH_STREAM_JWL: Decoration = Decoration {
    name: "Quickswitch/Stream Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const QUICKSWITCH_FROST_JWL: Decoration = Decoration {
    name: "Quickswitch/Frost Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const QUICKSWITCH_BOLT_JWL: Decoration = Decoration {
    name: "Quickswitch/Bolt Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const QUICKSWITCH_DRAGON_JWL: Decoration = Decoration {
    name: "Quickswitch/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const QUICKSWITCH_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Quickswitch/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const QUICKSWITCH_KO_JWL: Decoration = Decoration {
    name: "Quickswitch/KO Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const QUICKSWITCH_IRONWALL_JWL: Decoration = Decoration {
    name: "Quickswitch/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&RAPID_MORPH, 3),
        SkillAmount::new(&GUARD, 1),
    ],
};

const SONOROUS_JEWEL: Decoration = Decoration {
    name: "Sonorous Jewel",
    size: 2,
    skills: &[SkillAmount::new(&HORN_MAESTRO, 1)],
};

const SONOROUS_ATTACK_JWL: Decoration = Decoration {
    name: "Sonorous/Attack Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&HORN_MAESTRO, 2),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const SONOROUS_EXPERT_JWL: Decoration = Decoration {
    name: "Sonorous/Expert Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&HORN_MAESTRO, 2),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const SONOROUS_DRAW_JWL: Decoration = Decoration {
    name: "Sonorous/Draw Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&HORN_MAESTRO, 2),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const SONOROUS_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Sonorous/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&HORN_MAESTRO, 2),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const SONOROUS_KO_JEWEL: Decoration = Decoration {
    name: "Sonorous/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&HORN_MAESTRO, 2),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const CHARGE_UP_JEWEL: Decoration = Decoration {
    name: "Charge Up Jewel",
    size: 2,
    skills: &[SkillAmount::new(&CHARGE_UP, 1)],
};

const CHARGE_UP_ATTACK_JWL: Decoration = Decoration {
    name: "Charge Up/Attack Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_UP, 1),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const CHARGE_UP_EXPERT_JWL: Decoration = Decoration {
    name: "Charge Up/Expert Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_UP, 1),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const CHARGE_UP_DRAW_JWL: Decoration = Decoration {
    name: "Charge Up/Draw Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_UP, 1),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const CHARGE_UP_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Charge Up/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_UP, 1),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const CHARGE_UP_KO_JEWEL: Decoration = Decoration {
    name: "Charge Up/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&CHARGE_UP, 1),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const FLIGHT_JEWEL: Decoration = Decoration {
    name: "Flight Jewel",
    size: 2,
    skills: &[SkillAmount::new(&AIRBORNE, 1)],
};

const FLIGHT_ATTACK_JWL: Decoration = Decoration {
    name: "Flight/Attack Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&AIRBORNE, 1),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const FLIGHT_EXPERT_JEWEL: Decoration = Decoration {
    name: "Flight/Expert Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&AIRBORNE, 1),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const FLIGHT_DRAW_JEWEL: Decoration = Decoration {
    name: "Flight/Draw Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&AIRBORNE, 1),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const FLIGHT_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Flight/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&AIRBORNE, 1),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const POISON_JEWEL: Decoration = Decoration {
    name: "Poison Jewel",
    size: 2,
    skills: &[SkillAmount::new(&POISON_DURATION_UP, 1)],
};

const POISON_ATTACK_JEWEL: Decoration = Decoration {
    name: "Poison/Attack Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const POISON_EXPERT_JWL: Decoration = Decoration {
    name: "Poison/Expert Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const POISON_DRAW_JEWEL: Decoration = Decoration {
    name: "Poison/Draw Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const POISON_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Poison/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const POISON_PRECISE_JWL: Decoration = Decoration {
    name: "Poison/Precise Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&BALLISTICS, 1),
    ],
};

const POISON_KO_JEWEL: Decoration = Decoration {
    name: "Poison/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const POISON_IRONWALL_JWL: Decoration = Decoration {
    name: "Poison/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&POISON_DURATION_UP, 1),
        SkillAmount::new(&GUARD, 1),
    ],
};

const IRONWALL_JEWEL: Decoration = Decoration {
    name: "Ironwall Jewel",
    size: 1,
    skills: &[SkillAmount::new(&GUARD, 1)],
};

const IRONWALL_JEWEL_II: Decoration = Decoration {
    name: "Ironwall Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&GUARD, 2)],
};

const IRONWALL_JEWEL_III: Decoration = Decoration {
    name: "Ironwall Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&GUARD, 3)],
};

const SHIELD_JEWEL: Decoration = Decoration {
    name: "Shield Jewel",
    size: 1,
    skills: &[SkillAmount::new(&GUARD_UP, 1)],
};

const SHIELD_JEWEL_II: Decoration = Decoration {
    name: "Shield Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&GUARD_UP, 2)],
};

const SHIELD_JEWEL_III: Decoration = Decoration {
    name: "Shield Jewel III",
    size: 3,
    skills: &[SkillAmount::new(&GUARD_UP, 3)],
};

const SHIELD_BLAZE_JEWEL: Decoration = Decoration {
    name: "Shield/Blaze Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&FIRE_ATTACK, 1),
    ],
};

const SHIELD_STREAM_JEWEL: Decoration = Decoration {
    name: "Shield/Stream Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&WATER_ATTACK, 1),
    ],
};

const SHIELD_FROST_JEWEL: Decoration = Decoration {
    name: "Shield/Frost Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&ICE_ATTACK, 1),
    ],
};

const SHIELD_BOLT_JEWEL: Decoration = Decoration {
    name: "Shield/Bolt Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&THUNDER_ATTACK, 1),
    ],
};

const SHIELD_DRAGON_JWL: Decoration = Decoration {
    name: "Shield/Dragon Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&DRAGON_ATTACK, 1),
    ],
};

const SHIELD_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Shield/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const SHIELD_IRONWALL_JWL: Decoration = Decoration {
    name: "Shield/Ironwall Jwl",
    size: 3,
    skills: &[SkillAmount::new(&GUARD_UP, 3), SkillAmount::new(&GUARD, 1)],
};

const MAGAZINE_JEWEL: Decoration = Decoration {
    name: "Magazine Jewel",
    size: 1,
    skills: &[SkillAmount::new(&LOAD_SHELLS, 1)],
};

const MAGAZINE_JEWEL_II: Decoration = Decoration {
    name: "Magazine Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&LOAD_SHELLS, 2)],
};

const MAGAZINE_ATTACK_JWL: Decoration = Decoration {
    name: "Magazine/Attack Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&LOAD_SHELLS, 2),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const MAGAZINE_EXPERT_JWL: Decoration = Decoration {
    name: "Magazine/Expert Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&LOAD_SHELLS, 2),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const MAGAZINE_DRAW_JWL: Decoration = Decoration {
    name: "Magazine/Draw Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&LOAD_SHELLS, 2),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const MAGAZINE_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Magazine/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&LOAD_SHELLS, 2),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const MAGAZINE_IRONWALL_JWL: Decoration = Decoration {
    name: "Magazine/Ironwall Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&LOAD_SHELLS, 2),
        SkillAmount::new(&GUARD, 1),
    ],
};

const GRINDER_JEWEL: Decoration = Decoration {
    name: "Grinder Jewel",
    size: 1,
    skills: &[SkillAmount::new(&SPEED_SHARPENING, 1)],
};

const GRINDER_JEWEL_II: Decoration = Decoration {
    name: "Grinder Jewel II",
    size: 2,
    skills: &[SkillAmount::new(&SPEED_SHARPENING, 2)],
};

const GRINDER_ATTACK_JEWEL: Decoration = Decoration {
    name: "Grinder/Attack Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
};

const GRINDER_EXPERT_JEWEL: Decoration = Decoration {
    name: "Grinder/Expert Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&CRITICAL_EYE, 1),
    ],
};

const GRINDER_DRAW_JEWEL: Decoration = Decoration {
    name: "Grinder/Draw Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&CRITICAL_DRAW, 1),
    ],
};

const GRINDER_HANDICRAFT_JWL: Decoration = Decoration {
    name: "Grinder/Handicraft Jwl",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&HANDICRAFT, 1),
    ],
};

const GRINDER_KO_JEWEL: Decoration = Decoration {
    name: "Grinder/KO Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&SLUGGER, 1),
    ],
};

const GRINDER_IRONWALL_JEWEL: Decoration = Decoration {
    name: "Grinder/Ironwall Jewel",
    size: 3,
    skills: &[
        SkillAmount::new(&SPEED_SHARPENING, 2),
        SkillAmount::new(&GUARD, 1),
    ],
};
