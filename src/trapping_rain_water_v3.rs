#[derive(Debug, PartialEq)]
struct AirBox {
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq)]
pub struct TrappedSlice {
    water: i32,
    highest_point: i32,

    // Since the slice will now be filled with water, the remaining
    // air either side is always stacked from the outside ground
    // level.
    left_air: Vec<AirBox>,
    right_air: Vec<AirBox>,
}

pub struct Solution(());

// A third solution that can divide and conquer concurrently. Can't see an
// improvement yet, need to find out whether the spawning is just expensive or
// something else is causing the slowdown, or profile to see if it's allocating
// the vecs for the air boxes.
impl Solution {
    pub fn trap(heights: &Vec<i32>) -> i32 {
        if heights.len() > 5000 {
            std::thread::scope(|s| {
                let mid = heights.len() / 2;
                let (left, right) = heights.split_at(mid);
                let left_handle = s.spawn(|| Solution::trap_slice(left, true));
                let right_handle = s.spawn(|| Solution::trap_slice(right, true));
                let left = left_handle.join().unwrap();
                let right = right_handle.join().unwrap();
                Solution::join_slices(left, right).water
            })
        } else {
            Solution::trap_slice(&heights, false).water
        }
    }

    // trap_slice returns the water trapped by the slice in isolation as well as the
    // left and right boxes of air that may be filled when merged with an
    // adjacent slice.
    pub fn trap_slice(heights: &[i32], require_air: bool) -> TrappedSlice {
        let len = heights.len();
        let mut left_max = 0;
        let mut right_max = 0;
        let mut left_air = Vec::with_capacity(1000);
        let mut right_air = Vec::with_capacity(1000);

        let mut left_pos: i32 = -1;
        let mut right_pos = len;

        let mut water = 0;

        let mut height: &i32;

        while left_pos < right_pos as i32 {
            if left_max <= right_max {
                if left_pos == (len - 1) as i32 {
                    break;
                }
                left_pos += 1;
                height = &heights[left_pos as usize];
                if *height > left_max {
                    if require_air {
                        left_air.push(AirBox {
                            width: left_pos as usize,
                            height: (*height - left_max) as usize,
                        });
                    }
                    left_max = *height;
                } else {
                    water += left_max - *height;
                }
            } else {
                right_pos -= 1;
                height = &heights[right_pos];
                if *height > right_max {
                    if require_air {
                        right_air.push(AirBox {
                            width: len - (right_pos + 1),
                            height: (*height - right_max) as usize,
                        });
                    }
                    right_max = *height;
                } else {
                    water += right_max - *height;
                }
            }
        }
        TrappedSlice {
            water,
            highest_point: heights[left_pos as usize],
            left_air,
            right_air,
        }
    }

    pub fn join_slices(left: TrappedSlice, right: TrappedSlice) -> TrappedSlice {
        let mut water = left.water + right.water;

        let max_water_level = left.highest_point.min(right.highest_point);

        // Fill the air boxes that are lower than the max water level
        let mut rising_water_level = 0;
        for air_box in left.right_air {
            let fill_height = air_box
                .height
                .min((max_water_level - rising_water_level) as usize);
            water += (fill_height * air_box.width) as i32;
            rising_water_level += fill_height as i32;
        }

        rising_water_level = 0;
        for air_box in right.left_air {
            let fill_height = air_box
                .height
                .min((max_water_level - rising_water_level) as usize);
            water += (fill_height * air_box.width) as i32;
            rising_water_level += fill_height as i32;
        }

        TrappedSlice {
            water,
            highest_point: left.highest_point.max(right.highest_point),
            left_air: left.left_air,
            right_air: right.right_air,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6; "Test case 1")]
    #[test_case(vec![4,2,0,3,2,5], 9; "Test case 2")]
    #[test_case(vec![0], 0; "Test failure on leet")]
    fn test_trap_v3(heights: Vec<i32>, trapped_water: i32) {
        assert_eq!(Solution::trap(&heights), trapped_water);
    }

    #[test_case(vec![0, 1, 0], TrappedSlice{ water: 0, highest_point: 1, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 1}]}; "single wall")]
    #[test_case(vec![1, 0, 1], TrappedSlice{ water: 1, highest_point: 1, left_air: vec![AirBox{width: 0, height: 1}], right_air: vec![AirBox{width: 0, height: 1}]}; "single water")]
    #[test_case(vec![0, 1, 2, 1, 0], TrappedSlice{ water: 0, highest_point: 2, left_air: vec![AirBox{width: 1, height: 1}, AirBox{width: 2, height: 1}], right_air: vec![AirBox{width: 1, height: 1}, AirBox{width: 2, height: 1}]}; "small tower")]
    fn test_trap_slice(heights: Vec<i32>, trapped_slice: TrappedSlice) {
        assert_eq!(Solution::trap_slice(&heights, false), trapped_slice);
    }

    #[test_case(
        TrappedSlice{ water: 0, highest_point: 1, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 1}]},
        TrappedSlice{ water: 0, highest_point: 1, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 1}]},
        TrappedSlice{ water: 2, highest_point: 1, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 1}]};
        "two single walls")]
    #[test_case(
        TrappedSlice{ water: 1, highest_point: 3, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 4}]},
        TrappedSlice{ water: 0, highest_point: 4, left_air: vec![AirBox{width: 1, height: 3}], right_air: vec![AirBox{width: 1, height: 1}]},
        TrappedSlice{ water: 7, highest_point: 4, left_air: vec![AirBox{width: 1, height: 1}], right_air: vec![AirBox{width: 1, height: 1}]};
        "one airbox that is higher than the max water level")]
    fn test_join_slices(left: TrappedSlice, right: TrappedSlice, joined: TrappedSlice) {
        assert_eq!(Solution::join_slices(left, right), joined);
    }
}
