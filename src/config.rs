// This file is intended to hold all numbers representing assumptions about one's playstyle. It is
// recommended that users of this repository review this file and change any numbers that don't
// match that user's playstyle. Any numeric value defined outside of this file are objective values
// that should never be changed.

// How many units of sharpness the player usually consumes before stopping to sharpen their weapon.
// Note that several weapon attacks don't consume a full unit of sharpness on hit (typically
// individual hits in a multi-hit attack), so this isn't necessarily equivalent to the actual number
// of hits landed between sharpens.
pub const HITS_BEFORE_SHARPENING: f64 = 80.0;
