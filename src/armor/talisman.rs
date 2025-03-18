use crate::armor::Talisman;
use crate::skill::SkillId;

pub const TALISMANS: [Talisman; 3] = [
    Talisman {
        name: "Exploiter Charm II",
        skills: &[(SkillId::WeaknessExploit, 2)],
    },
    Talisman {
        name: "Challenger Charm II",
        skills: &[(SkillId::Agitator, 2)],
    },
    Talisman {
        name: "Chain Charm II",
        skills: &[(SkillId::Burst, 2)],
    },
];
