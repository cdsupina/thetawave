use serde::Deserialize;

/// Player activatable ability
pub enum Ability {
    PrimaryAttack(PrimaryAttackType),
    SecondaryAttack(SecondaryAttackType),
}

/// Low damage and short cooldown attack
#[derive(Deserialize, Debug, Clone)]
pub enum PrimaryAttackType {
    FireBlasts,
    FireBullets,
}

/// High damage and long cooldown attack
#[derive(Deserialize, Debug, Clone)]
pub enum SecondaryAttackType {
    MegaBlast,
    Charge,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AbilityLoadout {
    pub primary_attack: PrimaryAttackType,
    pub secondary_attack: SecondaryAttackType,
}
