use oort_api::prelude::Vec2;

use crate::math::{general::calculate_average, smoothing_functions::sma_filter, vector_logic::{get_change_in_percentage_between_two_acceleration_vec2, two_vecs_have_same_signs}};


fn get_until_signs_are_not_same(vec_of_a:&Vec<Vec2>) -> usize {
    //returns false if the vec is empty
    if vec_of_a.len() == 0 {
        return 0
    }
    let compare_to = *vec_of_a.last().unwrap();
    for i in (0..vec_of_a.len()-1).rev() {
        
        if !two_vecs_have_same_signs(compare_to,vec_of_a[i]) {
            return i
        }
    }

    return 0
}


fn get_until_acceleration_changes_too_much(vec_of_a:&Vec<Vec2>,percentage:f64) -> usize {
    //returns 0 if the vec is empty
    if vec_of_a.len() == 0 {
        return 0
    }

    let compare_to = *vec_of_a.last().unwrap();
    let mut max_recorded_value = 0.0;
    for i in (0..vec_of_a.len()-1).rev() {
        let change_percentage = 
        get_change_in_percentage_between_two_acceleration_vec2(
            &vec_of_a[i],
            &compare_to
        );
        if change_percentage > max_recorded_value {
            max_recorded_value = change_percentage
        } else {
            return i
        }
        
        if change_percentage > percentage {
            return i
        }
    }

    return 0
}



pub fn take_direction_into_acount(mut accelerations:Vec<Vec2>) -> Vec2 {
    let mut vec_length = get_until_signs_are_not_same(&accelerations);
    


    while  accelerations.len() > 10 && vec_length > 50 {
        accelerations.remove(accelerations.len()-1);
        vec_length = get_until_signs_are_not_same(&accelerations);
    }

    
    if accelerations.len() > 10 {
        let mut new_vec = accelerations.clone().split_off(vec_length);
        return calculate_average(&new_vec).unwrap();
        // debug!("veclength {}",new_vec.len());
    
        // let new_vec_length_for_acceleration_change = get_until_acceleration_changes_too_much(
        //     &new_vec,
        //     10.
        // );
     
        // debug!("new_vec_length_for_acceleration_change: {}",new_vec_length_for_acceleration_change);
    
        
    
    
        // if new_vec.len() > 10 && new_vec_length_for_acceleration_change < new_vec.len() {
        // // if new_vec.len() > 10 {
        //     new_vec = new_vec.clone().split_off(new_vec_length_for_acceleration_change);
        //     return calculate_average(&new_vec).unwrap();
        // }
    }

    return calculate_average(&sma_filter(&accelerations, 2)).unwrap();
}