
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



    fn when_will_two_objects_meet_without_acceleration(&self,position1: Vec2, velocity1: f64,position2: Vec2, velocity2: Vec2) -> f64 {
        // goal find t in 

        // p1 + v1*t + ½at² 
        // =
        // p2 + v2*t + ½at²

        // move t to the left 
        let velocity = velocity1 - velocity2;
        // move position aka non t to the right 
        let position = position2 - position1;
        // so you have v1-v2 = p2-p1


        // then (p2-p1)/(v1-v2) so you get T
        let magnitude = position.length();
        let time_to_collision = magnitude / velocity.length();
        
        time_to_collision
    }



    fn predict_target(&self,target_position:Vec2,target_velocity:Vec2,acceleration:Vec2,bullet_speed:f64) -> Vec2 {
        
        // get initial estimate of the time
        let time = self.when_will_two_objects_meet_without_acceleration(
            position(),
            bullet_speed,
            target_position,
            target_velocity,
        );
        
        // calculate the predicted_position with the initial estimate of the time
        let mut predicted_position = self.projectile_position(
            target_position, 
            target_velocity, 
            acceleration, 
            time
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
            // draw line to the refined position
            self.render_predicted_ship(predicted_position, (number*10) as f64);

        }
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

        let slowdown_angle = angular_velocity() * angular_velocity() / (2.0 * max_angular_acceleration());

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

        let bullet_speed: f64 = 1000.0/f64::sqrt(2.0);
        let predicted_position = self.predict_target(
            target(),
            target_velocity(),
            acceleration_of_target,
            bullet_speed,
        );

        let angle_difference = angle_diff(heading(), (predicted_position - position()).angle());
        
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

        self.render_predicted_ship(predicted_position,10.0);
        
        debug!("acceleration_of_target: {}", acceleration_of_target);
        debug!("----------");
        debug!("heading: {}", heading());
        debug!("angular_velocity: {}", angular_velocity());
        debug!("angle_difference: {}", angle_difference);
        

        
        

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
