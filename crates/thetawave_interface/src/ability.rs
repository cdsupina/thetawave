/// User activatable ability
pub enum Ability {
    PrimaryAttack(PrimaryAttackType),
    SecondaryAttack(SecondaryAttackType),
}

/// Low damage and short cooldown attack
pub enum PrimaryAttackType {
    FireBlasts,
    FireBullets,
}

/// High damage and long cooldown attack
pub enum SecondaryAttackType {
    MegaBlast,
    Charge,
}
