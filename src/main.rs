use oort_api::prelude::*;
fn main() {
    println!("")
}

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    fn get_distance(&self,v2:Vec2) -> f64 {
        let v1 = position();
        let dx = v1.x - v2.x;
        let dy = v1.y - v2.y;
    
        (dx * dx + dy * dy).sqrt()
    }

    fn projectile_position(&self,position: Vec2, velocity: Vec2, acceleration: Vec2, time: f64) -> Vec2 {
        let final_position = position + velocity * time + 0.5 * acceleration * time.powi(2);
        final_position
    }



    fn when_will_two_objects_meet_without_acceleration(&self,position1: Vec2, velocity1: f64,position2: Vec2, velocity2: Vec2) -> f64 {
        let velocity = velocity1 + velocity2*-1.0;
        let position = position2 + position1*-1.0;


        let magnitude = position.length();
        let time_to_collision = magnitude / velocity.length();
    
        time_to_collision
    }

    fn predict_target(&self) -> Vec2 {
        let bullet_speed: f64 = 1000.0;
        let time = self.when_will_two_objects_meet_without_acceleration(
            position(),
            1000.0,
            target(),
            target_velocity(),
        );

        let mut final_position = self.projectile_position(
            target(), 
            target_velocity(), 
            vec2(0.0, 0.0), 
            time
        );


        let colors: Vec<u32> = vec![
            0xFF0000, // Red
            0x00FF00, // Green
            0x0000FF, // Blue
            0xFFFF00, // Yellow
            0x800080, // Purple
        ];

        for color in colors {
            draw_line(
                position(), 
                final_position - position(), 
                color);
            let new_time = self.get_distance(final_position)/1000.0;
            final_position = self.projectile_position(
                target(), 
                target_velocity(), 
                vec2(0.0, 0.0), 
                new_time
            );

        }
        final_position
    }
    
    fn calculate_endpoint(
        &self, 
        final_position: Vec2,
    ) -> Vec2 {
        let line_length = self.get_distance(final_position); // Length of the line in units
        let endpoint_x = position().x + line_length * heading().cos();
        let endpoint_y = position().y + line_length * heading().sin();
        Vec2::new(endpoint_x, endpoint_y)
    }
    

    pub fn tick(&mut self) {

        let color = 0xFF0000; 
        let heading_color = 0x44FF00;

        let final_position = self.predict_target();

        let angle_difference = angle_diff(heading(), (final_position - position()).angle());

        let endpoint = self.calculate_endpoint(final_position);

        draw_line(
            position(), 
            target(), 
            color);
        draw_line(
            position(), 
            endpoint,
            heading_color);

        
        


        debug!("angular_velocity: {}", angular_velocity());
        debug!("angle_difference: {}", angle_difference);
        // debug!("pred_t: {}", time);
        // debug!("pred_dist/bullet_speed: {}", pred_dist_devide_by_bullet_speed);

        
        

        if (angle_difference > -0.2 && angle_difference < 0.2) && (angular_velocity() > -0.6 && angular_velocity() < 0.6){
            if angle_difference < 0.0 {
                turn(angle_difference-0.08*get_distance(final_position,endpoint));
            } else {
                turn(angle_difference+0.08*get_distance(final_position,endpoint));
            }

        } else {
            turn(angle_difference)
        }
        
        
        fire(0)
        

        
    }
}




fn get_distance(v1:Vec2,v2:Vec2) -> f64 {
    let dx = v1.x - v2.x;
    let dy = v1.y - v2.y;

    (dx * dx + dy * dy).sqrt()
}


