// Each water trap has a left and right wall. Since
// we're scanning the wall heights from left to right,
// we only need to track the left walls which can still
// possibly trap water.

// We'll fold each wall-height into an accumulator
// which is storing both the current left walls that can
// still possibly trap water, together with the
// current total trapped water.

// A left wall has a height and a bottom which is initially zero
// but may be raised by a following wall that will take over tracking
// lower trapped water.
#[derive(Debug)]
struct LeftWall {
    pos: i32,
    height: i32,
    bottom: i32,
}

// We could just use a tuple, but for clarity, define a
// struct for the accumulator.
#[derive(Debug)]
struct Accumulator {
    water: i32,
    walls: Vec<LeftWall>,
}

pub struct Solution(());

impl Solution {
    pub fn trap(heights: &Vec<i32>) -> i32 {
        let accumulated = heights.iter().enumerate().fold(
            Accumulator {
                water: 0,
                walls: Vec::with_capacity(heights.len() / 2),
            },
            |mut acc, (pos, height)| {
                // println!("acc={:?}", &acc);

                // Guard against a zero height which doesn't change
                // our accumulator at all.
                if *height == 0 {
                    return acc;
                }

                // Next, sum the water collected by this current
                // wall (if any), raising the existing left-wall bottoms
                // to the new height (whether it's filled with water or a new
                // left-wall has taken over tracking water at that level).
                let mut water_collected = 0;

                // Also try specifying the size of the vec.
                acc.walls.retain_mut(|left_wall| {
                    // Collect the water only for those left edges whose bottom extends to
                    // the heigth of this new wall.
                    //   |nn
                    //   ||y|
                    if left_wall.bottom < *height {
                        water_collected += (left_wall.height.min(*height) - left_wall.bottom)
                            * ((pos as i32 - 1) - left_wall.pos);
                        // println!("water collected after
                        // {left_wall:?}: {water_collected}");
                    }

                    if left_wall.height <= *height {
                        // If the left-wall was equal or less in height to the current one,
                        // we filter it out by returning
                        // None for the filter_map result.
                        false
                    } else {
                        // Otherwise we leave the left wall in our walls, but adjust its bottom
                        // to the height of this new wall if it is higher than the current
                        // bottom, since the new wall will track the
                        // trapped water to that level.
                        left_wall.bottom = left_wall.bottom.max(*height);
                        true
                    }
                });

                acc.water += water_collected;
                // Finally add the new left wall.
                acc.walls.push(LeftWall {
                    pos: pos as i32,
                    height: *height,
                    bottom: 0,
                });

                acc
            },
        );
        accumulated.water
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6; "Test case 1")]
    #[test_case(vec![4,2,0,3,2,5], 9; "Test case 2")]
    fn test_trap(heights: Vec<i32>, trapped_water: i32) {
        assert_eq!(Solution::trap(&heights), trapped_water);
    }
}
