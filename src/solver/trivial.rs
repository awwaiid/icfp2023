use crate::icfp::*;

pub fn solve(problem: &Problem) -> Solution {
    let mut players: Vec<Position> = Vec::new();

    let stage_x_min = problem.stage_bottom_left[0];
    let stage_y_min = problem.stage_bottom_left[1];
    let stage_x_max = stage_x_min + problem.stage_width;
    let stage_y_max = stage_y_min + problem.stage_height;

    let mut y = stage_y_min + 10.0;
    'iter: while y <= stage_y_max - 10.0 {
        let mut x = stage_x_min + 10.0;
        while x <= stage_x_max - 10.0 {
            if players.len() >= problem.musicians.len() {
                break 'iter;
            }
            players.push(Position {
                x: x as f32,
                y: y as f32,
            });
            x = x + 10.0;
        }
        y = y + 10.0;
    }

    Solution {
        placements: players,
    }
}
