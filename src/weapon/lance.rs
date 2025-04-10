use crate::skill::SkillAmount;
use crate::skill::weapon_skill::{
    AIRBORNE, ATTACK_BOOST, CRITICAL_BOOST, CRITICAL_DRAW, CRITICAL_ELEMENT, CRITICAL_EYE,
    CRITICAL_STATUS, GUARD, GUARD_UP, MASTERS_TOUCH, OFFENSIVE_GUARD, PUNISHING_DRAW,
    SLICKED_BLADE, SPEED_SHARPENING, WHITEFLAME_TORRENT,
};
use crate::weapon::{Element, Sharpness, Weapon, WeaponType};

pub const LANCES: [&Weapon; 22] = [
    &ESPERANZA_LANCE,
    &SHINING_PILLAR,
    &SANDSEA_PRALLAYA,
    &INDRA_CLAIRLANCE,
    &CHROME_LANCE,
    &ROMPO_TETROTOX,
    &ABADDONIAN_OSMINOG,
    &HARD_BONE_LANCE,
    &WINDPIERCE_TSUKIBAMI,
    &BEQUEATHED_REGRET,
    &FIRETRAIL_KAMINET,
    &FIERCEBORER_DUBHANITH,
    &G_PURIFYING_BEAUMAINS,
    &PURIFYING_BEAUMAINS,
    &GLOOMBORER_URSHANITH,
    &LALA_AVICULARI,
    &SPEAR_OF_PROMINENCE,
    &FIEBERSCHILD,
    &GRAVIOS_LANCE,
    &BABEL_SPEAR,
    &BLAZING_SITAL,
    &KIMI_GA_KIRU,
];

const ESPERANZA_LANCE: Weapon = Weapon {
    name: "Esperanza Lance",
    tree: "Expedition",
    attack: 210,
    affinity: 0,
    element: Element::None,
    sharpness: Sharpness {
        red: 50,
        orange: 60,
        yellow: 60,
        green: 60,
        blue: 40,
        white: 130,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&CRITICAL_EYE, 2),
        SkillAmount::new(&SPEED_SHARPENING, 2),
    ],
    weapon_type: WeaponType::Lance,
};

const SHINING_PILLAR: Weapon = Weapon {
    name: "Shining Pillar",
    tree: "G. Rathalos",
    attack: 220,
    affinity: 15,
    element: Element::Fire(25),
    sharpness: Sharpness {
        red: 50,
        orange: 150,
        yellow: 50,
        green: 40,
        blue: 40,
        white: 70,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
        SkillAmount::new(&MASTERS_TOUCH, 1),
    ],
    weapon_type: WeaponType::Lance,
};

const SANDSEA_PRALLAYA: Weapon = Weapon {
    name: "Sandsea Prallaya",
    tree: "Balahara",
    attack: 210,
    affinity: 10,
    element: Element::Water(25),
    sharpness: Sharpness {
        red: 40,
        orange: 30,
        yellow: 100,
        green: 80,
        blue: 70,
        white: 80,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const INDRA_CLAIRLANCE: Weapon = Weapon {
    name: "Indra Clairlance",
    tree: "Rey Dau",
    attack: 220,
    affinity: 0,
    element: Element::Thunder(25),
    // For the Indra Clairlance in particular, I have gone into the training area and hit the dummy
    // 350 times (without Handicraft or any other sharpness skills) to ensure sharpness drops after
    // the correct number of hits, which it did. Thus these sharpness numbers are the only ones I've
    // explicitly confirmed myself. Sharpness numbers for all other Lances come from counting the
    // pixels on sharpness bars.
    sharpness: Sharpness {
        red: 10,
        orange: 10,
        yellow: 110,
        green: 70,
        blue: 70,
        white: 130,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&GUARD, 2),
        SkillAmount::new(&OFFENSIVE_GUARD, 1),
    ],
    weapon_type: WeaponType::Lance,
};

const CHROME_LANCE: Weapon = Weapon {
    name: "Chrome Lance",
    tree: "Ore",
    attack: 220,
    affinity: 0,
    element: Element::None,
    sharpness: Sharpness {
        red: 30,
        orange: 60,
        yellow: 90,
        green: 70,
        blue: 50,
        white: 100,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD_UP, 3)],
    weapon_type: WeaponType::Lance,
};

