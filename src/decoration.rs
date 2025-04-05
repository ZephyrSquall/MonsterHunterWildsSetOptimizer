use crate::decoration::weapon_decoration::ALL_WEAPON_DECORATIONS;
use crate::skill::{SkillAmount, SkillId};

pub mod armor_decoration;
mod weapon_decoration;

pub struct Decoration {
    pub name: &'static str,
    pub size: u8,
    pub skills: &'static [SkillAmount],
}

pub fn get_weapon_decoration_pools() -> (
    Vec<&'static Decoration>,
    Vec<&'static Decoration>,
    Vec<&'static Decoration>,
) {
    // Searching all possible combination of decorations in weapon slots makes the search take too
    // long. For testing purposes, the full list of decorations is filtered to only include
    // decorations that contains at least one of a certain set of skills.
    let decoration_pool: Vec<_> = ALL_WEAPON_DECORATIONS
        .iter()
        .filter(|&&decoration| {
            decoration.skills.iter().any(|skill_amount| {
                matches!(
                    skill_amount.skill.id,
                    SkillId::AttackBoost | SkillId::CriticalEye | SkillId::CriticalBoost
                )
            })
        })
        .copied()
        .collect();

    println!("Filtered weapon decorations:");
    for decoration in &decoration_pool {
        println!("{}", decoration.name);
    }
    println!();

    // Divide the decoration pool into three separate decoration pools for each decoration slot
    // size.
    let size_one_weapon_decorations: Vec<_> = decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 1)
        // Copy the reference to the decoration instead of reusing the reference to the reference to
        // the decoration. This ensures collections of decorations remain of type &[&Decoration]
        // (without using .copied, it would otherwise be &[&&Decoration]) which simplifies function
        // signatures.
        .copied()
        .collect();

    // Technically, all one-slot decorations could be placed in a two-slot. But since the vast
    // majority of two-slot decorations are strictly better than one-slot decorations, this is
    // extremely unlikely to be required for the most optimal set. To check for one-slot decorations
    // in two-slots would massively increase the search space, which I don't deem worth it for
    // something so unlikely to be optimal. Hence only two-slot decorations are considered for
    // placement in a two-slot. For the same reasons, only three-slot decorations are considered for
    // placement in a three-slot.
    let size_two_weapon_decorations: Vec<_> = decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 2)
        .copied()
        .collect();
    let size_three_weapon_decorations: Vec<_> = decoration_pool
        .iter()
        .filter(|&&decoration| decoration.size == 3)
        .copied()
        .collect();

    (
        size_one_weapon_decorations,
        size_two_weapon_decorations,
        size_three_weapon_decorations,
    )
}
