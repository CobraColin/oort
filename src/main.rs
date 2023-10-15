// Tutorial: Lead
// Destroy the enemy ship. Its position is given by the "target" function and velocity by the
// "target_velocity" function. Your ship is not able to accelerate in this scenario.
//
// This is where the game becomes challenging! You'll need to lead the target
// by firing towards where the target will be by the time the bullet gets there.
//
// Hint: target() + target_velocity() * t gives the position of the target after t seconds.
//
// You can scale a vector by a number: vec2(a, b) * c == vec2(a * c, b * c)

use oort_api::prelude::{maths_rs::vec, *};

#[derive(Clone)]
enum ColorName {
    Red,
    Green,
    Blue,
    Orange,
    Yellow,
    Purple,
    Pink,
    Brown,
    Cyan,
    Magenta,
    Teal,
    Lavender,
    Maroon,
    Olive,
    Coral,
    Indigo,
    Turquoise,
    DarkSlateGray,
    DarkMagenta,
    Gold,
    SeaGreen,
    Tomato,
    RoyalBlue,
    DarkOrange,
    Lime,
    DarkViolet,
    DarkGreen,
    DarkOliveGreen,
    DarkCyan,
    DarkGoldenrod,
    DarkSlateBlue,
    DarkRed,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    MediumAquamarine,
    MediumOrchid,
    MediumPurple,
    MediumSlateBlue,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    SlateBlue,
    SandyBrown,
    Silver,
    White,
}

struct ColorPalette {
    colors: [u32; 50],
}

impl ColorPalette {
    fn new() -> ColorPalette {
        let mut colors = [0; 50];

        colors[ColorName::Red as usize] = 0xFF0000;
        colors[ColorName::Green as usize] = 0x00FF00;
        colors[ColorName::Blue as usize] = 0x0000FF;
        colors[ColorName::Orange as usize] = 0xFFA500;
        colors[ColorName::Yellow as usize] = 0xFFFF00;
        colors[ColorName::Purple as usize] = 0x800080;
        colors[ColorName::Pink as usize] = 0xFFC0CB;
        colors[ColorName::Brown as usize] = 0xA52A2A;
        colors[ColorName::Cyan as usize] = 0x00FFFF;
        colors[ColorName::Magenta as usize] = 0xFF00FF;
        colors[ColorName::Teal as usize] = 0x008080;
        colors[ColorName::Lavender as usize] = 0xE6E6FA;
        colors[ColorName::Maroon as usize] = 0x800000;
        colors[ColorName::Olive as usize] = 0x808000;
        colors[ColorName::Coral as usize] = 0xFF6F61;
        colors[ColorName::Indigo as usize] = 0x4B0082;
        colors[ColorName::Turquoise as usize] = 0x40E0D0;
        colors[ColorName::DarkSlateGray as usize] = 0x2F4F4F;
        colors[ColorName::DarkMagenta as usize] = 0x8B008B;
        colors[ColorName::Gold as usize] = 0xFFD700;
        colors[ColorName::SeaGreen as usize] = 0x2E8B57;
        colors[ColorName::Tomato as usize] = 0xFF6347;
        colors[ColorName::RoyalBlue as usize] = 0x4169E1;
        colors[ColorName::DarkOrange as usize] = 0xFF8C00;
        colors[ColorName::Lime as usize] = 0x00FF00;
        colors[ColorName::DarkViolet as usize] = 0x9400D3;
        colors[ColorName::DarkGreen as usize] = 0x006400;
        colors[ColorName::DarkOliveGreen as usize] = 0x556B2F;
        colors[ColorName::DarkCyan as usize] = 0x008B8B;
        colors[ColorName::DarkGoldenrod as usize] = 0xB8860B;
        colors[ColorName::DarkSlateBlue as usize] = 0x483D8B;
        colors[ColorName::DarkRed as usize] = 0x8B0000;
        colors[ColorName::LightPink as usize] = 0xFFB6C1;
        colors[ColorName::LightSalmon as usize] = 0xFFA07A;
        colors[ColorName::LightSeaGreen as usize] = 0x20B2AA;
        colors[ColorName::LightSkyBlue as usize] = 0x87CEFA;
        colors[ColorName::LightSlateGray as usize] = 0x778899;
        colors[ColorName::LightSteelBlue as usize] = 0xB0C4DE;
        colors[ColorName::MediumAquamarine as usize] = 0x66CDAA;
        colors[ColorName::MediumOrchid as usize] = 0xBA55D3;
        colors[ColorName::MediumPurple as usize] = 0x9370DB;
        colors[ColorName::MediumSlateBlue as usize] = 0x7B68EE;
        colors[ColorName::MediumTurquoise as usize] = 0x48D1CC;
        colors[ColorName::MediumVioletRed as usize] = 0xC71585;
        colors[ColorName::MidnightBlue as usize] = 0x191970;
        colors[ColorName::SlateBlue as usize] = 0x6A5ACD;
        colors[ColorName::SandyBrown as usize] = 0xF4A460;
        colors[ColorName::Silver as usize] = 0xC0C0C0;
        colors[ColorName::White as usize] = 0xFFFFFF;

        ColorPalette { colors }
    }
}

