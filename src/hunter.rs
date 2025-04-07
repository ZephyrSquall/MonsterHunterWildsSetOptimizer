use crate::armor::{Armor, Talisman};
use crate::decoration::{Decoration, DecorationPool};
use crate::skill::{Modifier, SkillAmount};
use crate::weapon::{EffectiveSharpness, Weapon};
use itertools::Itertools;

pub struct Set<'a> {
    pub weapon: &'a Weapon,
    pub head: &'a Armor,
    pub chest: &'a Armor,
    pub arms: &'a Armor,
    pub waist: &'a Armor,
    pub legs: &'a Armor,
    pub talisman: &'a Talisman,
}
impl<'a> Set<'a> {
    fn get_weapon_skills(&self) -> Vec<SkillAmount> {
        // 10 capacity is just an arbitrary choice; I may experiment to find the capacity that gives
        // the best performance later, or figure out the theoretical maximum number of unique skills
        // that can be on a set and use that for capacity.
        let mut skills = Vec::with_capacity(10);

        for skill_amount in self.weapon.skills {
            skill_amount.add_to(&mut skills);
        }

        skills
    }

    fn get_armor_skills(&self) -> Vec<SkillAmount> {
        let mut skills = Vec::with_capacity(20);

        for skill_amount in self
            .head
            .skills
            .iter()
            .chain(self.chest.skills.iter())
            .chain(self.arms.skills.iter())
            .chain(self.waist.skills.iter())
            .chain(self.legs.skills.iter())
            .chain(self.talisman.skills)
        {
            skill_amount.add_to(&mut skills);
        }

        skills
    }

    fn get_armor_slots(&self) -> (u8, u8, u8) {
        let mut armor_three_slots = 0;
        let mut armor_two_slots = 0;
        let mut armor_one_slots = 0;

        for armor in [self.head, self.chest, self.arms, self.waist, self.legs] {
            armor_three_slots += armor.three_slots;
            armor_two_slots += armor.two_slots;
            armor_one_slots += armor.one_slots;
        }

        (armor_three_slots, armor_two_slots, armor_one_slots)
    }

    pub fn get_hunter(self, decoration_pool: &DecorationPool) -> Hunter<'a> {
        let base_attack = self.weapon.attack;
        let base_affinity = self.weapon.affinity;

        let base_weapon_skills = self.get_weapon_skills();
        let base_armor_skills = self.get_armor_skills();
        let (armor_three_slots, armor_two_slots, armor_one_slots) = self.get_armor_slots();
        let mut highest_efr_hunter = None;

        let weapon_three_decoration_combinations = decoration_pool
            .weapon_three
            .iter()
            .combinations_with_replacement(self.weapon.three_slots as usize);
        let weapon_two_decoration_combinations = decoration_pool
            .weapon_two
            .iter()
            .combinations_with_replacement(self.weapon.two_slots as usize);
        let weapon_one_decoration_combinations = decoration_pool
            .weapon_one
            .iter()
            .combinations_with_replacement(self.weapon.one_slots as usize);

        let armor_three_decoration_combinations = decoration_pool
            .armor_three
            .iter()
            .combinations_with_replacement(armor_three_slots as usize);
        let armor_two_decoration_combinations = decoration_pool
            .armor_two
            .iter()
            .combinations_with_replacement(armor_two_slots as usize);
        let armor_one_decoration_combinations = decoration_pool
            .armor_one
            .iter()
            .combinations_with_replacement(armor_one_slots as usize);

        // Iterates over every possible combination of decorations from the six weapon decoration
        // pools (three weapon decoration pools and three armor decoration pools, each for
        // decorations of size one, two, and three) that fits in this weapon's decoration slots.
        for decoration_combination in [
            weapon_three_decoration_combinations,
            weapon_two_decoration_combinations,
            weapon_one_decoration_combinations,
            armor_three_decoration_combinations,
            armor_two_decoration_combinations,
            armor_one_decoration_combinations,
        ]
        .into_iter()
        .multi_cartesian_product()
        {
            let mut decoration_combination_weapon_skills = base_weapon_skills.clone();
            let mut decoration_combination_armor_skills = base_armor_skills.clone();
            let mut decorations = Vec::with_capacity(20);

            let mut decoration_combination_iter = decoration_combination.iter();

            // "decoration_combination" represents one possible combination of decorations that can
            // be equipped to this set. It is a vector of six inner vectors, which each contain the
            // decorations in the following order: weapon three-slots, weapon two-slots, weapon
            // one-slots, armor three-slots, armor two-slots, armor one-slots. To get just the
            // weapon decorations, take the first three vectors, then flatten them so the three
            // weapon decoration vectors are combined into a single vector that can be iterated
            // over. The iterator must be taken as a reference first so it isn't consumed, letting
            // it be reused later to get the armor decorations next.
            for weapon_decoration in decoration_combination_iter.by_ref().take(3).flatten() {
                for skill_amount in weapon_decoration.skills {
                    skill_amount.add_to(&mut decoration_combination_weapon_skills);
                }
                // Here, "weapon_decoration" is a triple reference - a reference from the
                // decoration_combination vector to a reference in one of the
                // weapon_n_decoration_combinations vectors to a reference to a Decoration. We want
                // a copy of the reference to a Decoration, a single reference, so deference and
                // then clone to strip away the first two layers of the triple reference.
                decorations.push(*weapon_decoration.clone());
            }

            // The first three inner vectors have now been iterated over, so all that remains in
            // decoration_combination_iter are the last three inner vectors corresponding to armor
            // decorations.
            for armor_decoration in decoration_combination_iter.flatten() {
                for skill_amount in armor_decoration.skills {
                    skill_amount.add_to(&mut decoration_combination_armor_skills);
                }
                decorations.push(*armor_decoration.clone());
            }

            // Apply each skill to obtain all of the modifiers to the player's stats.
            let mut modifier = Modifier::default();
            for skill_amount in decoration_combination_weapon_skills
                .iter()
                .chain(decoration_combination_armor_skills.iter())
            {
                (skill_amount.skill.apply)(&mut modifier, skill_amount.level, self.weapon);
            }

            let effective_sharpness = EffectiveSharpness::new(&self.weapon.sharpness);
            let total_raw_sharpness_mod = effective_sharpness.get_avg_raw_sharpness_mod();

            let total_attack =
                f64::from(base_attack) * modifier.attack_multiplier + modifier.bonus_attack;
            let total_affinity = f64::from(base_affinity) + modifier.bonus_affinity;

            // TODO: Handle negative affinity, over 100% affinity, and crit boost.
            let effective_raw =
                total_attack * (1.0 + 1.25 * total_affinity / 100.0) * total_raw_sharpness_mod;

            if highest_efr_hunter
                .as_ref()
                .is_none_or(|highest_efr_hunter: &HunterStats| {
                    effective_raw > highest_efr_hunter.effective_raw
                })
            {
                highest_efr_hunter = Some(HunterStats {
                    weapon_skills: decoration_combination_weapon_skills,
                    armor_skills: decoration_combination_armor_skills,
                    decorations,
                    base_attack,
                    bonus_attack: modifier.bonus_attack,
                    total_attack,
                    base_affinity,
                    bonus_affinity: modifier.bonus_affinity,
                    total_affinity,
                    effective_sharpness,
                    total_raw_sharpness_mod,
                    effective_raw,
                });
            }
        }

