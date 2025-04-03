use crate::armor::Armor;
use crate::skill::SkillAmount;
use crate::skill::armor_skill::{
    BLIGHT_RESISTANCE, CONSTITUTION, CONVERT_ELEMENT, EVADE_WINDOW, RECOVERY_SPEED,
    WEAKNESS_EXPLOIT,
};
use crate::skill::group_skill::{
    ALLURING_PELT, FORTIFYING_PELT, GUARDIANS_PROTECTION, GUARDIANS_PULSE, SCALE_LAYERING,
    SCALING_PROWESS,
};
use crate::skill::set_bonus_skill::{
    ARKVELDS_HUNGER, GORE_MAGALAS_TYRANNY, GUARDIAN_ARKVELDS_VITALITY,
};

pub const ARMS_ARMORS: [&Armor; 6] = [
    &GORE_VAMBRACES_ALPHA,
    &GORE_VAMBRACES_BETA,
    &ARKVULCAN_VAMBRACES_ALPHA,
    &ARKVULCAN_VAMBRACES_BETA,
    &G_ARKVELD_VAMBRACES_ALPHA,
    &G_ARKVELD_VAMBRACES_BETA,
];

const GORE_VAMBRACES_ALPHA: Armor = Armor {
    name: "Gore Vambraces α",
    set: "Gore α",
    defense: 60,
    three_slots: 0,
    two_slots: 1,
    one_slots: 1,
    fire_res: -2,
    water_res: 3,
    thunder_res: -1,
    ice_res: 2,
    dragon_res: -1,
    skills: &[
        SkillAmount::new(&CONSTITUTION, 2),
        SkillAmount::new(&EVADE_WINDOW, 1),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALING_PROWESS, 1),
    ],
};

const GORE_VAMBRACES_BETA: Armor = Armor {
    name: "Gore Vambraces β",
    set: "Gore β",
    defense: 60,
    three_slots: 0,
    two_slots: 2,
    one_slots: 0,
    fire_res: -2,
    water_res: 3,
    thunder_res: -1,
    ice_res: 2,
    dragon_res: -1,
    skills: &[
        SkillAmount::new(&CONSTITUTION, 1),
        SkillAmount::new(&EVADE_WINDOW, 1),
        SkillAmount::new(&GORE_MAGALAS_TYRANNY, 1),
        SkillAmount::new(&SCALE_LAYERING, 1),
    ],
};

const ARKVULCAN_VAMBRACES_ALPHA: Armor = Armor {
    name: "Arkvulcan Vambraces α",
    set: "Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 2,
    one_slots: 0,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -3,
    skills: &[
        SkillAmount::new(&CONVERT_ELEMENT, 1),
        SkillAmount::new(&RECOVERY_SPEED, 1),
        SkillAmount::new(&ARKVELDS_HUNGER, 1),
        SkillAmount::new(&FORTIFYING_PELT, 1),
    ],
};

const ARKVULCAN_VAMBRACES_BETA: Armor = Armor {
    name: "Arkvulcan Vambraces β",
    set: "Arkveld β",
    defense: 66,
    three_slots: 0,
    two_slots: 2,
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

const G_ARKVELD_VAMBRACES_ALPHA: Armor = Armor {
    name: "G. Arkveld Vambraces α",
    set: "Guardian Arkveld α",
    defense: 66,
    three_slots: 0,
    two_slots: 0,
    one_slots: 1,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&WEAKNESS_EXPLOIT, 2),
        SkillAmount::new(&BLIGHT_RESISTANCE, 1),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PULSE, 1),
    ],
};

const G_ARKVELD_VAMBRACES_BETA: Armor = Armor {
    name: "G. Arkveld Vambraces β",
    set: "Guardian Arkveld β",
    defense: 66,
    three_slots: 0,
    two_slots: 0,
    one_slots: 3,
    fire_res: 2,
    water_res: 0,
    thunder_res: -1,
    ice_res: 0,
    dragon_res: -4,
    skills: &[
        SkillAmount::new(&WEAKNESS_EXPLOIT, 2),
        SkillAmount::new(&GUARDIAN_ARKVELDS_VITALITY, 1),
        SkillAmount::new(&GUARDIANS_PROTECTION, 1),
    ],
};
