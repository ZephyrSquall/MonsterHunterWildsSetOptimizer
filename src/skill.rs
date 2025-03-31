use crate::weapon::Weapon;

pub mod weapon_skill;

pub struct Skill {
    pub name: &'static str,
    // alt_name is used for Set Bonus and Group skills to indicate the name of the skill that is
    // granted if enough of the Set Bonus or Group pieces are equipped. e.g. "Gore Magala's Tyranny"
    // is the name, "Black Eclipse" (without the "I" or "II") is the alt_name. For Weapon and Armor
    // skills, alt_name is None.
    alt_name: Option<&'static str>,
    pub id: SkillId,
    skill_type: SkillType,
    max: u8,
    pub modifier: Modifier,
}

pub struct Modifier {
    pub attack: fn(level: u8, bonus_attack: &mut f64, weapon: &Weapon),
    pub affinity: fn(level: u8, bonus_affinity: &mut f64),
    raw_crit_multiplier: fn(level: u8, bonus_raw_crit_multiplier: &mut f64),
    element_crit_multiplier:
        fn(level: u8, bonus_element_crit_multiplier: &mut f64, weapon: &Weapon),
    status_crit_multiplier: fn(level: u8, bonus_status_crit_multiplier: &mut f64),
    element: fn(level: u8, bonus_element: &mut f64, weapon: &Weapon),
}

// The default modifiers are simply all no-op functions. Most skills don't affect any given stat, so
// this simplifies skill definitions by only requiring a function to be explicitly provided for
// stats that the skill actually modifies. It is assumed that every skill only performs addition or
// subtraction to the bonus to their stat, and hence modifiers can be applied in any order; as I
// know is true for all skills in the game currently. For each modifier, the first parameter is the
// skill level, the second parameter is the bonus to the stat so far and is mutable so it can be
// modified further, and if needed, the third parameter is a reference to the weapon so the skill
// can check it to see what to do.
const DEFAULT_MODIFIER: Modifier = Modifier {
    attack: |_level, _bonus_attack, _weapon| {},
    affinity: |_level, _bonus_affinity| {},
    raw_crit_multiplier: |_level, _bonus_raw_crit_multiplier| {},
    element_crit_multiplier: |_level, _bonus_element_crit_multiplier, _weapon| {},
    status_crit_multiplier: |_level, _bonus_status_crit_multiplier| {},
    element: |_level, _bonus_element, _weapon| {},
};

pub enum SkillType {
    Weapon,
    Armor,
    SetBonus,
    Group,
}

#[derive(Clone)]
pub struct SkillAmount {
    pub skill: &'static Skill,
    pub level: u8,
}
impl SkillAmount {
    // Using this constructor in const expressions allows for SkillAmounts to be defined in one
    // line, reducing verbosity.
    pub const fn new(skill: &'static Skill, level: u8) -> SkillAmount {
        SkillAmount { skill, level }
    }

    // Add this SkillAmount to a Vec of SkillAmounts. If this skill already exists in the Vec, then
    // the Vec's skill is increased according to this skill's level. Otherwise, this SkillAmount is
    // pushed to the Vec. This method assumes all skills in the Vec are unique, which will always be
    // true if skills are only added to it using this method.
    pub fn add_to(&self, other: &mut Vec<SkillAmount>) {
        // Search the other Vec to see if it contains the skill.
        if let Some(matched_skill_amount) = other
            .iter_mut()
            // To check if both SkillAmounts refer to the same skill, check the SkillId.
            .find(|other_skill_amount| self.skill.id == other_skill_amount.skill.id)
        {
            // If it does contain the skill, add to its level, making sure it doesn't exceed the max
            // level.
            matched_skill_amount.level += self.level;
            if matched_skill_amount.level > matched_skill_amount.skill.max {
                matched_skill_amount.level = matched_skill_amount.skill.max;
            }
        } else {
            // If it doesn't contain the skill, push the self SkillAmount to it. The self
            // SkillAmount must be copied, as the copy in the Vec might be modified later.
            other.push(self.clone());
        }
    }
}

// SkillId is used to compare if two skills are the same. This is needed when calculating the total
// skill points of an armor set to know when to combine two of the same skill, which makes listing a
// set's skills neater and helps in ensuring a skill does not exceed its maximum level.
#[derive(PartialEq)]
pub enum SkillId {
    // Weapon skills
    Airborne,
    Artillery,
    AttackBoost,
    Ballistics,
    BlastAttack,
    BlastFunctionality,
    Bludgeoner,
    ChargeMaster,
    ChargeUp,
    CriticalBoost,
    CriticalDraw,
    CriticalElement,
    CriticalEye,
    CriticalStatus,
    DragonAttack,
    ExhaustFunctionality,
    FireAttack,
    Focus,
    Guard,
    GuardUp,
    Handicraft,
    HornMaestro,
    IceAttack,
    LoadShells,
    MastersTouch,
    MindsEye,
    NormalShots,
    OffensiveGuard,
    OpeningShot,
    ParaFunctionality,
    ParalysisAttack,
    PiercingShots,
    PoisonAttack,
    PoisonDurationUp,
    PoisonFunctionality,
    PowerProlonger,
    ProtectivePolish,
    PunishingDraw,
    RapidFireUp,
    RapidMorph,
    RazorSharp,
    SleepAttack,
    SleepFunctionality,
    Slugger,
    SpecialAmmoBoost,
    SpeedSharpening,
    SpreadPowerShots,
    StaminaThief,
    TetradShot,
    ThunderAttack,
    WaterAttack,

    // Armor skills
    Agitator,
    Antivirus,
    BlightResistance,
    Burst,
    Coalescence,
    Constitution,
    ConvertElement,
    ElementalAbsorption,
    EvadeWindow,
    Flayer,
    FlinchFree,
    Partbreaker,
    QuickSheathe,
    RecoverySpeed,
    WeaknessExploit,

    // Set Bonus skills
    ArkveldsHunger,
    GoreMagalasTyranny,
    GuardianArkveldsVitality,

    // Group skills
    AlluringPelt,
    FortifyingPelt,
    GuardiansProtection,
    GuardiansPulse,
    ScaleLayering,
    ScalingProwess,
}
