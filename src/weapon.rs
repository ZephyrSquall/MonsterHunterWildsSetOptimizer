use crate::config::HITS_BEFORE_SHARPENING;
use crate::skill::SkillAmount;

pub mod lance;

pub struct Weapon {
    pub name: &'static str,
    tree: &'static str,
    pub attack: u16,
    pub affinity: i16,
    pub element: Element,
    pub sharpness: Sharpness,
    pub three_slots: u8,
    pub two_slots: u8,
    pub one_slots: u8,
    defense: u16,
    pub skills: &'static [SkillAmount],
    pub weapon_type: WeaponType,
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
// hit of sharpness, so the exact amount may differ in practice. As the weapon stats screen does not
// display exact numbers for sharpness, I obtain these numbers by assuming that the sharpness bar
// only increases in increments of 10 hits (to my knowledge this has been true in every Monster
// Hunter game to date), and carefully measuring the length of each segment. This is an error-prone
// process, so it would be great if someone could double-check my numbers or point me to a reliable
// resource that displays exact sharpness numbers. For now, I get sharpness numbers by copying the
// sharpness image from Kiranico (https://mhwilds.kiranico.com/data/weapons) and counting the
// pixels; every 2 pixels equals 10 hits of sharpness. As far as I can tell, Kiranico's sharpness
// images are accurate with one exception: When the sharpness bar is full, Kiranico's image cuts off
// the last 2 pixels. Thus I assume there's always an additional 10 hits over what Kiranico shows in
// these circumstances, and I check in-game to see whether these last 10 hits are the same colour.
pub struct Sharpness {
    red: u16,
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

// EffectiveSharpness represents expected sharpness values after the armor skills Handicraft, Razor
// Sharp, and Master's Touch are taken into account. It is possible for effective sharpness to have
// fractional values, so they must now be represented by doubles.
pub struct EffectiveSharpness {
    red: f64,
    orange: f64,
    yellow: f64,
    green: f64,
    blue: f64,
    white: f64,
}
impl EffectiveSharpness {
    // TODO: Add parameters for Handicraft, Razor Sharp, Master's Touch, and Protective Polish.
    // Calculate effective sharpness from a weapon's inherent sharpness.
    pub fn new(sharpness: &Sharpness) -> EffectiveSharpness {
        let mut red_after_handicraft = sharpness.red;
        let mut orange_after_handicraft = sharpness.orange;
        let mut yellow_after_handicraft = sharpness.yellow;
        let mut green_after_handicraft = sharpness.green;
        let mut blue_after_handicraft = sharpness.blue;
        let mut white_after_handicraft = sharpness.white;

        // TODO: Reduce hits_to_remove according to how much Handicraft the player has.
        let mut hits_to_remove = sharpness.handicraft_locked;
        // Starting from white sharpness and working backwards, remove hits from each sharpness
        // level until all hits locked by handicraft have been removed.
        for after_handicraft_hits in [
            &mut white_after_handicraft,
            &mut blue_after_handicraft,
            &mut green_after_handicraft,
            &mut yellow_after_handicraft,
            &mut orange_after_handicraft,
            &mut red_after_handicraft,
        ] {
            if *after_handicraft_hits < hits_to_remove {
                hits_to_remove -= *after_handicraft_hits;
                *after_handicraft_hits = 0;
            } else {
                *after_handicraft_hits -= hits_to_remove;
                break;
            }
        }

        // TODO: multiply these values by however much sharpness reduction is given by Razor Sharp
        // and Master's Touch.
        EffectiveSharpness {
            red: f64::from(red_after_handicraft),
            orange: f64::from(orange_after_handicraft),
            yellow: f64::from(yellow_after_handicraft),
            green: f64::from(green_after_handicraft),
            blue: f64::from(blue_after_handicraft),
            white: f64::from(white_after_handicraft),
        }

        // TODO: extend the highest sharpness level according to the player's Protective Polish.
    }

    // Calculates the overall raw sharpness modifier the player experiences by considering how many
    // hits are made at each sharpness level before the player sharpens. Adjust
    // HITS_BEFORE_SHARPENING to control how many hits in total are considered.
    pub fn get_avg_raw_sharpness_mod(&self) -> f64 {
        let mut running_modifier = 0.0;
        let mut remaining_hits = HITS_BEFORE_SHARPENING;

        for (effective_hits, raw_sharpness_mod) in [
            (self.white, MOD_SHARPNESS_RAW_WHITE),
            (self.blue, MOD_SHARPNESS_RAW_BLUE),
            (self.green, MOD_SHARPNESS_RAW_GREEN),
            (self.yellow, MOD_SHARPNESS_RAW_YELLOW),
            (self.orange, MOD_SHARPNESS_RAW_ORANGE),
            (self.red, MOD_SHARPNESS_RAW_RED),
        ] {
            if effective_hits < remaining_hits {
                running_modifier += raw_sharpness_mod * effective_hits / HITS_BEFORE_SHARPENING;
                remaining_hits -= effective_hits;
            } else {
                running_modifier += raw_sharpness_mod * remaining_hits / HITS_BEFORE_SHARPENING;
                break;
            }
        }

        running_modifier
    }
}

const MOD_SHARPNESS_RAW_RED: f64 = 0.50;
const MOD_SHARPNESS_RAW_ORANGE: f64 = 0.75;
const MOD_SHARPNESS_RAW_YELLOW: f64 = 1.00;
const MOD_SHARPNESS_RAW_GREEN: f64 = 1.05;
const MOD_SHARPNESS_RAW_BLUE: f64 = 1.20;
const MOD_SHARPNESS_RAW_WHITE: f64 = 1.32;
const MOD_SHARPNESS_ELEMENT_RED: f64 = 0.25;
const MOD_SHARPNESS_ELEMENT_ORANGE: f64 = 0.50;
const MOD_SHARPNESS_ELEMENT_YELLOW: f64 = 0.75;
const MOD_SHARPNESS_ELEMENT_GREEN: f64 = 1.00;
const MOD_SHARPNESS_ELEMENT_BLUE: f64 = 1.063;
const MOD_SHARPNESS_ELEMENT_WHITE: f64 = 1.15;

pub enum WeaponType {
    GreatSword,
    LongSword,
    SwordAndShield,
    DualBlades,
    Hammer,
    HuntingHorn,
    Lance,
    Gunlance,
    SwitchAxe,
    ChargeBlade,
    InsectGlaive,
    LightBowgun,
    HeavyBowgun,
    Bow,
}
