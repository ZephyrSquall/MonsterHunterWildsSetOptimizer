use crate::armor::arms::ARMS_ARMORS;
use crate::armor::chest::CHEST_ARMORS;
use crate::armor::head::HEAD_ARMORS;
use crate::armor::legs::LEGS_ARMORS;
use crate::armor::talisman::TALISMANS;
use crate::armor::waist::WAIST_ARMORS;
use crate::decoration::get_weapon_decoration_pools;
use crate::hunter::Set;
use crate::weapon::lance::LANCES;
use hunter::Hunter;
use itertools::iproduct;
use rayon::prelude::*;
use std::{sync::RwLock, time::Instant};

mod armor;
mod config;
mod decoration;
mod hunter;
mod skill;
mod weapon;

fn main() {
    let (size_one_weapon_decorations, size_two_weapon_decorations, size_three_weapon_decorations) =
        get_weapon_decoration_pools();

    let set_iter = iproduct!(
        LANCES,
        HEAD_ARMORS,
        CHEST_ARMORS,
        ARMS_ARMORS,
        WAIST_ARMORS,
        LEGS_ARMORS,
        TALISMANS,
    );

    let sets_checked = set_iter.clone().count();
    // Write access is only needed when a new highest-damage set is found, and the overwhelming
    // majority of sets that are checked will not be the highest-damage set found so far, so a
    // read-write lock is preferred over a mutex.
    let highest_efr_hunter_lock = RwLock::new(None);
    let start = Instant::now();

    set_iter
        .par_bridge()
        .for_each(|(weapon, head, chest, arms, waist, legs, talisman)| {
            let set = Set {
                weapon,
                head,
                chest,
                arms,
                waist,
                legs,
                talisman,
            };

            let hunter = set.get_hunter(
                &size_one_weapon_decorations,
                &size_two_weapon_decorations,
                &size_three_weapon_decorations,
            );

            // Get a read lock to check if the set found is the highest damage set found so far (or
            // is a None, which means the set found is the first set and is trivially the highest
            // damage set found so far). If so, get the write lock to update it.
            let highest_efr_hunter = highest_efr_hunter_lock.read().unwrap();
            if highest_efr_hunter
                .as_ref()
                .is_none_or(|highest_efr_hunter: &Hunter| {
                    hunter.effective_raw > highest_efr_hunter.effective_raw
                })
            {
                // Manually dropping highest_efr_hunter is required to ensure this thread releases
                // its read lock before trying to acquire the write lock. Otherwise, this thread
                // could try to acquire a read lock and the write lock at the same time, causing a
                // deadlock.
                std::mem::drop(highest_efr_hunter);
                let mut highest_efr_hunter = highest_efr_hunter_lock.write().unwrap();
                *highest_efr_hunter = Some(hunter);
            }
        });

    let mut formatted_search_time = format!("{:04}", start.elapsed().as_millis());
    formatted_search_time.insert(formatted_search_time.len() - 3, '.');
    println!("Checked {sets_checked} sets in {formatted_search_time} s.");

    if let Some(highest_efr_hunter) = highest_efr_hunter_lock.into_inner().unwrap() {
        println!(
            "Best set found ({} effective raw):",
            highest_efr_hunter.effective_raw
        );
        println!(
            "Sharpness mod: {}",
            highest_efr_hunter.total_raw_sharpness_mod
        );
        highest_efr_hunter.set.print();
        println!("Decorations:");
        highest_efr_hunter.print_decorations();
        println!("Skills:");
        highest_efr_hunter.print_skills();
    } else {
        println!("No set found.");
    }
}
