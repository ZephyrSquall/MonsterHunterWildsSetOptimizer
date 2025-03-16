use crate::armor::head::HEAD_ARMORS;
use crate::hunter::Set;
use crate::weapon::lance::LANCES;

mod armor;
mod decoration;
mod hunter;
mod skill;
mod weapon;

fn main() {
    let set = Set {
        weapon: &LANCES[0],
        head: &HEAD_ARMORS[0],
        chest: &HEAD_ARMORS[0],
        arms: &HEAD_ARMORS[0],
        waist: &HEAD_ARMORS[0],
        legs: &HEAD_ARMORS[0],
        talisman: &HEAD_ARMORS[0],
    };

    let hunter = set.get_hunter();

    println!("Base attack: {}", hunter.base_attack);
    println!("Bonus attack: {}", hunter.bonus_attack);
    println!("Total attack: {}", hunter.total_attack);
    println!("Base affinity: {}", hunter.base_affinity);
    println!("Bonus affinity: {}", hunter.bonus_affinity);
    println!("Total affinity: {}", hunter.total_affinity);
}
