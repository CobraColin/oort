use oort_api::prelude::{Vec2, vec2};

pub fn sma_filter(vec: &Vec<Vec2>, window_size: usize) -> Vec<Vec2> {
    if window_size <= 0 || vec.is_empty() {
        return vec.clone(); // No smoothing needed
    }

    let mut smoothed_vec = Vec::new();

    for i in 0..vec.len() {
        let mut sum = vec2(0., 0.);


        for j in (i.saturating_sub(window_size - 1)..=i).take(window_size) {
            if j < vec.len() {
                sum += vec[j];
            }
        }

        let smoothed_value = sum/window_size as f64;
        smoothed_vec.push(smoothed_value);
    }

    smoothed_vec
}