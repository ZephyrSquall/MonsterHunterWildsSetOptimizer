use crate::armor::Talisman;
use crate::skill::SkillAmount;
use crate::skill::armor_skill::{AGITATOR, BURST, WEAKNESS_EXPLOIT};

pub const TALISMANS: [&Talisman; 3] = [&EXPLOITER_CHARM_II, &CHALLENGER_CHARM_II, &CHAIN_CHARM_II];

const EXPLOITER_CHARM_II: Talisman = Talisman {
    name: "Exploiter Charm II",
    skills: &[SkillAmount::new(&WEAKNESS_EXPLOIT, 2)],
};

const CHALLENGER_CHARM_II: Talisman = Talisman {
    name: "Challenger Charm II",
    skills: &[SkillAmount::new(&AGITATOR, 2)],
};

const CHAIN_CHARM_II: Talisman = Talisman {
    name: "Chain Charm II",
    skills: &[SkillAmount::new(&BURST, 2)],
};
