use crate::weapon::Weapon;

pub mod armor_skill;
pub mod group_skill;
pub mod set_bonus_skill;
pub mod weapon_skill;

pub struct Skill {
    pub name: &'static str,
    // alt_name is used for Set Bonus and Group skills to indicate the name of the skill that is
    // granted if enough of the Set Bonus or Group pieces are equipped. e.g. "Gore Magala's Tyranny"
    // is the name, "Black Eclipse" (without the "I" or "II") is the alt_name. For Weapon and Armor
    // skills, alt_name is None.
    pub alt_name: Option<&'static str>,
    id: SkillId,
    skill_type: SkillType,
    max: u8,
    pub apply: fn(modifier: &mut Modifier, level: u8, weapon: &Weapon),
}
impl PartialEq for Skill {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Skill {}

pub struct Modifier {
    pub attack_multiplier: f64,
    pub bonus_attack: f64,
    pub element_multiplier: f64,
    pub bonus_element: f64,
    pub bonus_affinity: f64,
    pub raw_crit_multiplier: f64,
    pub element_crit_multiplier: f64,
    pub status_crit_multiplier: f64,
}
impl Default for Modifier {
    fn default() -> Self {
        Modifier {
            attack_multiplier: 1.0,
            bonus_attack: 0.0,
            element_multiplier: 1.0,
            bonus_element: 0.0,
            bonus_affinity: 0.0,
            raw_crit_multiplier: 1.25,
            element_crit_multiplier: 1.0,
            status_crit_multiplier: 1.0,
        }
    }
}

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
            .find(|other_skill_amount| self.skill == other_skill_amount.skill)
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
#[derive(PartialEq, Eq)]
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
    SlickedBlade,
    Slugger,
    SpecialAmmoBoost,
    SpeedSharpening,
    SpreadPowerShots,
    StaminaThief,
    TetradShot,
    ThunderAttack,
    WaterAttack,
    WhiteflameTorrent,

    // Armor skills
    Adaptability,
    AdrenalineRush,
    Agitator,
    Ambush,
    Antivirus,
    AquaticOilsiltMobility,
    BindResistance,
    BlastResistance,
    BleedingResistance,
    BlightResistance,
    Blindsider,
    Bombardier,
    Botanist,
    Burst,
    Cliffhanger,
    Coalescence,
    Constitution,
    ConvertElement,
    Counterstrike,
    DefenseBoost,
    DivineBlessing,
    DragonResistance,
    Earplugs,
    ElementalAbsorption,
    Entomologist,
    EvadeExtender,
    EvadeWindow,
    FireResistance,
    Flayer,
    FlinchFree,
    Foray,
    FreeMeal,
    Geologist,
    Heroics,
    HungerResistance,
    IceResistance,
    Intimidator,
    IronSkin,
    ItemProlonger,
    JumpMaster,
    LatentPower,
    LeapOfFaith,
    MarathonRunner,
    MaximumMight,
    Mushroomancer,
    Outdoorsman,
    ParalysisResistance,
    Partbreaker,
    PeakPerformance,
    PoisonResistance,
    QuickSheathe,
    RecoverySpeed,
    RecoveryUp,
    Resentment,
    SelfImprovement,
    ShockAbsorber,
    SleepResistance,
    SpeedEating,
    StaminaSurge,
    StenchResistance,
    StunResistance,
    SurvivalExpert,
    ThunderResistance,
    ToolSpecialist,
    TremorResistance,
    WaterResistance,
    WeaknessExploit,
    WideRange,
    Windproof,

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
