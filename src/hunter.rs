use crate::armor::Armor;
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
    pub talisman: &'a Armor,
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
                eprintln!("Skill not found");
            }
        }

        Hunter {
            base_attack,
            bonus_attack,
            total_attack: base_attack + bonus_attack,
            base_affinity,
            bonus_affinity,
            total_affinity: base_affinity + bonus_affinity,
        }
    }
}

pub struct Hunter {
    pub base_attack: u16,
    pub bonus_attack: u16,
    pub total_attack: u16,
    pub base_affinity: i16,
    pub bonus_affinity: i16,
    pub total_affinity: i16,
}
