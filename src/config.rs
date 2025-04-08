// This file is intended to hold all numbers representing assumptions about one's playstyle. It is
// recommended that users of this repository review this file and change any numbers that don't
// match that user's playstyle. Any numeric value defined outside of this file are objective values
// that should never be changed.

// How many units of sharpness the player usually consumes before stopping to sharpen their weapon.
// Note that several weapon attacks don't consume a full unit of sharpness on hit (typically
// individual hits in a multi-hit attack), so this isn't necessarily equivalent to the actual number
// of hits landed between sharpens.
pub const HITS_BEFORE_SHARPENING: f64 = 80.0;

// --- UPTIME ---
// These constants are an estimation of what proportion of the hunt certain skills are active for.
// These should numbers between 0.0 and 1.0.

pub const OFFENSIVE_GUARD_UPTIME: f64 = 0.6;
pub const AGITATOR_UPTIME: f64 = 0.5;
// Do not include five-hit uptime in the one-hit uptime. For example, if one-hit uptime is 0.5 and
// five-hit uptime is 0.4, this means 40% of hits have the full five-hit bonus, 50% of hits only
// have the one-hit bonus, and 10% of hits have no bonus.
pub const BURST_ONE_HIT_UPTIME: f64 = 0.4;
pub const BURST_FIVE_HIT_UPTIME: f64 = 0.4;
pub const MAXIMUM_MIGHT_UPTIME: f64 = 0.6;
// Do not include uptime from hitting wounds in the weak point uptime. For example, if weak point
// uptime is 0.7 and the wound uptime is 0.2, this means 20% of one's damage hits a wound, 70% of
// one's damage hits a weak point that isn't a wound, and 10% of one's damage hits neither a weak
// point nor a wound.
pub const WEAKNESS_EXPLOIT_WEAK_POINT_UPTIME: f64 = 0.7;
pub const WEAKNESS_EXPLOIT_WOUND_UPTIME: f64 = 0.2;

// --- COVERAGE ---
// These constants are an estimation of how much the player's overall damage in a hunt is attributed
// to the specific attacks that are boosted by a skill. Conceptually, this is pretty much identical
// to uptime, but it's a lot more playstyle-dependent since different players use different attacks,
// and players might intentionally use some attacks more if they get a boost. As such, these skills
// are separated from the rest of the uptime skills to make editing them easier. Like uptime, these
// should be numbers between 0.0 and 1.0.

// Airborne boosts aerial attacks.
pub const AIRBORNE_COVERAGE: f64 = 0.01;
// Critical Draw and Punishing Draw boost draw attacks.
pub const CRITICAL_DRAW_AND_PUNISHING_DRAW_COVERAGE: f64 = 0.005;
