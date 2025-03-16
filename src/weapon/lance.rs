use crate::skill::SkillId;
use crate::weapon::{Element, Sharpness, Weapon};

pub const LANCES: [Weapon; 3] = [
    Weapon {
        name: "Esperanza Lance",
        tree: "Expedition",
        attack: 210,
        affinity: 0,
        element: Element::None,
        // TODO: Determine correct sharpness values.
        sharpness: Sharpness {
            orange: 1,
            yellow: 1,
            green: 1,
            blue: 1,
            white: 1,
            handicraft_locked: 50,
        },
        skills: &[(SkillId::CriticalEye, 2), (SkillId::SpeedSharpening, 2)],
        slots: &[3, 2, 1],
        defense: 0,
    },
    Weapon {
        name: "Shining Pillar",
        tree: "G. Rathalos",
        attack: 220,
        affinity: 15,
        element: Element::Fire(250),
        // TODO: Determine correct sharpness values.
        sharpness: Sharpness {
            orange: 1,
            yellow: 1,
            green: 1,
            blue: 1,
            white: 1,
            handicraft_locked: 50,
        },
        skills: &[(SkillId::CriticalElement, 1), (SkillId::MastersTouch, 1)],
        slots: &[3, 2, 1],
        defense: 0,
    },
    Weapon {
        name: "Sandsea Prallaya",
        tree: "Balahara",
        attack: 210,
        affinity: 10,
        element: Element::Water(250),
        // TODO: Determine correct sharpness values.
        sharpness: Sharpness {
            orange: 1,
            yellow: 1,
            green: 1,
            blue: 1,
            white: 1,
            handicraft_locked: 50,
        },
        skills: &[(SkillId::OffensiveGuard, 3)],
        slots: &[3, 2, 1],
        defense: 0,
    },
];
