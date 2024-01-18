use bevy_ecs::component::Component;
use bevy_time::Timer;

#[derive(Component, Clone)]
pub struct ChargeAilityComponent {
    /// Tracks time until next charge can be used
    pub recharge_timer: Timer,
    /// Initial delay before first charge can be used
    pub initial_timer: Timer,
    /// Whether the ability can currently be used
    pub is_enabled: bool,
}
