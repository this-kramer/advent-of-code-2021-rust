use std::cmp::max;

fn main() {
    println!("The result is {}", find_highest_y(288, 330, -96, -50));
}

fn find_highest_y(x_min: u32, x_max: u32, y_min: i32, y_max: i32) -> i32 {
    let v_x_min = compute_x_min(x_min);
    let v_x_max = x_max;
    let mut max_height = i32::MIN;
    for v_x_start in v_x_min..=v_x_max {
        let mut pos_x = 0;
        let mut v_x = v_x_start;
        let mut steps = 0;
        while pos_x <= x_max {
            println!("v_x: {}, pos_x: {}, steps: {}", v_x, pos_x, steps);
            pos_x += v_x;
            v_x = max(v_x as i32 - 1, 0) as u32;
            steps += 1;
            if (x_min..=x_max).contains(&pos_x) {
                // y + y-1 + ... + 1 + 0  | y+1
                // -1 -2 -3 ... -(steps-y-1)
                // = y(y+1)/2 - (steps-y-1)(steps-y)/2
                // = 0.5 [y^2+y-steps^2+ysteps+steps+ysteps-y^2-y]
                // = 0.5 [ steps^2 + 2ysteps+steps]

                // reduce by 1 -> target height reduced by y+steps-y-1 = steps=1
                // yields some positive or negative subset sum

                let mut y = y_min;
                loop {
                    let target_y = (steps * steps + 2 * y * steps + steps) / 2;
                    if target_y > y_max {
                        break;
                    } else if target_y >= y_min {
                        max_height = max(max_height, y * (y + 1) / 2);
                    }
                    y += 1;
                }
            }
            if v_x == 0 {
                break;
            }
        }
    }
    max_height
}

/// Compute minimum starting velocity required to reach the target area
/// x -> x+(x-1)+...+1 = x(x+1)/2 max distance => x_min such that x(x+1)/2 > x_1, x_max = x_2
/// x^2 + x - 2x_1 > 0 => 1/2 +- sqrt(1/4+2x_1) ~ sqrt(x_1)
fn compute_x_min(x_1: u32) -> u32 {
    f64::ceil(-0.5 + f64::sqrt(0.25 + (2 * x_1) as f64)) as u32
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1, 1)]
    #[case(2, 2)]
    #[case(3, 2)]
    #[case(4, 3)]
    fn test_v_x_min(#[case] x_1: u32, #[case] x_min: u32) {
        assert_eq!(x_min, compute_x_min(x_1));
    }

    #[rstest]
    #[case(20, 30, -10, -5)]
    fn test_find_highest_min(
        #[case] x_min: u32,
        #[case] x_max: u32,
        #[case] y_min: i32,
        #[case] y_max: i32,
    ) {
        assert_eq!(45, find_highest_y(x_min, x_max, y_min, y_max))
    }
}