pub struct Ship {
    last_target_velocity: Vec2,
    last_time: f64,
    color_palette: ColorPalette,
    last_target_velocities: Vec<Vec2>,
}

impl Ship {
    pub fn new() -> Ship {
        Ship {
            last_target_velocity: vec2(0.0, 0.0),
            last_time: 0.0,
            color_palette: ColorPalette::new(),
            last_target_velocities: vec![],
        }
    }
    fn get_color(&self, color_name: ColorName) -> u32 {
        self.color_palette.colors[color_name as usize]
    }

    fn get_distance(&self, v2: Vec2) -> f64 {
        let v1 = position();
        let dx = v1.x - v2.x;
        let dy = v1.y - v2.y;

        (dx * dx + dy * dy).sqrt()
    }

    fn when_will_two_objects_meet_without_acceleration(
        &self,
        non_relative_position: Vec2,
        velocity1: Vec2,
    ) -> f64 {
        // goal find t in

        let position1 = non_relative_position - position();

        // p1 + v1*t = p2 + v2*t

        let dot_product_pos1_and_vel1 = position1.dot(velocity1);

        let speed_squared: f64 = velocity1.length().powi(2);

        let discriminant = dot_product_pos1_and_vel1.powi(2)
            + position1.length().powi(2) * (1_000_000.0 - speed_squared);

        if discriminant > 0.0 {
            let down_stairs = 1_000_000.0 - speed_squared;

            if down_stairs != 0.0 {
                let up_upstares_plus = dot_product_pos1_and_vel1 + discriminant.sqrt();
                let up_upstares_minus = dot_product_pos1_and_vel1 - discriminant.sqrt();

                let plus_version = up_upstares_plus / down_stairs;
                let minus_version = up_upstares_minus / down_stairs;
                if plus_version > minus_version {
                    return plus_version;
                } else {
                    return minus_version;
                }
            }
        }

        0.0
    }

    fn predict_target_with_guessing(
        &self,
        target_position: Vec2,
        target_velocity: Vec2,
        acceleration: Vec2,
        bullet_speed: f64,
    ) -> Vec2 {
        // calculate the predicted_position with the initial estimate of the time
        let mut predicted_position = kinematic_projectile_position(
            target_position,
            target_velocity,
            acceleration,
            self.get_distance(target()) / bullet_speed,
        );

        let re_calc_amount = 50;
        for number in (1..=re_calc_amount).rev() {
            // get the time it will take the bullet to travel over our position
            // to the predicted position
            let new_time = self.get_distance(predicted_position) / bullet_speed;

            predicted_position = kinematic_projectile_position(
                target_position,
                target_velocity,
                acceleration,
                new_time,
            );

            // if number == re_calc_amount {
            //     self.render_predicted_ship(predicted_position, 10.0,self.get_color(ColorName::MediumVioletRed));
            // }
        }

        predicted_position
    }

    fn predict_target_in_one_go(
        &self,
        target_position: Vec2,
        target_velocity: Vec2,
        acceleration: Vec2,
        _bullet_speed: f64,
    ) -> Vec2 {
        // predict the time
        let time =
            self.when_will_two_objects_meet_without_acceleration(target_position, target_velocity);

        // calculate the predicted_position with the initial estimate of the time
        let predicted_position =
            kinematic_projectile_position(target_position, target_velocity, acceleration, time);

        predicted_position
    }

    fn calculate_endpoint(&self, predicted_position: Vec2) -> Vec2 {
        let line_length = self.get_distance(predicted_position); // Length of the line in units
        let endpoint_x = position().x + line_length * heading().cos();
        let endpoint_y = position().y + line_length * heading().sin();
        Vec2::new(endpoint_x, endpoint_y)
    }

