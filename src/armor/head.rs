use crate::armor::Armor;
use crate::skill::SkillAmount;
use crate::skill::armor_skill::{
    COALESCENCE, CONVERT_ELEMENT, ELEMENTAL_ABSORPTION, EVADE_WINDOW, FLAYER, PARTBREAKER,
};
use crate::skill::group_skill::{
    ALLURING_PELT, FORTIFYING_PELT, GUARDIANS_PROTECTION, GUARDIANS_PULSE, SCALE_LAYERING,
    SCALING_PROWESS,
};
use crate::skill::set_bonus_skill::{
    ARKVELDS_HUNGER, GORE_MAGALAS_TYRANNY, GUARDIAN_ARKVELDS_VITALITY,
};

pub const HEAD_ARMORS: [&Armor; 6] = [
    &GORE_HELM_ALPHA,
    &GORE_HELM_BETA,
    &ARKVULCAN_HELM_ALPHA,
    &ARKVULCAN_HELM_BETA,
    &G_ARKVELD_HELM_ALPHA,
    &G_ARKVELD_HELM_BETA,
];

const GORE_HELM_ALPHA: Armor = Armor {
    name: "Gore Helm α",
    set: "Gore α",
    defense: 60,
    three_slots: 0,
    two_slots: 1,
    one_slots: 0,
    fire_res: -2,
    water_res: 3,
    thunder_res: -1,
    ice_res: 2,
    dragon_res: -1,
    skills: &[
        SkillAmount::new(&EVADE_WINDOW, 2),
        SkillAmount::new(&COALESCENCE, 1),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALING_PROWESS, 1),
    ],
};

const GORE_HELM_BETA: Armor = Armor {
    name: "Gore Helm β",
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
        SkillAmount::new(&EVADE_WINDOW, 1),
        SkillAmount::new(&COALESCENCE, 1),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALE_LAYERING, 1),
    ],
};

const ARKVULCAN_HELM_ALPHA: Armor = Armor {
    name: "Arkvulcan Helm α",
    set: "Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 0,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -3,
    skills: &[
        SkillAmount::new(&ELEMENTAL_ABSORPTION, 2),
        SkillAmount::new(&CONVERT_ELEMENT, 1),
        SkillAmount::new(&ARKVELDS_HUNGER, 1),
        SkillAmount::new(&FORTIFYING_PELT, 1),
    ],
};

const ARKVULCAN_HELM_BETA: Armor = Armor {
    name: "Arkvulcan Helm β",
    set: "Arkveld β",
    defense: 66,
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -3,
    skills: &[
        SkillAmount::new(&CONVERT_ELEMENT, 1),
        SkillAmount::new(&ARKVELDS_HUNGER, 1),
        SkillAmount::new(&ALLURING_PELT, 1),
    ],
};

const G_ARKVELD_HELM_ALPHA: Armor = Armor {
    name: "G. Arkveld Helm α",
    set: "Guardian Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 0,
    one_slots: 2,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&PARTBREAKER, 2),
        SkillAmount::new(&FLAYER, 1),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PULSE, 1),
    ],
};

const G_ARKVELD_HELM_BETA: Armor = Armor {
    name: "G. Arkveld Helm β",
    set: "Guardian Arkveld β",
    defense: 66,
    three_slots: 1,
    two_slots: 0,
    one_slots: 1,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&FLAYER, 1),
        SkillAmount::new(&PARTBREAKER, 1),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PROTECTION, 1),
    ],
};
