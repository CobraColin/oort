use std::default;
use crate::{logic::acceleration_processing::take_direction_into_acount, math::{target_prediction::predict_target_with_guessing, smoothing_functions::sma_filter}};
use oort_api::prelude::{Vec2, TICK_LENGTH, vec2, position, current_tick};


pub struct TargetTickInformation {
    pub position: Vec2,
    pub velocity: Vec2,
    pub predicted_position: Vec2,
}


pub fn make_data_entry(position: Option<Vec2>,velocity: Option<Vec2>,predicted_position: Option<Vec2>) -> TargetTickInformation {
    let target_tick_information_object = TargetTickInformation {
        position: position.unwrap_or(Default::default()),
        velocity: velocity.unwrap_or(Default::default()),
        predicted_position: predicted_position.unwrap_or(Default::default()),
    };
    target_tick_information_object
}



pub fn calculate_average_acceleration_of_target(target:&Target,current_target_velocity:Vec2,my_current_velocity:Vec2) -> Vec2 {
    let velocities: Vec<Vec2> = get_velocities(&target);
    // if there isn't a sufficient amount of velocities to take the avarage then return 
    // acceleration of 0
    if velocities.len() <= 1 {  
        return Default::default()
    }
    let mut total: Vec<Vec2> = vec![];
    let vec_length = velocities.len();

    let current_acceleration = (current_target_velocity - my_current_velocity - velocities[0]) / TICK_LENGTH;
    
    for i in (0..vec_length).rev() {
        let current_velocity = velocities[i];

        if i == 0 {
            total.push(current_acceleration);

            return take_direction_into_acount(total);
        }

        let acceleration = (velocities[i - 1] - current_velocity) / TICK_LENGTH;

        total.push(acceleration);
    }
    Default::default()
}

pub fn predict_target(target:&Target,target_position:Vec2,current_target_velocity:Vec2,acceleration_of_target:Vec2,my_position:Vec2,my_current_velocity:Vec2,bullet_speed:f64) -> Vec2 {
    let mut past_target_predictions = get_predicted_positions(&target);
    
    let mut predicted_position = predict_target_with_guessing(
        target_position,
        current_target_velocity - my_current_velocity,
        acceleration_of_target,
        my_position,
        bullet_speed,
    );

    past_target_predictions.push(predicted_position);
    if past_target_predictions.len() > 11 {
        predicted_position = *sma_filter(
            &past_target_predictions.split_off(past_target_predictions.len() - 10),
            2
            )
            .last()
            .unwrap(); 
        
    }
    
    return predicted_position
}