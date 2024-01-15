use std::collections::HashMap;

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
#[derive(Clone, Copy, Debug)]
struct LeftWall {
    height: i32,
    bottom: i32,
}

// The left-walls in the walls hashmap are indexed by their
// position in the input array.
type Walls = HashMap<i32, LeftWall>;

// We could just use a tuple, but for clarity, define a
// struct for the accumulator.
#[derive(Debug)]
struct Accumulator {
    water: i32,
    walls: Walls,
}
impl Accumulator {
    fn new() -> Self {
        Accumulator {
            water: 0,
            walls: HashMap::new(),
        }
    }
}

pub struct Solution(());

impl Solution {
    pub fn trap(heights: &Vec<i32>) -> i32 {
        let accumulated =
            heights
                .iter()
                .enumerate()
                .fold(Accumulator::new(), |acc, (pos, height)| {
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
                    let mut new_walls: Walls = acc
                        .walls
                        .iter()
                        .filter_map(|(&left_pos, left_wall)| {
                            // Collect the water only for those left edges whose bottom extends to
                            // the heigth of this new wall.
                            //   |nn
                            //   ||y|
                            if left_wall.bottom < *height {
                                water_collected += (left_wall.height.min(*height)
                                    - left_wall.bottom)
                                    * ((pos as i32 - 1) - left_pos);
                                // println!("water collected after
                                // {left_wall:?}: {water_collected}");
                            }

                            if left_wall.height <= *height {
                                // If the left-wall was equal or less in height to the current one,
                                // we filter it out by returning
                                // None for the filter_map result.
                                return None;
                            }

                            // Otherwise we leave the left wall in our walls, but adjust its bottom
                            // to the height of this new wall if it is higher than the current
                            // bottem, since the new wall will track the
                            // trapped water to that level.
                            Some((
                                left_pos,
                                LeftWall {
                                    height: left_wall.height,
                                    bottom: left_wall.bottom.max(*height),
                                },
                            ))
                        })
                        .collect();

                    // Finally add the new left wall.
                    new_walls.insert(
                        pos as i32,
                        LeftWall {
                            height: *height,
                            bottom: 0,
                        },
                    );

                    Accumulator {
                        water: acc.water + water_collected,
                        walls: new_walls,
                    }
                });
        accumulated.water
    }
}
