use std::error::Error;

use leet::trapping_rain_water::Solution;

fn main() -> Result<(), Box<dyn Error>> {
    let heights = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    for _i in 0..1000 {
        Solution::trap(&heights);
    }
    print!("Trapped water: {}\n", Solution::trap(&heights));
    Ok(())
}
