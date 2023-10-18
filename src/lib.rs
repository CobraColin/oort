mod math;
use math::target_prediction::*;
use math::general::*;
use math::smoothing_functions::sma_filter;

mod misc;
use misc::colors::*;

use oort_api::prelude::{maths_rs::vec, *};


pub struct Ship {
    last_target_velocity: Vec2,
    last_time: f64,
    color_palette: ColorPalette,
    last_target_velocities: Vec<Vec2>,
    predicted_target_positions: Vec<Vec2>,
    predicted_target_positions_for_drawing: Vec<Vec2>,
    target_positions: Vec<Vec2>,
    information_storage: Vec<tick_information>,
}

struct tick_information {
    target_position: Vec2,
    target_velocity: Vec2,
    predicted_target_position: Vec2,
    my_position: Vec2,
    my_velocity: Vec2,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            last_target_velocity: vec2(0.0, 0.0),
            last_time: 0.0,
            color_palette: ColorPalette::new(),
            last_target_velocities: vec![],
            predicted_target_positions: vec![],
            predicted_target_positions_for_drawing: vec![],
            target_positions: vec![],
            information_storage: vec![],
        }
    }
    fn get_color(&self, color_name: ColorName) -> u32 {
        self.color_palette.colors[color_name as usize]
    }








    fn look_at(&mut self, angle: f64, final_position: Vec2, endpoint: Vec2) {
        // give angle difference

        let slowdown_angle = angular_velocity().powi(2) / (2.0 * max_angular_acceleration());

        if angle > slowdown_angle {
            torque(max_angular_acceleration());
        } else if angle < -slowdown_angle {
            torque(-max_angular_acceleration());
        } else {
            if (angle > -0.2 && angle < 0.2)
                && (angular_velocity() > -0.6 && angular_velocity() < 0.6)
            {
                if angle < 0.0 {
                    turn(
                        angle
                            - 0.05 * final_position.distance(endpoint)
                                / (position().distance(final_position) / 100.),
                    );
                } else {
                    turn(
                        angle
                            + 0.05 * final_position.distance(endpoint)
                                / (position().distance(final_position) / 100.),
                    );
                    
                }
            } else {
                turn(angle);
            }
        }
    }

    fn calculate_average_acceleration_of_target(&self) -> Vec2 {
        let velocities: Vec<vec::Vec2<f64>> = self.last_target_velocities.clone();
        let mut total: Vec<vec::Vec2<f64>> = vec![];
        let vec_length = velocities.len();

        let current_acceleration = (target_velocity() - velocity() - velocities[0]) / TICK_LENGTH;
        
        for i in (0..vec_length).rev() {
            let current_velocity = velocities[i];

            if i == 0 {
                total.push(current_acceleration);
                // let smoothed_vec = sma_filter(&total, 0);

                return take_direction_into_acount(total);
            }

            let acceleration = (velocities[i - 1] - current_velocity) / TICK_LENGTH;
            // if !two_vecs_have_same_signs(acceleration, current_acceleration){
                // continue;
            // }
            total.push(acceleration);
        }
        vec2(0.0, 0.0)
    }


    pub fn tick(&mut self) {
        let mut acceleration_of_target = vec2(0.0, 0.0);
        if self.last_target_velocities.len() > 1 {
            acceleration_of_target = self.calculate_average_acceleration_of_target();
        }

        let bullet_speed: f64 = 1000.0;


        self.target_positions.push(target());
        
        draw_a_line_from_a_vec(&self.target_positions, self.get_color(ColorName::Green));
        

        let mut predicted_position = predict_target_with_guessing(
            target(),
            target_velocity() - velocity(),
            acceleration_of_target,
            position(),
            bullet_speed,
        );

        

        self.predicted_target_positions_for_drawing
            .push(predicted_position);
        
        draw_a_line_from_a_vec(&self.predicted_target_positions_for_drawing, self.get_color(ColorName::Red));


        self.predicted_target_positions.push(predicted_position);
        if self.predicted_target_positions.len() > 11 {
            predicted_position = *sma_filter(&self.predicted_target_positions.split_off(self.predicted_target_positions.len() - 10), 2)
                .last()
                .unwrap(); 
        }

        debug!("----------");
        debug!(
            "av_acceleration_of_target: {}",
            acceleration_of_target.length()
        );
        debug!(
            "acceleration_of_target   : {}",
            ((target_velocity() - velocity() - (self.last_target_velocity)) / TICK_LENGTH).length()
        );
        debug!("speed_of_target          : {}", target_velocity().length());
        debug!("distance to target       : {}",position().distance(target()));
        debug!("----------");
        


        
        // debug!("simplified lines: {}",);

        let angle_difference = angle_diff(heading(), (predicted_position - position()).angle());

        // calculate a vec2 that goes from our headings and has the length of our position to predicted_position
        let endpoint = calculate_vector_at_end_of_line_heading_in_a_direction(position(),position().distance(predicted_position),heading());

        // draw lines from our position to target and
        draw_line(position(), target(), self.get_color(ColorName::Red));
        draw_line(position(), endpoint, self.get_color(ColorName::Green));

        draw_line(
            predicted_position,
            target(),
            self.get_color(ColorName::MediumPurple),
        );
        draw_line(
            position(),
            predicted_position,
            self.get_color(ColorName::Blue),
        );

        draw_triangle(predicted_position, 10.0, self.get_color(ColorName::DarkRed));

        if angle_difference > -0.005 && angle_difference < 0.005 {
            // if current_tick() > 900 {
            // fire(0);
            // }
        }

        self.look_at(angle_difference, predicted_position, endpoint);
 

 
        // if position().distance(target()) < 1000.0 {
        //     accelerate(predicted_position-position());
        // } else if position().distance(target()) > 1000.0 && position().distance(target()) < 1100.0 {
        //     accelerate(vec2(0., 0.))

        // } else if position().distance(target()) > 1100.0 {
        //     accelerate(predicted_position-position());
        // }

        self.last_time = current_time();
        self.last_target_velocities.insert(0, target_velocity() - velocity());

        self.last_target_velocity = target_velocity() - velocity();
        let indexs = 60;
        if indexs < self.last_target_velocities.len() {
            self.last_target_velocities.remove(indexs);
        }

        let current_tick_information = tick_information {
            target_position: target(),
            target_velocity: target_velocity(),
            predicted_target_position: predicted_position,
            my_position: position(),
            my_velocity: velocity(),
        };

        self.information_storage.push(current_tick_information);
    }
}









