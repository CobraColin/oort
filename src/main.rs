use oort_api::prelude::*;

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
    
    

    pub fn tick(&mut self) {
        if self.get_distance(target()) > 500.0 {
            accelerate(target()-position())
        }
        

        let final_position = self.projectile_position(target(), target_velocity(), vec2(1.0, 1.0), 0.4);
        let angle_difference = angle_diff(heading(), (final_position - position()).angle());
        turn(angle_difference);
        if angle_difference < 0.0745329 && angle_difference > -0.0745329 {
            fire(0)
        } 
        

        
    }
}

fn main() {
    println!("")
}




