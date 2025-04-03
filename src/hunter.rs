use crate::armor::{Armor, Talisman};
use crate::decoration::Decoration;
use crate::skill::SkillAmount;
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
    fn get_skills(&self) -> Vec<SkillAmount> {
        // 10 capacity is just an arbitrary choice; I may experiment to find the capacity that gives
        // the best performance later, or figure out the theoretical maximum number of unique skills
        // that can be on a set and use that for capacity.
        let mut skills = Vec::with_capacity(10);

        for skill_amount in self.weapon.skills {
            skill_amount.add_to(&mut skills);
        }

        skills
    }

    pub fn get_hunter(
        self,
        size_one_weapon_decorations: &[&'static Decoration],
        size_two_weapon_decorations: &[&'static Decoration],
        size_three_weapon_decorations: &[&'static Decoration],
    ) -> Hunter<'a> {
        let base_attack = self.weapon.attack;
        let base_affinity = self.weapon.affinity;

        let base_skills = self.get_skills();
        let mut highest_efr_hunter = None;

        let three_slot_decoration_combinations = size_three_weapon_decorations
            .iter()
            .combinations_with_replacement(self.weapon.three_slots as usize);
        let two_slot_decoration_combinations = size_two_weapon_decorations
            .iter()
            .combinations_with_replacement(self.weapon.two_slots as usize);
        let one_slot_decoration_combinations = size_one_weapon_decorations
            .iter()
            .combinations_with_replacement(self.weapon.one_slots as usize);

        // Iterates over every possible combination of weapon decorations from the three weapon
        // decoration pools that fits in this weapon's decoration slots.
        for decoration_combination in [
            three_slot_decoration_combinations,
            two_slot_decoration_combinations,
            one_slot_decoration_combinations,
        ]
        .into_iter()
        .multi_cartesian_product()
        {
            let mut decoration_combination_skills = base_skills.clone();
            let mut decorations = Vec::with_capacity(3);
            for decoration in decoration_combination.iter().flatten() {
                for skill_amount in decoration.skills {
                    skill_amount.add_to(&mut decoration_combination_skills);
                }
                // Here, "decoration" is a triple reference - a reference from the
                // decoration_combination vector to a reference in one of the
                // n_slot_decoration_combinations vectors to a reference to a Decoration. We want a
                // copy of the reference to a Decoration, a single reference, so deference and then
                // clone to strip away the first two layers of the triple reference.
                decorations.push(*decoration.clone());
            }

            let mut bonus_attack = 0.0;
            let mut bonus_affinity = 0.0;
            for skill_amount in &decoration_combination_skills {
                (skill_amount.skill.modifier.attack)(
                    skill_amount.level,
                    &mut bonus_attack,
                    self.weapon,
                );
                (skill_amount.skill.modifier.affinity)(skill_amount.level, &mut bonus_affinity);
            }

            let effective_sharpness = EffectiveSharpness::new(&self.weapon.sharpness);
            let total_raw_sharpness_mod = effective_sharpness.get_avg_raw_sharpness_mod();

            let total_attack = f64::from(base_attack) + bonus_attack;
            let total_affinity = f64::from(base_affinity) + bonus_affinity;

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
                    skills: decoration_combination_skills,
                    decorations,
                    base_attack,
                    bonus_attack,
                    total_attack,
                    base_affinity,
                    bonus_affinity,
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
                skills: highest_efr_hunter.skills,
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
    pub skills: Vec<SkillAmount>,
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
        for skill_amount in &self.skills {
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
    pub skills: Vec<SkillAmount>,
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
