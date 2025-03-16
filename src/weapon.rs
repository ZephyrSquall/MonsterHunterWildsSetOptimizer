use crate::skill::SkillId;

pub mod lance;

pub struct Weapon {
    name: &'static str,
    tree: &'static str,
    pub attack: u16,
    pub affinity: i16,
    element: Element,
    sharpness: Sharpness,
    pub skills: &'static [(SkillId, u8)],
    slots: &'static [u8],
    defense: u16,
}

// The weapon's element or status. If not None, the enum also holds the base elemental/status attack
// value is included. Note that the game bloats elemental values even if the option to turn off
// bloat values is enabled; the values used here are the unbloated values which is 10 times less
// than what is shown in-game.
pub enum Element {
    None,
    Fire(u16),
    Water(u16),
    Thunder(u16),
    Ice(u16),
    Dragon(u16),
    Poison(u16),
    Paralysis(u16),
    Sleep(u16),
    Blast(u16),
}

// Sharpness is tracked by how many hits of sharpness it takes at a given sharpness level for the
// weapon to drop to the previous sharpness level. Note that some weapon attacks consume less than 1
// hit of sharpness, so the exact amount may differ in practice. Red sharpness is not tracked as it
// is effectively infinite for all weapons. As the weapon stats screen does not display exact
// numbers for sharpness, I obtain these numbers by assuming that the sharpness bar only increases
// in increments of 10 hits (to my knowledge this has been true in every Monster Hunter game to
// date), and carefully measuring the length of each segment. This is an error-prone process, so it
// would be great if someone could double-check my numbers or point me to a reliable resource that
// displays exact sharpness numbers.
struct Sharpness {
    orange: u16,
    yellow: u16,
    green: u16,
    blue: u16,
    white: u16,
    // How many hits of sharpness are locked behind the skill Handicraft. If the player has no
    // Handicraft skills, this many hits need to be removed from the above values to get the actual
    // sharpness amount. For each point of Handicraft the player has, remove 10 fewer hits. I
    // believe all weapons either have 50 (this weapon will receive the full benefit from every
    // level of Handicraft) or 0 (this weapon's sharpness bar is already full so Handicraft will
    // have no effect), though I am not yet certain of this.
    handicraft_locked: u16,
}