        if let Some(highest_efr_hunter) = highest_efr_hunter {
            Hunter {
                set: self,
                weapon_skills: highest_efr_hunter.weapon_skills,
                armor_skills: highest_efr_hunter.armor_skills,
                decorations: highest_efr_hunter.decorations,
                base_attack: highest_efr_hunter.base_attack,
                bonus_attack: highest_efr_hunter.bonus_attack,
                total_attack: highest_efr_hunter.total_attack,
                base_affinity: highest_efr_hunter.base_affinity,
                bonus_affinity: highest_efr_hunter.bonus_affinity,
                total_affinity: highest_efr_hunter.total_affinity,
                effective_sharpness: highest_efr_hunter.effective_sharpness,
                total_raw_sharpness_mod: highest_efr_hunter.total_raw_sharpness_mod,
                effective_raw: highest_efr_hunter.effective_raw,
            }
        } else {
            panic!("No set found");
        }
    }

    pub fn print(&self) {
        println!("Weapon: {}", self.weapon.name);
        println!("Head:   {}", self.head.name);
        println!("Chest:  {}", self.chest.name);
        println!("Arms:   {}", self.arms.name);
        println!("Waist:  {}", self.waist.name);
        println!("Legs:   {}", self.legs.name);
        println!("Talis:  {}", self.talisman.name);
    }

    pub fn print_one_line(&self) {
        println!(
            "{} -- {} -- {} -- {} -- {} -- {}",
            self.weapon.name,
            self.chest.name,
            self.arms.name,
            self.waist.name,
            self.legs.name,
            self.talisman.name
        );
    }
}

pub struct Hunter<'a> {
    pub set: Set<'a>,
    pub weapon_skills: Vec<SkillAmount>,
    pub armor_skills: Vec<SkillAmount>,
    pub decorations: Vec<&'static Decoration>,
    pub base_attack: u16,
    pub bonus_attack: f64,
    pub total_attack: f64,
    pub base_affinity: i16,
    pub bonus_affinity: f64,
    pub total_affinity: f64,
    pub effective_sharpness: EffectiveSharpness,
    pub total_raw_sharpness_mod: f64,
    pub effective_raw: f64,
}
impl Hunter<'_> {
    pub fn print_summary(&self) {
        println!("Effective raw: {}", self.effective_raw);
    }

    pub fn print_verbose(&self) {
        println!("Base attack: {}", self.base_attack);
        println!("Bonus attack: {}", self.bonus_attack);
        println!("Total attack: {}", self.total_attack);
        println!("Base affinity: {}", self.base_affinity);
        println!("Bonus affinity: {}", self.bonus_affinity);
        println!("Total affinity: {}", self.total_affinity);
        println!("Effective raw: {}", self.effective_raw);
    }

    pub fn print_skills(&self) {
        for skill_amount in self.weapon_skills.iter().chain(self.armor_skills.iter()) {
            println!("{} {}", skill_amount.skill.name, skill_amount.level);
        }
    }

    pub fn print_decorations(&self) {
        for decoration in &self.decorations {
            println!("{}", decoration.name);
        }
    }
}

// Hunter without the set information. Used to compare stats between possible skill combinations
// before the highest-damage set is found and the set is moved into it, as the set itself can only
// be moved once.
pub struct HunterStats {
    pub weapon_skills: Vec<SkillAmount>,
    pub armor_skills: Vec<SkillAmount>,
    pub decorations: Vec<&'static Decoration>,
    pub base_attack: u16,
    pub bonus_attack: f64,
    pub total_attack: f64,
    pub base_affinity: i16,
    pub bonus_affinity: f64,
    pub total_affinity: f64,
    pub effective_sharpness: EffectiveSharpness,
    pub total_raw_sharpness_mod: f64,
    pub effective_raw: f64,
}
