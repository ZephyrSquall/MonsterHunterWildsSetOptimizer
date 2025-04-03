use crate::armor::Armor;
use crate::skill::SkillAmount;
use crate::skill::armor_skill::{
    ANTIVIRUS, BLIGHT_RESISTANCE, ELEMENTAL_ABSORPTION, EVADE_WINDOW, FLAYER, WEAKNESS_EXPLOIT,
};
use crate::skill::group_skill::{
    ALLURING_PELT, FORTIFYING_PELT, GUARDIANS_PROTECTION, GUARDIANS_PULSE, SCALE_LAYERING,
    SCALING_PROWESS,
};
use crate::skill::set_bonus_skill::{
    ARKVELDS_HUNGER, GORE_MAGALAS_TYRANNY, GUARDIAN_ARKVELDS_VITALITY,
};

pub const CHEST_ARMORS: [&Armor; 6] = [
    &GORE_MAIL_ALPHA,
    &GORE_MAIL_BETA,
    &ARKVULCAN_MAIL_ALPHA,
    &ARKVULCAN_MAIL_BETA,
    &G_ARKVELD_MAIL_ALPHA,
    &G_ARKVELD_MAIL_BETA,
];

const GORE_MAIL_ALPHA: Armor = Armor {
    name: "Gore Mail α",
    set: "Gore α",
    defense: 60,
    three_slots: 1,
    two_slots: 0,
    one_slots: 0,
    fire_res: -2,
    water_res: 3,
    thunder_res: -1,
    ice_res: 2,
    dragon_res: -1,
    skills: &[
        SkillAmount::new(&EVADE_WINDOW, 2),
        SkillAmount::new(&ANTIVIRUS, 1),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALING_PROWESS, 1),
    ],
};

const GORE_MAIL_BETA: Armor = Armor {
    name: "Gore Mail β",
    set: "Gore β",
    defense: 60,
    three_slots: 1,
    two_slots: 0,
    one_slots: 1,
    fire_res: -2,
    water_res: 3,
    thunder_res: -1,
    ice_res: 2,
    dragon_res: -1,
    skills: &[
        SkillAmount::new(&EVADE_WINDOW, 2),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALE_LAYERING, 1),
    ],
};

const ARKVULCAN_MAIL_ALPHA: Armor = Armor {
    name: "Arkvulcan Mail α",
    set: "Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 1,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -3,
    skills: &[
        SkillAmount::new(&WEAKNESS_EXPLOIT, 1),
        SkillAmount::new(&ELEMENTAL_ABSORPTION, 1),
        SkillAmount::new(&ARKVELDS_HUNGER, 1),
        SkillAmount::new(&FORTIFYING_PELT, 1),
    ],
};

const ARKVULCAN_MAIL_BETA: Armor = Armor {
    name: "Arkvulcan Mail β",
    set: "Arkveld β",
    defense: 66,
    three_slots: 1,
    two_slots: 1,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -3,
    skills: &[
        SkillAmount::new(&WEAKNESS_EXPLOIT, 1),
        SkillAmount::new(&ARKVELDS_HUNGER, 1),
        SkillAmount::new(&ALLURING_PELT, 1),
    ],
};

const G_ARKVELD_MAIL_ALPHA: Armor = Armor {
    name: "G. Arkveld Mail α",
    set: "Guardian Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 0,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&FLAYER, 2),
        SkillAmount::new(&BLIGHT_RESISTANCE, 2),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PULSE, 1),
    ],
};

const G_ARKVELD_MAIL_BETA: Armor = Armor {
    name: "G. Arkveld Mail β",
    set: "Guardian Arkveld β",
    defense: 66,
    three_slots: 1,
    two_slots: 0,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&BLIGHT_RESISTANCE, 2),
        SkillAmount::new(&FLAYER, 1),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PROTECTION, 1),
    ],
};
