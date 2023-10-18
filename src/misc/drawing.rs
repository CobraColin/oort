use oort_api::prelude::{Vec2, draw_line};

pub fn draw_a_line_from_a_vec(vec_to_draw:&Vec<Vec2>,color:u32,max_lines:usize) {
    for i in 0..vec_to_draw.len() {
        if vec_to_draw.len() > max_lines {
            if i <= vec_to_draw.len() - max_lines {
                continue;
            }
        }

        if vec_to_draw.get(i + 1).is_none() {
            break;
        }

        let pred_from = vec_to_draw[i].clone();
        let pred_to = vec_to_draw[i + 1].clone();

        draw_line(pred_from, pred_to, color)
    }
}
