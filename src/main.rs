use crate::armor::head::HEAD_ARMORS;
use crate::hunter::Set;
use crate::weapon::lance::LANCES;
use itertools::iproduct;
use std::time::Instant;

mod armor;
mod decoration;
mod hunter;
mod skill;
mod weapon;

fn main() {
    let mut sets_checked: u64 = 0;
    let mut highest_effective_raw_set = None;
    let mut highest_effective_raw = f64::MIN;
    let start = Instant::now();

    for (weapon, head, chest, arms, waist, legs, talisman) in iproduct!(
        &LANCES,
        &HEAD_ARMORS,
        &HEAD_ARMORS,
        &HEAD_ARMORS,
        &HEAD_ARMORS,
        &HEAD_ARMORS,
        &HEAD_ARMORS
    ) {
        let set = Set {
            weapon,
            head,
            chest,
            arms,
            waist,
            legs,
            talisman,
        };
        set.print_one_line();

        let hunter = set.get_hunter();
        if hunter.effective_raw > highest_effective_raw {
            highest_effective_raw = hunter.effective_raw;
            highest_effective_raw_set = Some(set);
        }

        sets_checked += 1;
    }

    let mut formatted_search_time = format!("{:04}", start.elapsed().as_millis());
    formatted_search_time.insert(formatted_search_time.len() - 3, '.');
    println!("Checked {sets_checked} sets in {formatted_search_time} s.");
    if let Some(highest_effective_raw_set) = highest_effective_raw_set {
        println!("Best set found ({highest_effective_raw} effective raw):");
        highest_effective_raw_set.print();
    } else {
        println!("No set found.");
    }
}
