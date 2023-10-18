use oort_api::prelude::{Vec2, Vec2Extras};

pub fn two_vecs_have_same_signs(compare1:Vec2,compare2:Vec2) -> bool {
    if (compare1.x.is_sign_negative() && compare2.x.is_sign_positive()) || ((compare1.y.is_sign_negative() && compare2.y.is_sign_positive())) {
        return false;
    }
    return true;
}

pub fn get_change_in_percentage_between_two_acceleration_vec2(older: &Vec2, new: &Vec2) -> f64 {
    let magnitudeolder = older.length();
    let magnitudenew = new.length();

    if magnitudeolder == 0.0 {
        return 100.0; // If the initial value is 0, we'll consider the change to be 100%
    }

    let change = magnitudenew - magnitudeolder;
    let percentage_change = (change / magnitudeolder) * 100.0;

    percentage_change.abs()
}