use oort_api::prelude::*;
fn main() {
    println!("")
}

pub struct Ship {}

impl Ship {
    pub fn new() -> Ship {
        Ship {}
    }

    fn get_distance(&mut self,v2:Vec2) -> f64 {
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
    
    

    pub fn tick(&mut self) {
        // if self.get_distance(target()) > 500.0 {
        //     accelerate(target()-position())
        // }
        
        let bullet_time: f64 = 1.0;



        let time = self.when_will_two_objects_meet_without_acceleration(
            position(),
            1000.0,
            target(),
            target_velocity(),
        );

        let final_position = self.projectile_position(
            target(), 
            target_velocity(), 
            vec2(0.0, 0.0), 
            time
            );
        // let position = Vec2::new(10.0, 20.0); // Create a Vec2<f64> with the correct types
        let color = 0xFF0000; // Assuming the color is represented as a u32
        let heading_color = 0x44FF00;
    
        // draw_text!(position, color, "Hello, World!");
        

        let angle_difference = angle_diff(heading(), (final_position - position()).angle());

        draw_line(position(), target(), color);

        draw_line(position(), final_position - position(), color);
        let line_length = 1000.0; // Length of the line in units
        let endpoint_x = position().x + line_length * heading().cos();
        let endpoint_y = position().y + line_length * heading().sin();
        let endpoint = Vec2::new(endpoint_x, endpoint_y);
    
        // Now, you can draw the line from your current position to the calculated endpoint

        draw_line(position(), endpoint, heading_color);



        debug!("time: {}\nfinal_position{}",time,angle_difference);
        turn(angle_difference);
        // if angle_difference < 0.0745329 && angle_difference > -0.0745329 {
        //     fire(0)
        // } 
        fire(0)

        
    }
}