    fn render_predicted_ship(&self, ship: Vec2, radius: f64, color: u32) {
        draw_triangle(ship, radius, color)
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
                            - 0.05 * get_distance(final_position, endpoint)
                                / (position().distance(final_position) / 1000.),
                    );
                } else {
                    turn(
                        angle
                            + 0.05 * get_distance(final_position, endpoint)
                                / (position().distance(final_position) / 1000.),
                    );
                }
            } else {
                turn(angle);
            }
        }
    }

    fn calculate_average_acceleration_of_target(&self) -> Vec2 {
        let mut total = vec2(0.0, 0.0);
        let vec_length = self.last_target_velocities.len();
        for i in (0..vec_length).rev() {
            let old_velocity = self.last_target_velocities[i];

            if i == 0 {
                total += (target_velocity() - velocity() - old_velocity) / TICK_LENGTH;
                debug!("a{}", total / vec_length as f64);
                return total / vec_length as f64;
            }
            total += (self.last_target_velocities[i - 1] - old_velocity) / TICK_LENGTH
        }
        vec2(0.0, 0.0)
    }

    fn get_acceleration(&self, target_position: Vec2, my_pos: Vec2) -> Vec2 {
        let mut acceleration = target_position - my_pos;
        acceleration = acceleration.rotate(-heading());
        if acceleration.x > max_forward_acceleration() {
            acceleration *= max_forward_acceleration() / acceleration.x;
        }
        if acceleration.x < -max_backward_acceleration() {
            acceleration *= max_backward_acceleration() / -acceleration.x;
        }
        if acceleration.y.abs() > max_lateral_acceleration() {
            acceleration *= max_lateral_acceleration() / acceleration.y.abs();
        }

        return acceleration;
    }

    fn predict_the_future(
        &self,
        f_target_position: Vec2,
        f_target_velocity: Vec2,
        f_target_acceleration: Vec2,
    ) {
        let mut my_position = position();
        let mut my_velocity = velocity();

        let mut my_acceleration = self.get_acceleration(
            self.predict_target_with_guessing(
                target(),
                target_velocity() - velocity(),
                f_target_acceleration,
                1000.,
            ),
            position(),
        );

        let mut f_target_position = f_target_position;
        let mut f_target_velocity = f_target_velocity;
        let mut f_target_acceleration = f_target_acceleration;

        let future_predicted_positions: Vec<vec::Vec2<f64>> = vec![];

        // loops 5 times
        for i in 1..6 {
            let i = i as f64;
            my_position = kinematic_projectile_position(my_position, my_velocity, my_acceleration, TICK_LENGTH);
            my_velocity = my_velocity+my_acceleration*TICK_LENGTH;

            
            f_target_position = kinematic_projectile_position(f_target_position, f_target_velocity, my_acceleration, TICK_LENGTH);
            f_target_velocity = f_target_velocity+f_target_acceleration*TICK_LENGTH;

            self.render_predicted_ship(my_position, 10., self.get_color(ColorName::Pink));


            self.render_predicted_ship(f_target_position, 10., self.get_color(ColorName::DarkGreen));
        }
        }

    pub fn tick(&mut self) {
        let mut acceleration_of_target = vec2(0.0, 0.0);
        if self.last_target_velocities.len() > 0 {
            acceleration_of_target = self.calculate_average_acceleration_of_target()
            // acceleration_of_target = (target_velocity()-velocity()-(self.last_target_velocity))/TICK_LENGTH
        }
        self.predict_the_future(target(),target_velocity(),acceleration_of_target);
        let bullet_speed: f64 = 1000.0;
        let mut predicted_position = self.predict_target_in_one_go(
            target(),
            target_velocity(),
            vec2(0.0, 0.0),
            bullet_speed,
        );

        let mut gokken_predicted_position = self.predict_target_with_guessing(
            target(),
            target_velocity() - velocity(),
            acceleration_of_target,
            bullet_speed,
        );

        debug!("target_velocity: {}", target_velocity().length());
        debug!("----------");
        debug!("no guessing: {}", predicted_position);
        debug!("guessing     : {}", gokken_predicted_position);

        let switch = true;
        if switch {
            std::mem::swap(&mut predicted_position, &mut gokken_predicted_position);
        }

        let angle_difference = angle_diff(heading(), (predicted_position - position()).angle());

        // calculate a vec2 that goes from our headings and has the length of our position to predicted_position
        let endpoint = self.calculate_endpoint(predicted_position);

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

        self.render_predicted_ship(predicted_position, 10.0, self.get_color(ColorName::DarkRed));

        if angle_difference > -0.005 && angle_difference < 0.005 {
            // fire(0);
        }

        self.look_at(angle_difference, predicted_position, endpoint);

        debug!("acceleration: {}", predicted_position - position());
        accelerate(self.get_acceleration(predicted_position, position()));

        self.last_time = current_time();
        self.last_target_velocities
            .insert(0, target_velocity() - velocity());
        let indexs = 5;
        if indexs < self.last_target_velocities.len() {
            self.last_target_velocities.remove(indexs);
        }
    }
}

fn get_distance(v1: Vec2, v2: Vec2) -> f64 {
    let dx = v1.x - v2.x;
    let dy = v1.y - v2.y;

    (dx * dx + dy * dy).sqrt()
}

fn make_relative_to_origin(origin: &Vec2, relative_point: &Vec2) -> Vec2 {
    Vec2 {
        x: relative_point.x - origin.x,
        y: relative_point.y - origin.y,
    }
}
fn kinematic_projectile_position(
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
fn main() {}

//bullets inherit the velocity of the vehicle.
// right now 4.047 for both
// gokken lower if steps are 3

/* er is ook nog een inaccuracy
gun.inaccuracy = 0.25
let relative_heading = if gun.inaccuracy > 0.0 {
    relative_heading + rng.gen_range(-gun.inaccuracy..gun.inaccuracy)
} else {
    relative_heading
};
*/
