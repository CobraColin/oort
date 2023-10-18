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

pub fn kinematic_angular_position(
    initial_angle: f64,
    initial_angular_velocity: f64,
    angular_acceleration: f64,
    time: f64,
) -> f64 {
    // calculates θ = θ + ω​t + ½​αt²
    let predicted_angle = initial_angle + initial_angular_velocity * time + 0.5 * angular_acceleration * time.powi(2);
    // return θ
    predicted_angle
}
