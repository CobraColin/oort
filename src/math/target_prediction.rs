use super::formulas::kinematic_projectile_position;

use oort_api::prelude::{Vec2, Vec2Extras};

pub fn predict_target_with_guessing(
    target_position: Vec2,
    target_velocity: Vec2,
    target_acceleration: Vec2,
    my_position:Vec2,
    bullet_speed: f64,
    
) -> Vec2 {
    // calculate the predicted_position with the initial estimate of the time
    let mut predicted_position = kinematic_projectile_position(
        target_position,
        target_velocity,
        target_acceleration,
        my_position.distance(target_position) / bullet_speed,
    );

    let re_calc_amount = 10;
    for _number in (1..=re_calc_amount).rev() {
        // get the time it will take the bullet to travel over our position
        // to the predicted position
        let new_time = my_position.distance(predicted_position) / bullet_speed;

        predicted_position = kinematic_projectile_position(
            target_position,
            target_velocity,
            target_acceleration,
            new_time,
        );
    }

    predicted_position
}