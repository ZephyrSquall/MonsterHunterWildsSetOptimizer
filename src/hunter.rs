use crate::armor::{Armor, Talisman};
use crate::skill::SkillId;
use crate::skill::weapon_skill::WEAPON_SKILLS;
use crate::weapon::Weapon;

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
    fn get_skills(&self) -> Vec<&(SkillId, u8)> {
        // 10 capacity is just an arbitrary choice; I may experiment to find the capacity that gives
        // the best performance later, or figure out the theoretical maximum number of unique skills
        // that can be on a set and use that for capacity.
        let mut skills = Vec::with_capacity(10);

        // The skills from the weapon and first armor piece can just be directly pushed to the
        // skills vector, as they are guaranteed to be unique skills. For all remaining armor pieces
        // and decorations, the skills vector must be checked to see if the hunter already has a
        // skill before adding it.
        for skill in self.weapon.skills {
            skills.push(skill);
        }
        for skill in self.head.skills {
            skills.push(skill);
        }

        // TODO: If any skill went over its maximum, reduce it to its maximum.

        skills
    }

    pub fn get_hunter(&self) -> Hunter {
        let base_attack = self.weapon.attack;
        let base_affinity = self.weapon.affinity;

        let mut bonus_attack = 0;
        let mut bonus_affinity = 0;

        for (skill_id, level) in self.get_skills() {
            let mut skill = None;
            for weapon_skill in WEAPON_SKILLS {
                if weapon_skill.id == *skill_id {
                    skill = Some(weapon_skill);
                }
            }

            if let Some(skill) = skill {
                (skill.modifier.attack)(*level, &mut bonus_attack, base_attack);
                (skill.modifier.affinity)(*level, &mut bonus_affinity);
            } else {
                // Printing this now causes runtime to go from roughly 0.03 s to over 100 s due to
                // the vast number of times it is printed. It is commented out for now, but should
                // be uncommented when skills are properly implemented to check for missed skill
                // implementations.
                //eprintln!("Skill not found");
            }
        }

        let total_attack = base_attack + bonus_attack;
        let total_affinity = base_affinity + bonus_affinity;

        Hunter {
            base_attack,
            bonus_attack,
            total_attack,
            base_affinity,
            bonus_affinity,
            total_affinity,
            // TODO: Handle negative affinity, over 100% affinity, and crit boost.
            effective_raw: f64::from(total_attack)
                * (1.0 + 1.25 * f64::from(total_affinity) / 100.0),
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

pub struct Hunter {
    pub base_attack: u16,
    pub bonus_attack: u16,
    pub total_attack: u16,
    pub base_affinity: i16,
    pub bonus_affinity: i16,
    pub total_affinity: i16,
    pub effective_raw: f64,
}
impl Hunter {
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
}
