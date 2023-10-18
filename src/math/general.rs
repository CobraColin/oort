use oort_api::prelude::*;

/// Calculates the coordinates of a vector's endpoint at the given line length and the heading of the line.
pub fn calculate_vector_at_end_of_line_heading_in_a_direction(from:Vec2,line_length: f64,heading:f64) -> Vec2 {
    let endpoint_x = from.x + line_length * heading.cos();
    let endpoint_y = from.y + line_length * heading.sin();
    Vec2::new(endpoint_x, endpoint_y)
}

pub fn calculate_average(vec: &Vec<Vec2>) -> Option<Vec2> {
    let num_elements = vec.len();

    if num_elements == 0 {
        return None; // Return None for an empty vector
    }

    let sum = vec.iter().fold(Vec2::new(0.0, 0.0), |acc, item| {
        Vec2::new(acc.x + item.x, acc.y + item.y)
    });

    Some(Vec2::new(
        sum.x / num_elements as f64,
        sum.y / num_elements as f64,
    ))
}