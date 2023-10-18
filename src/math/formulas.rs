use oort_api::prelude::Vec2;

pub fn kinematic_projectile_position(
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    time: f64,
) -> Vec2 {
    // calculates p = p₀ + v₀t + ½at²
    let predicted_position = position + velocity * time + 0.5 * acceleration * time.powi(2);
    // return p
    predicted_position
}