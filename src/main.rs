
use std::mem::Discriminant;

use oort_api::prelude::*;

pub struct Ship {
    last_target_position:Vec2,
    last_time:f64,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            last_target_position: vec2(0.0,0.0),
            last_time:0.0,
        }
    }

    fn get_distance(&self,v2:Vec2) -> f64 {
        let v1 = position();
        let dx = v1.x - v2.x;
        let dy = v1.y - v2.y;
    
        (dx * dx + dy * dy).sqrt()
    }

    fn projectile_position(&self,position: Vec2, velocity: Vec2, acceleration: Vec2, time: f64) -> Vec2 {
        // calculates p = p₀ + v₀t + ½at²
        let predicted_position = position + velocity * time + 0.5 * acceleration * time.powi(2);
        // return p
        predicted_position
    }

    

    fn when_will_two_objects_meet_without_acceleration(&self,position1: Vec2, velocity1: Vec2) -> f64 {
        // goal find t in 

        // p1 + v1*t = p2 + v2*t

        let dot_product_pos1_and_vel1 = position1.dot(velocity1);

        let speed_squared: f64 = velocity1.length().powi(2);

        let discriminant = dot_product_pos1_and_vel1.powi(2) + position1.length().powi(2) * (1_000_000.0 - speed_squared);
        
        if discriminant > 0.0 {
            let up_upstares = dot_product_pos1_and_vel1 + discriminant.sqrt();
            let down_stairs = 1_000_000.0 - speed_squared;

            if down_stairs != 0.0 {
                let plus_version = up_upstares/down_stairs;
                debug!("plus_version: {}", plus_version);
                return plus_version
            } 
        }

        

        0.0
    }



    fn predict_target_met_gokken(&self,target_position:Vec2,target_velocity:Vec2,acceleration:Vec2,bullet_speed:f64) -> Vec2 {
        
        // get initial estimate of the time

        
        // calculate the predicted_position with the initial estimate of the time
        let mut predicted_position = self.projectile_position(
            target_position, 
            target_velocity, 
            acceleration, 
             self.get_distance(target())/1000.0
        );

        // draw line to initial estimated predicted position
        self.render_predicted_ship(predicted_position, (6*10) as f64);



        let re_calc_amount = 3;
        for number in (1..=re_calc_amount).rev() {
            // get the time it will take the bullet to travel over our position
            // to the predicted position
            let new_time = self.get_distance(predicted_position)/1000.0; 

            predicted_position = self.projectile_position(
                target_position, 
                target_velocity, 
                acceleration, 
                new_time
            );

            if number == 3 {
                debug!("gokken: {}", new_time);
            }
     

        }
        predicted_position
    }
    
    fn predict_target_in_one_go(&self,target_position:Vec2,target_velocity:Vec2,acceleration:Vec2,bullet_speed:f64) -> Vec2 {
        
        // predict the time
        let time = self.when_will_two_objects_meet_without_acceleration(target_position, target_velocity);

        
        // calculate the predicted_position with the initial estimate of the time
        let predicted_position = self.projectile_position(
            target_position, 
            target_velocity, 
            acceleration, 
            time
        );

        predicted_position
    }

    fn calculate_endpoint(
        &self, 
        predicted_position: Vec2,
    ) -> Vec2 {
        let line_length = self.get_distance(predicted_position); // Length of the line in units
        let endpoint_x = position().x + line_length * heading().cos();
        let endpoint_y = position().y + line_length * heading().sin();
        Vec2::new(endpoint_x, endpoint_y)
    }
    fn render_predicted_ship(&self,ship:Vec2,radius:f64) {
        let colors: Vec<u32> = vec![
            0xFF0000, // Red
            0x00FF00, // Green
            0x0000FF, // Blue
            0xFFFF00, // Yellow
            0x800080, // Purple
        ];
        draw_triangle(ship, radius,colors[4])
    } 

    fn look_at(&mut self, angle: f64) {
        // give angle difference  

        let slowdown_angle = angular_velocity().powi(2) / (2.0 * max_angular_acceleration());

        if angle > slowdown_angle {
            torque(max_angular_acceleration());
        } else if angle < -slowdown_angle {
            torque(-max_angular_acceleration());
        } else {
            turn(angle);
        }
    }

    pub fn tick(&mut self) {
        let mut acceleration_of_target = vec2(0.0, 0.0);
        if self.last_target_position != vec2(0.0, 0.0) {
            acceleration_of_target = target()-(self.last_target_position)
        }

        let color = 0xFF0000; 
        let heading_color = 0x44FF00;

        let bullet_speed: f64 = 1000.0;
        let predicted_position: maths_rs::vec::Vec2<f64> = self.predict_target_in_one_go(
            target(),
            target_velocity(),
            acceleration_of_target,
            bullet_speed,
        );


        let gokken_predicted_position = self.predict_target_met_gokken(
            target(),
            target_velocity(),
            acceleration_of_target,
            bullet_speed,
        );

        let angle_difference = angle_diff(
            heading(),
            (
                predicted_position - position()
            ).angle()
            );
        
        // calculate a vec2 that goes from our headings and has the length of our position to predicted_position
        let endpoint = self.calculate_endpoint(predicted_position);


        // draw lines from our position to target and 
        draw_line(
            position(), 
            target(), 
            color
        );
        draw_line(
            position(), 
            endpoint,
            heading_color
        );

        draw_line(
            predicted_position, 
            target(),
            0x800080
        );
        draw_line(
            position(), 
            predicted_position,
            0x0000FF
        );

        self.render_predicted_ship(predicted_position,10.0);
        
        debug!("angular_velocity: {}", angular_velocity());
        debug!("----------");
        // debug!("heading: {}", heading());
        debug!("gokken     : {}", gokken_predicted_position);
        debug!("niet gokken: {}", predicted_position);
        

        
        

        if angle_difference > -0.005 && angle_difference < 0.005{
            fire(0);
        }

        self.look_at(angle_difference);
        
        
        
        
        self.last_time = current_time();
        self.last_target_position = target();
    }
}




fn get_distance(v1:Vec2,v2:Vec2) -> f64 {
    let dx = v1.x - v2.x;
    let dy = v1.y - v2.y;

    (dx * dx + dy * dy).sqrt()
}

fn main() {

}
