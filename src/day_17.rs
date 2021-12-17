use std::collections::HashSet;

fn solve() -> (i32, usize) {
    // let target_x = 20..=30;
    // let target_y = -10..=-5;
    let target_x = 70..=96;
    let target_y = -179..=-124;
    let mut y_highest_point = i32::MIN;
    let mut count_initials = HashSet::new();
    for svx in 1i32..=*target_x.end() {
        let mut px = 0;
        let mut vx = svx;
        let mut step = 0;
        loop {
            px += vx;
            vx -= vx.signum();
            step += 1;
            if target_x.contains(&px) {
                let mut highest_y = 0;
                for svy in -200i32..200 {
                    let (mut py, mut vy) = (0..step).fold((0, svy), |(mut py, mut vy), _| {
                        py += vy;
                        vy -= 1;
                        if py > highest_y {
                            highest_y = py;
                        }
                        (py, vy)
                    });
                    let mut px_0 = px;
                    let mut vx_0 = vx;

                    loop {
                        if target_x.contains(&px_0) && target_y.contains(&py) {
                            count_initials.insert((svx, svy));
                            if y_highest_point < highest_y {
                                y_highest_point = highest_y;
                            }
                            break;
                        }
                        if !target_x.contains(&px_0) || py < *target_y.start() {
                            break;
                        }
                        py += vy;
                        vy -= 1;
                        px_0 += vx_0;
                        vx_0 -= vx_0.signum();
                        if py > highest_y {
                            highest_y = py;
                        }
                    }
                }
            }
            if vx == 0 || px > *target_x.end() {
                break;
            }
        }
    }
    (y_highest_point, count_initials.len())
}

fn main() {
    adventofcode2021::print_time!({
        let res = solve();
        println!("part1: {}", res.0);
        println!("part2: {}", res.1);
    });
}