const ROMPO_TETROTOX: Weapon = Weapon {
    name: "Rompo Tetrotox",
    tree: "Rompopolo",
    attack: 220,
    affinity: 0,
    element: Element::Poison(25),
    sharpness: Sharpness {
        red: 40,
        orange: 50,
        yellow: 100,
        green: 30,
        blue: 100,
        white: 80,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD_UP, 3)],
    weapon_type: WeaponType::Lance,
};

const ABADDONIAN_OSMINOG: Weapon = Weapon {
    name: "Abaddonian Osminog",
    tree: "Nu Udra",
    attack: 230,
    affinity: -15,
    element: Element::Fire(25),
    sharpness: Sharpness {
        red: 100,
        orange: 20,
        yellow: 100,
        green: 50,
        blue: 50,
        white: 80,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const HARD_BONE_LANCE: Weapon = Weapon {
    name: "Hard Bone Lance",
    tree: "Bone",
    attack: 230,
    affinity: 0,
    element: Element::None,
    sharpness: Sharpness {
        red: 50,
        orange: 60,
        yellow: 70,
        green: 70,
        blue: 150,
        white: 0,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&GUARD_UP, 3),
        SkillAmount::new(&ATTACK_BOOST, 1),
    ],
    weapon_type: WeaponType::Lance,
};

const WINDPIERCE_TSUKIBAMI: Weapon = Weapon {
    name: "Windpierce Tsukibami",
    tree: "Hirabami",
    attack: 220,
    affinity: 0,
    element: Element::Ice(25),
    sharpness: Sharpness {
        red: 50,
        orange: 50,
        yellow: 50,
        green: 30,
        blue: 120,
        white: 100,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD, 3), SkillAmount::new(&AIRBORNE, 1)],
    weapon_type: WeaponType::Lance,
};

const BEQUEATHED_REGRET: Weapon = Weapon {
    name: "Bequeathed Regret",
    tree: "G. Ebony",
    attack: 210,
    affinity: 10,
    element: Element::Dragon(35),
    sharpness: Sharpness {
        red: 50,
        orange: 50,
        yellow: 80,
        green: 80,
        blue: 100,
        white: 40,
        handicraft_locked: 0,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const FIRETRAIL_KAMINET: Weapon = Weapon {
    name: "Firetrail Kaminet",
    tree: "Quematrice",
    attack: 240,
    affinity: 5,
    element: Element::Fire(20),
    sharpness: Sharpness {
        red: 60,
        orange: 70,
        yellow: 80,
        green: 80,
        blue: 110,
        white: 0,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const FIERCEBORER_DUBHANITH: Weapon = Weapon {
    name: "Fierceborer Dubhanith",
    tree: "Doshaguma",
    attack: 240,
    affinity: -5,
    element: Element::None,
    sharpness: Sharpness {
        red: 50,
        orange: 100,
        yellow: 110,
        green: 50,
        blue: 40,
        white: 50,
        handicraft_locked: 50,
    },
    three_slots: 2,
    two_slots: 0,
    one_slots: 1,
    defense: 20,
    skills: &[SkillAmount::new(&PUNISHING_DRAW, 3)],
    weapon_type: WeaponType::Lance,
};

const G_PURIFYING_BEAUMAINS: Weapon = Weapon {
    name: "G. Purifying Beaumains",
    tree: "G. Arkveld",
    attack: 240,
    affinity: -10,
    element: Element::Dragon(20),
    sharpness: Sharpness {
        red: 100,
        orange: 70,
        yellow: 50,
        green: 80,
        blue: 60,
        white: 40,
        handicraft_locked: 50,
    },
    three_slots: 2,
    two_slots: 0,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD, 2)],
    weapon_type: WeaponType::Lance,
};

const PURIFYING_BEAUMAINS: Weapon = Weapon {
    name: "Purifying Beaumains",
    tree: "Arkveld",
    attack: 220,
    affinity: 0,
    element: Element::Dragon(35),
    sharpness: Sharpness {
        red: 50,
        orange: 50,
        yellow: 50,
        green: 70,
        blue: 80,
        white: 100,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const GLOOMBORER_URSHANITH: Weapon = Weapon {
    name: "Gloomborer Urshanith",
    tree: "G. Doshaguma",
    attack: 250,
    affinity: -15,
    element: Element::None,
    sharpness: Sharpness {
        red: 50,
        orange: 70,
        yellow: 90,
        green: 90,
        blue: 70,
        white: 30,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&OFFENSIVE_GUARD, 3)],
    weapon_type: WeaponType::Lance,
};

const LALA_AVICULARI: Weapon = Weapon {
    name: "Lala Aviculari",
    tree: "Lala Barina",
    attack: 200,
    affinity: 15,
    element: Element::Paralysis(25),
    sharpness: Sharpness {
        red: 70,
        orange: 50,
        yellow: 30,
        green: 60,
        blue: 40,
        white: 150,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&CRITICAL_DRAW, 2),
        SkillAmount::new(&CRITICAL_STATUS, 1),
    ],
    weapon_type: WeaponType::Lance,
};

const SPEAR_OF_PROMINENCE: Weapon = Weapon {
    name: "Spear of Prominence",
    tree: "Rathalos",
    attack: 210,
    affinity: 10,
    element: Element::Fire(35),
    sharpness: Sharpness {
        red: 100,
        orange: 70,
        yellow: 50,
        green: 40,
        blue: 50,
        white: 90,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&CRITICAL_BOOST, 3)],
    weapon_type: WeaponType::Lance,
};

const FIEBERSCHILD: Weapon = Weapon {
    name: "Fieberschild",
    tree: "Gore Magala",
    attack: 210,
    affinity: 20,
    element: Element::Dragon(25),
    sharpness: Sharpness {
        red: 50,
        orange: 50,
        yellow: 80,
        green: 60,
        blue: 40,
        white: 120,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&CRITICAL_ELEMENT, 3)],
    weapon_type: WeaponType::Lance,
};