fn draw_a_line_from_a_vec(vec_to_draw:&Vec<Vec2>,color:u32) {
    for i in 0..vec_to_draw.len() {
        if vec_to_draw.len() > 400 {
            if i <= vec_to_draw.len() - 400 {
                continue;
            }
        }

        if vec_to_draw.get(i + 1).is_none() {
            break;
        }

        let pred_from = vec_to_draw[i].clone();
        let pred_to = vec_to_draw[i + 1].clone();

        draw_line(pred_from, pred_to, color)
    }
}



fn two_vecs_have_same_signs(compare1:Vec2,compare2:Vec2) -> bool {
    if (compare1.x.is_sign_negative() && compare2.x.is_sign_positive()) || ((compare1.y.is_sign_negative() && compare2.y.is_sign_positive())) {
        return false;
    }
    return true;
}

fn get_until_signs_are_not_same(vec_of_a:&Vec<Vec2>) -> usize {
    //returns false if the vec is empty
    if vec_of_a.len() == 0 {
        return 0
    }
    let compare_to = *vec_of_a.last().unwrap();
    for i in (0..vec_of_a.len()-1).rev() {
        if !two_vecs_have_same_signs(compare_to,vec_of_a[i]) {
            if i == 0 {
                return 0
            }
            return i
        }
    }

    return vec_of_a.len()
}

fn get_change_in_percentage_between_two_acceleration_vec2(older: &Vec2, new: &Vec2) -> f64 {
    let magnitudeolder = older.length();
    let magnitudenew = new.length();

    if magnitudeolder == 0.0 {
        return 100.0; // If the initial value is 0, we'll consider the change to be 100%
    }

    let change = magnitudenew - magnitudeolder;
    let percentage_change = (change / magnitudeolder) * 100.0;

    percentage_change.abs()
}
fn get_until_acceleration_changes_too_much(vec_of_a:&Vec<Vec2>,percentage:f64) -> usize {
    //returns false if the vec is empty
    if vec_of_a.len() == 0 {
        return 0
    }

    let compare_to = *vec_of_a.last().unwrap();
    for i in (0..vec_of_a.len()-1).rev() {
        let change_percentage = 
        get_change_in_percentage_between_two_acceleration_vec2(
            &vec_of_a[i],
            &compare_to
        );
        // debug!("{}",change_percentage);
        if change_percentage > percentage {
            if i == 0 {
                return 0
            }
            return i
        }
    }

    return vec_of_a.len()
}



fn take_direction_into_acount(mut accelerations:Vec<Vec2>) -> Vec2 {
    let mut vec_length = get_until_signs_are_not_same(&accelerations);
    


    while  accelerations.len() > 10 && vec_length < 10 {
        accelerations.remove(accelerations.len()-1);
        vec_length = get_until_signs_are_not_same(&accelerations);
    }
    debug!("{}",vec_length);
    
    if accelerations.len() > 10 {
        let mut new_vec = accelerations.clone().split_off(accelerations.len()-vec_length);
    
        // let new_vec_length_for_acceleration_change = get_until_acceleration_changes_too_much(
        //     &new_vec,
        //     3.
        // );
    
        // debug!("new_vec_length_for_acceleration_change{}",new_vec_length_for_acceleration_change);
    
        
    
    
        // if new_vec.len() > 5 && new_vec_length_for_acceleration_change > 5 {
        if new_vec.len() > 10 {
            // new_vec = new_vec.clone().split_off(new_vec.len()-new_vec_length_for_acceleration_change);
            return calculate_average(&sma_filter(&new_vec, 2)).unwrap();
        }
    }

    return calculate_average(&sma_filter(&accelerations, 2)).unwrap();
}


