use crate::decoration::armor_decoration::ALL_ARMOR_DECORATIONS;
use crate::decoration::weapon_decoration::ALL_WEAPON_DECORATIONS;
use crate::skill::{Skill, SkillAmount};

mod armor_decoration;
mod weapon_decoration;

pub struct Decoration {
    pub name: &'static str,
    pub size: u8,
    pub skills: &'static [SkillAmount],
}

pub fn get_decoration_pool(weapon_skills: &[&Skill], armor_skills: &[&Skill]) -> DecorationPool {
    // Filter down the decorations to only consider decorations that have at least one of the
    // specified skills. This is important because many skills are useless and searching through all
    // decoration combinations is the slowest part of checking every set.
    let weapon_decoration_pool: Vec<_> = ALL_WEAPON_DECORATIONS
        .iter()
        .filter(|&&decoration| {
            decoration.skills.iter().any(|skill_amount| {
                weapon_skills
                    .iter()
                    .any(|&weapon_skill| skill_amount.skill == weapon_skill)
            })
        })
        // We want another copy of the reference to the Decoration (&Decoration), rather than a new
        // reference to the reference to the Decoration (&&Decoration).
        .copied()
        .collect();

    println!("Filtered weapon decorations:");
    for decoration in &weapon_decoration_pool {
        println!("{}", decoration.name);
    }
    println!();

    // Divide the decoration pool into three separate decoration pools for each decoration slot
    // size.
    let size_one_weapon_decorations: Vec<_> = weapon_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 1)
        .copied()
        .collect();

    // Technically, all one-slot decorations could be placed in a two-slot. But since the vast
    // majority of two-slot decorations are strictly better than one-slot decorations, this is
    // extremely unlikely to be required for the most optimal set. To check for one-slot decorations
    // in two-slots would massively increase the search space, which I don't deem worth it for
    // something so unlikely to be optimal. Hence only two-slot decorations are considered for
    // placement in a two-slot. For the same reasons, only three-slot decorations are considered for
    // placement in a three-slot.
    let size_two_weapon_decorations: Vec<_> = weapon_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 2)
        .copied()
        .collect();
    let size_three_weapon_decorations: Vec<_> = weapon_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 3)
        .copied()
        .collect();

    // Repeat the above process for armor decorations.
    let armor_decoration_pool: Vec<_> = ALL_ARMOR_DECORATIONS
        .iter()
        .filter(|&&decoration| {
            decoration.skills.iter().any(|skill_amount| {
                armor_skills
                    .iter()
                    .any(|&armor_skill| skill_amount.skill == armor_skill)
            })
        })
        .copied()
        .collect();

    println!("Filtered armor decorations:");
    for decoration in &armor_decoration_pool {
        println!("{}", decoration.name);
    }
    println!();

    let size_one_armor_decorations: Vec<_> = armor_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 1)
        .copied()
        .collect();

    // Unlike weapon decorations, the skills provided by armor decorations are completely different
    // between decoration sizes. For now, I continue to assume larger armor decorations are strictly
    // better than smaller ones to make testing easier, but this will likely be relaxed later.
    let size_two_armor_decorations: Vec<_> = armor_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 2)
        .copied()
        .collect();
    let size_three_armor_decorations: Vec<_> = armor_decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 3)
        .copied()
        .collect();

    DecorationPool {
        weapon_three: size_three_weapon_decorations,
        weapon_two: size_two_weapon_decorations,
        weapon_one: size_one_weapon_decorations,
        armor_three: size_three_armor_decorations,
        armor_two: size_two_armor_decorations,
        armor_one: size_one_armor_decorations,
    }
}

pub struct DecorationPool {
    pub weapon_three: Vec<&'static Decoration>,
    pub weapon_two: Vec<&'static Decoration>,
    pub weapon_one: Vec<&'static Decoration>,
    pub armor_three: Vec<&'static Decoration>,
    pub armor_two: Vec<&'static Decoration>,
    pub armor_one: Vec<&'static Decoration>,
}
