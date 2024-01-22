pub struct Solution(());

// My second attempt after submitting and seeing how slow my first
// single-sided attempt was and that we can access both ends.
impl Solution {
    pub fn trap(heights: &Vec<i32>) -> i32 {
        let mut left_max = i32::MIN;
        let mut right_max = i32::MIN;

        let mut left_pos = 0;
        let mut right_pos = heights.len() - 1;

        let mut water_collected = 0;

        let mut height: &i32;

        while left_pos <= right_pos {
            if left_max <= right_max {
                height = &heights[left_pos];
                if *height > left_max {
                    left_max = *height;
                } else {
                    water_collected += left_max - *height;
                }
                left_pos += 1;
            } else {
                height = &heights[right_pos];
                if *height > right_max {
                    right_max = *height;
                } else {
                    water_collected += right_max - *height;
                }
                right_pos -= 1;
            }
        }
        water_collected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6; "Test case 1")]
    #[test_case(vec![4,2,0,3,2,5], 9; "Test case 2")]
    #[test_case(vec![0], 0; "Test failure on leet")]
    fn test_trap_v2(heights: Vec<i32>, trapped_water: i32) {
        assert_eq!(Solution::trap(&heights), trapped_water);
    }
}
