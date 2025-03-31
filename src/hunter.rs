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
        size_one_weapon_decorations: &[&Decoration],
        size_two_weapon_decorations: &[&Decoration],
        size_three_weapon_decorations: &[&Decoration],
    ) -> Hunter<'a> {
        let base_attack = self.weapon.attack;
        let base_affinity = self.weapon.affinity;

        let mut bonus_attack = 0.0;
        let mut bonus_affinity = 0.0;

        let mut base_skills = self.get_skills();
        for skill_amount in &base_skills {
            (skill_amount.skill.modifier.attack)(
                skill_amount.level,
                &mut bonus_attack,
                self.weapon,
            );
            (skill_amount.skill.modifier.affinity)(skill_amount.level, &mut bonus_affinity);
        }

        let one_slots = self.weapon.slots.iter().filter(|&slot| *slot == 1).count();
        let two_slots = self.weapon.slots.iter().filter(|&slot| *slot == 2).count();
        let three_slots = self.weapon.slots.iter().filter(|&slot| *slot == 3).count();

        let one_slot_decoration_combinations = size_one_weapon_decorations
            .iter()
            .combinations_with_replacement(one_slots);
        let two_slot_decoration_combinations = size_two_weapon_decorations
            .iter()
            .combinations_with_replacement(two_slots);
        let three_slot_decoration_combinations = size_three_weapon_decorations
            .iter()
            .combinations_with_replacement(three_slots);

        // Iterates over every possible combination of weapon decorations from the three weapon
        // decoration pools that fits in this weapon's decoration slots.
        for decoration_combination in [
            one_slot_decoration_combinations,
            two_slot_decoration_combinations,
            three_slot_decoration_combinations,
        ]
        .into_iter()
        .multi_cartesian_product()
        {
            for decoration in decoration_combination.iter().flatten() {
                for skill_amount in decoration.skills {
                    // TODO: Calculate the total skills for this combination of weapon decorations,
                    // and the resulting damage output, then record this combination of weapon
                    // decorations if the damage output is the highest found yet. For now, to test
                    // that this works, the skills are simply added to the base skills of the armor,
                    // which will inevitably max out any skills found on decorations in the weapon
                    // decoration pools.
                    skill_amount.add_to(&mut base_skills);
                }
            }
        }

        let effective_sharpness = EffectiveSharpness::new(&self.weapon.sharpness);
        let total_raw_sharpness_mod = effective_sharpness.get_avg_raw_sharpness_mod();

        let total_attack = f64::from(base_attack) + bonus_attack;
        let total_affinity = f64::from(base_affinity) + bonus_affinity;

        Hunter {
            set: self,
            skills: base_skills,
            base_attack,
            bonus_attack,
            total_attack,
            base_affinity,
            bonus_affinity,
            total_affinity,
            effective_sharpness,
            total_raw_sharpness_mod,
            // TODO: Handle negative affinity, over 100% affinity, and crit boost.
            effective_raw: total_attack
                * (1.0 + 1.25 * total_affinity / 100.0)
                * total_raw_sharpness_mod,
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
}
