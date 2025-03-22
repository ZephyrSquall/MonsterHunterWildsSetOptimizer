use crate::decoration::Decoration;
use crate::skill::SkillAmount;
use crate::skill::weapon_skill::{
    ATTACK_BOOST, BALLISTICS, BLAST_ATTACK, BLUDGEONER, CHARGE_MASTER, CRITICAL_BOOST,
    CRITICAL_DRAW, CRITICAL_ELEMENT, CRITICAL_EYE, CRITICAL_STATUS, DRAGON_ATTACK, FIRE_ATTACK,
    FOCUS, GUARD, GUARD_UP, HANDICRAFT, ICE_ATTACK, MASTERS_TOUCH, MINDS_EYE, NORMAL_SHOTS,
    OFFENSIVE_GUARD, OPENING_SHOT, PARALYSIS_ATTACK, PIERCING_SHOTS, POISON_ATTACK,
    POWER_PROLONGER, PROTECTIVE_POLISH, RAPID_FIRE_UP, RAPID_MORPH, RAZOR_SHARP, SLEEP_ATTACK,
    SLUGGER, SPREAD_POWER_SHOTS, TETRAD_SHOT, THUNDER_ATTACK, WATER_ATTACK,
};

const ATTACK_JEWEL_I: Decoration = Decoration {
    name: "Attack Jewel I",
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

const GUARDIAN_JEWEL_I: Decoration = Decoration {
    name: "Guardian Jewel I",
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

const EXPERT_JEWEL_I: Decoration = Decoration {
    name: "Expert Jewel I",
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

const CRITICAL_JEWEL_I: Decoration = Decoration {
    name: "Critical Jewel I",
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

const DRAW_JEWEL_I: Decoration = Decoration {
    name: "Draw Jewel I",
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

const BLAZE_JEWEL_I: Decoration = Decoration {
    name: "Blaze Jewel I",
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

const STREAM_JEWEL_I: Decoration = Decoration {
    name: "Stream Jewel I",
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

const BOLT_JEWEL_I: Decoration = Decoration {
    name: "Bolt Jewel I",
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

const FROST_JEWEL_I: Decoration = Decoration {
    name: "Frost Jewel I",
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

const DRAGON_JEWEL_I: Decoration = Decoration {
    name: "Dragon Jewel I",
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

const VENOM_JEWEL_I: Decoration = Decoration {
    name: "Venom Jewel I",
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

const PARALYZER_JEWEL_I: Decoration = Decoration {
    name: "Paralyzer Jewel I",
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

const SLEEP_JEWEL_I: Decoration = Decoration {
    name: "Sleep Jewel I",
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

const BLAST_JEWEL_I: Decoration = Decoration {
    name: "Blast Jewel I",
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

const CRIT_ELEMENT_JEWEL_I: Decoration = Decoration {
    name: "Crit Element Jewel I",
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

const CRIT_STATUS_JEWEL_I: Decoration = Decoration {
    name: "Crit Status Jewel I",
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

const CHARGE_JEWEL_I: Decoration = Decoration {
    name: "Charge Jewel I",
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

const HANDICRAFT_JEWEL_I: Decoration = Decoration {
    name: "Handicraft Jewel I",
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

const RAZOR_SHARP_JEWEL_I: Decoration = Decoration {
    name: "Razor Sharp Jewel I",
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

const SHARP_JEWEL_I: Decoration = Decoration {
    name: "Sharp Jewel I",
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

const MINDS_EYE_JEWEL_I: Decoration = Decoration {
    name: "Mind's Eye Jewel I",
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

const BLUNT_JEWEL_I: Decoration = Decoration {
    name: "Blunt Jewel I",
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

const PRECISE_JEWEL_I: Decoration = Decoration {
    name: "Precise Jewel I",
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