const GRAVIOS_LANCE: Weapon = Weapon {
    name: "Gravios Lance",
    tree: "Gravios",
    attack: 250,
    affinity: -15,
    element: Element::Blast(35),
    sharpness: Sharpness {
        red: 90,
        orange: 70,
        yellow: 70,
        green: 70,
        blue: 100,
        white: 0,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 30,
    skills: &[SkillAmount::new(&GUARD_UP, 3)],
    weapon_type: WeaponType::Lance,
};

const BABEL_SPEAR: Weapon = Weapon {
    name: "Babel Spear",
    tree: "Workshop",
    attack: 220,
    affinity: 0,
    element: Element::None,
    sharpness: Sharpness {
        red: 20,
        orange: 60,
        yellow: 60,
        green: 50,
        blue: 130,
        white: 80,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&CRITICAL_DRAW, 3),
        SkillAmount::new(&GUARD, 1),
    ],
    weapon_type: WeaponType::Lance,
};

const BLAZING_SITAL: Weapon = Weapon {
    name: "Blazing Sital",
    tree: "Zoh Shia",
    attack: 220,
    affinity: 5,
    element: Element::Dragon(20),
    sharpness: Sharpness {
        red: 100,
        orange: 40,
        yellow: 90,
        green: 40,
        blue: 30,
        white: 100,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[SkillAmount::new(&WHITEFLAME_TORRENT, 1)],
    weapon_type: WeaponType::Lance,
};

const KIMI_GA_KIRU: Weapon = Weapon {
    name: "Kimi Ga Kiru",
    tree: "Mizutsune",
    attack: 210,
    affinity: 15,
    element: Element::Water(30),
    sharpness: Sharpness {
        red: 80,
        orange: 60,
        yellow: 40,
        green: 40,
        blue: 70,
        white: 110,
        handicraft_locked: 50,
    },
    three_slots: 1,
    two_slots: 1,
    one_slots: 1,
    defense: 0,
    skills: &[
        SkillAmount::new(&SLICKED_BLADE, 3),
        SkillAmount::new(&CRITICAL_ELEMENT, 1),
    ],
    weapon_type: WeaponType::Lance,
};
