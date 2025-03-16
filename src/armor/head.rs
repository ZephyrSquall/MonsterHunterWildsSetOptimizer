use crate::armor::Armor;
use crate::skill::SkillId;

pub const HEAD_ARMORS: [Armor; 2] = [
    Armor {
        name: "Gore Helm α",
        set: "Gore α",
        defense: 60,
        slots: &[2],
        fire_res: -2,
        water_res: 3,
        thunder_res: -1,
        ice_res: 2,
        dragon_res: -1,
        skills: &[
            (SkillId::EvadeWindow, 2),
            (SkillId::Coalescence, 1),
            (SkillId::GoreMagalasTyranny, 1),
            (SkillId::ScalingProwess, 1),
        ],
    },
    Armor {
        name: "Gore Helm β",
        set: "Gore β",
        defense: 60,
        slots: &[3, 1],
        fire_res: -2,
        water_res: 3,
        thunder_res: -1,
        ice_res: 2,
        dragon_res: -1,
        skills: &[
            (SkillId::EvadeWindow, 1),
            (SkillId::Coalescence, 1),
            (SkillId::GoreMagalasTyranny, 1),
            (SkillId::ScaleLayering, 1),
        ],
    },
];
