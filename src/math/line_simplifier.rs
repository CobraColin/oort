use oort_api::prelude::Vec2;



pub fn douglas_peucker_iterative(line: &Vec<Vec2>, epsilon: f64) -> Vec<Vec2> {
    let mut stack = vec![(0, line.len() - 1)];
    let mut result = vec![false; line.len()];
    result[0] = true;
    result[line.len() - 1] = true;

    while let Some((start, end)) = stack.pop() {
        let (mut index, mut max_dist) = (0, 0.0);

        for i in start + 1..end {
            let dist = line_distance(&line[i], &line[start], &line[end]);
            if dist > max_dist {
                index = i;
                max_dist = dist;
            }
        }

        if max_dist >= epsilon {
            result[index] = true;
            stack.push((start, index));
            stack.push((index, end));
        }
    }

    line.iter().enumerate().filter_map(|(i, &point)| if result[i] { Some(point) } else { None }).collect()
}

fn line_distance(point: &Vec2, start: &Vec2, end: &Vec2) -> f64 {
    let n = ((end.y - start.y) * point.x - (end.x - start.x) * point.y + end.x * start.y - end.y * start.x).abs();
    let d = ((end.y - start.y).powi(2) + (end.x - start.x).powi(2)).sqrt();
    n / d
}