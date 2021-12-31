fn main() {
    println!(
        "The result is {}",
        count_starting_velocities(288, 330, -96, -50)
    );
}

fn count_starting_velocities(x_min: u32, x_max: u32, y_min: i32, y_max: i32) -> usize {
    let mut results = Vec::<(u32, i32)>::new();

    for v_x_start in compute_x_min(x_min)..=x_max {
        for v_y_start in y_min..=i32::abs(y_min) {
            // Larger starting values for y will always cause overshotting the target area due to symmetry in velocity computation
            let mut v_x = v_x_start;
            let mut v_y = v_y_start;
            let mut x = 0;
            let mut y = 0;

            while y >= y_min {
                x += v_x;
                y += v_y;
                v_x = if v_x > 0 { v_x - 1 } else { 0 };
                v_y -= 1;
                if (y_min..=y_max).contains(&y) && (x_min..=x_max).contains(&x) {
                    results.push((x, y));
                    break;
                }
            }
        }
    }

    for result in &results {
        println!("({}, {})", result.0, result.1);
    }
    results.len()
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
        assert_eq!(112, count_starting_velocities(x_min, x_max, y_min, y_max))
    }
}
