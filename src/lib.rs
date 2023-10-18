mod math;
mod logic;
use math::target_prediction::*;
use math::general::*;
use math::smoothing_functions::sma_filter;

mod misc;
use misc::colors::*;
use misc::drawing::draw_a_line_from_a_vec;

use misc::information_storage::InformationBank;
use oort_api::prelude::{maths_rs::vec, *};


pub struct Ship {
    color_palette: ColorPalette,
    information: InformationBank,
}



impl Ship {
    pub fn new() -> Ship {
        Ship {
            color_palette: ColorPalette::new(),
            information: InformationBank::new(),
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




    pub fn tick(&mut self) {
        
        if current_tick() == 0 {
            let current_target = self.information.make_target(target());
            current_target.make_data_entry(Some(target()), Some(target_velocity()), None);
        }

        let current_target = self.information.get_id_last_target_making();
        let current_target = self.information.targets.get_mut(current_target).unwrap();

        let bullet_speed: f64 = 1000.0;
        let acceleration_of_target = current_target.calculate_average_acceleration_of_target(target_velocity(), velocity());




        if true {
        draw_a_line_from_a_vec(&vec![], self.get_color(ColorName::Red),400);
        }



        debug!("----------");
        debug!(
            "av_acceleration_of_target: {}",
            acceleration_of_target.length()
        );
        // debug!(
        //     "acceleration_of_target   : {}",
        //     ((target_velocity() - velocity() - (self.last_target_velocity)) / TICK_LENGTH).length()
        // );
        
        debug!("speed_of_target          : {}", target_velocity().length());
        debug!("distance to target       : {}",position().distance(target()));
        debug!("----------");
        


        
        // debug!("simplified lines: {}",);

        // let angle_difference = angle_diff(heading(), (predicted_position - position()).angle());

        // // calculate a vec2 that goes from our headings and has the length of our position to predicted_position
        // let endpoint = calculate_vector_at_end_of_line_heading_in_a_direction(position(),position().distance(predicted_position),heading());

        // // draw lines from our position to target and
        // draw_line(position(), target(), self.get_color(ColorName::Red));
        // draw_line(position(), endpoint, self.get_color(ColorName::Green));

        // draw_line(
        //     predicted_position,
        //     target(),
        //     self.get_color(ColorName::MediumPurple),
        // );
        // draw_line(
        //     position(),
        //     predicted_position,
        //     self.get_color(ColorName::Blue),
        // );

        // draw_triangle(predicted_position, 10.0, self.get_color(ColorName::DarkRed));

        // if angle_difference > -0.005 && angle_difference < 0.005 {
        //     // if current_tick() > 900 {
        //     fire(0);
        //     // }
        // }

        // self.look_at(angle_difference, predicted_position, endpoint);
 

 
        // if position().distance(target()) < 1000.0 {
        //     accelerate(predicted_position-position());
        // } else if position().distance(target()) > 1000.0 && position().distance(target()) < 1100.0 {
        //     accelerate(vec2(0., 0.))

        // } else if position().distance(target()) > 1100.0 {
        //     accelerate(predicted_position-position());
        // }

        // self.last_time = current_time();
        // self.last_target_velocities.insert(0, target_velocity() - velocity());

        // self.last_target_velocity = target_velocity() - velocity();
        // let indexs = 60;
        // if indexs < self.last_target_velocities.len() {
        //     self.last_target_velocities.remove(indexs);
        // }

        // let current_tick_information = tick_information {
        //     target_position: target(),
        //     target_velocity: target_velocity(),
        //     predicted_target_position: predicted_position,
        //     my_position: position(),
        //     my_velocity: velocity(),
        // };

        // self.information_storage.push(current_tick_information);
        current_target.make_data_entry(Some(target()), Some(target_velocity()), None);
    }
}
















