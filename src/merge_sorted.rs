pub struct Solution(());

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        // Could advance the index of nums1 or swap into nums2 before advancing the
        // index? No, can't simply swap as this can set nums2 out of
        // non-decreasing order. Could instead end up with two pointers into
        // nums2, one for the actual index, the other for the cache of num1
        // numbers waiting to be readded.
        let mut nums1_index: usize = 0;
        let mut nums2_index: usize = 0;
        let mut cached_index: usize = 0;
        let mut cache_len: usize = 0;

        while nums1_index < (m + n) as usize {
            let num1 = match nums1_index < m as usize {
                true => nums1[nums1_index],
                false => i32::MAX,
            };
            let num2 = match nums2_index < n as usize {
                true => nums2[nums2_index],
                false => i32::MAX,
            };
            let cached_num = match cache_len {
                0 => i32::MAX,
                _ => nums2[cached_index],
            };

            dbg!(
                nums1_index,
                num1,
                nums2_index,
                num2,
                cached_index,
                cached_num,
                cache_len,
                &nums1,
                &nums2
            );
            // TODO: don't need to calculate this condition every loop
            // TODO: need to check the cache here also.
            if nums1_index < m as usize && num1 <= num2 && num1 <= cached_num {
                nums1_index += 1;
                continue;
            } else if num2 <= num1 && num2 <= cached_num {
                std::mem::swap(&mut nums1[nums1_index], &mut nums2[nums2_index]);
                dbg!("after", &nums1, &nums2);

                // This needs to do *either* cached num or nums2_index. Currently
                // the logic appears to assume we're swapping nums2_index, not the
                // cached value.

                // We're going to start the cache at the current nums2_index
                // if num1 != 0 && cache_len == 0 {
                if nums1_index < m as usize {
                    if cache_len == 0 {
                        cached_index = nums2_index;
                    }
                    cache_len += 1;
                }

                nums1_index += 1;
                nums2_index += 1;
            } else {
                // The cached num must be the smallest
                // UPTOHERE: can't just swap into the first cache entry like this as
                // it puts the cache out of order. Perhaps need a ring-buffer for the
                // cache? Or start by just sorting, but must be better way.
                std::mem::swap(&mut nums2[cached_index], &mut nums1[nums1_index]);
                if nums1_index >= m as usize {
                    // It was a zero
                    cached_index += 1;
                    cache_len -= 1;
                }
                nums2[cached_index..(cached_index + cache_len)].sort_unstable();
                dbg!("after", &nums1, &nums2);
                nums1_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(&mut vec![1,2,3,0,0,0], 3, &mut vec![4,5,6], 3, vec![1,2,3,4,5,6]; "just merge")]
    #[test_case(&mut vec![1,2,3,0,0,0], 3, &mut vec![2,5,6], 3, vec![1,2,2,3,5,6]; "small ordering")]
    #[test_case(&mut vec![0], 0, &mut vec![1], 1, vec![1]; "zero nums1")]
    #[test_case(&mut vec![0, 1, 2, 0, 0, 0], 3, &mut vec![1, 2, 3], 3, vec![0, 1, 1, 2, 2, 3]; "with zeros")]
    #[test_case(&mut vec![2, 0], 1, &mut vec![1], 1, vec![1, 2]; "failed test-case 1")]
    #[test_case(&mut vec![4, 0, 0, 0, 0, 0], 1, &mut vec![1, 2, 3, 5, 6], 5, vec![1, 2, 3, 4, 5, 6]; "failed test-case 2")]
    #[test_case(&mut vec![0, 0, 3, 0, 0, 0, 0, 0, 0], 3, &mut vec![-1, 1, 1, 1, 2, 3], 6, vec![-1, 0, 0, 1, 1, 1, 2, 3, 3]; "failed test-case 3")]
    #[test_case(&mut vec![1, 5, 8, 0, 0, 0, 0, 0], 3, &mut vec![-1, 2, 2, 4, 6], 5, vec![-1, 1, 2, 2, 4, 5, 6, 8]; "failed test-case 4")]
    #[test_case(&mut vec![4,5,6,0,0,0], 3, &mut vec![1,2,3], 3, vec![1,2,3,4,5,6]; "failed test-case 5")]
    #[test_case(
        &mut vec![-10,-10,-9,-9,-9,-8,-8,-7,-7,-7,-6,-6,-6,-6,-6,-6,-6,-5,-5,-5,-4,-4,-4,-3,-3,-2,-2,-1,-1,0,1,1,1,2,2,2,3,3,3,4,5,5,6,6,6,6,7,7,7,7,8,9,9,9,9,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 55,
        &mut vec![-10,-10,-9,-9,-9,-9,-8,-8,-8,-8,-8,-7,-7,-7,-7,-7,-7,-7,-7,-6,-6,-6,-6,-5,-5,-5,-5,-5,-4,-4,-4,-4,-4,-3,-3,-3,-2,-2,-2,-2,-2,-2,-2,-1,-1,-1,0,0,0,0,0,1,1,1,2,2,2,2,2,2,2,2,3,3,3,3,4,4,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,7,7,7,7,7,7,7,8,8,8,8,9,9,9,9], 99,
        vec![-10,-10,-10,-10,-9,-9,-9,-9,-9,-9,-9,-8,-8,-8,-8,-8,-8,-8,-7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-6,-6,-6,-6,-6,-6,-6,-6,-6,-6,-6,-5,-5,-5,-5,-5,-5,-5,-5,-4,-4,-4,-4,-4,-4,-4,-4,-3,-3,-3,-3,-3,-2,-2,-2,-2,-2,-2,-2,-2,-2,-1,-1,-1,-1,-1,0,0,0,0,0,0,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,7,7,7,7,8,8,8,8,8,9,9,9,9,9,9,9,9]; "failed test-case 6")]
    #[test_case(&mut vec![-10,-10,-9,-9,-9,-8,-8, -7, -7, -7, 0,0,0,0,0,0,0,0], 10, &mut vec![-10,-10,-9,-9,-9,-9, -8, -8], 8, vec![-10, -10, -10, -10, -9, -9, -9, -9, -9, -9, -9, -8, -8, -8, -8, -7, -7, -7]; "simple example of failure")]
    #[test_case(&mut vec![-50,-50,-48,-46,-45,-45,-42,-41,-41,-40,-38,-38,-37,-36,-35,-34,-32,-31,-31,-31,-29,-28,-28,-26,-26,-26,-24,-22,-21,-21,-18,-16,-16,-16,-15,-14,-13,-13,-12,-11,-11,-6,-6,-5,-4,-3,-3,-1,0,1,2,3,4,5,6,7,7,7,7,8,10,12,13,15,16,17,19,19,20,21,22,23,24,24,25,27,28,29,30,30,31,31,34,34,34,34,35,36,37,38,39,42,43,44,44,47,49,49,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], 98, &mut vec![-49,-44,-44,-41,-40,-38,-37,-36,-32,-28,-26,-23,-23,-22,-22,-22,-20,-19,-16,-12,-11,-8,-6,-5,-4,-2,2,3,5,5,6,7,9,9,11,11,12,13,14,15,20,21,21,22,22,23,24,25,29,29,30,31,31,33,33,34,34,34,37,37,40,40,41,41,41,41,43,43,43,44,46,47,49], 73, vec![1]; "time limit")]
    fn test_merge_sorted(
        nums1: &mut Vec<i32>,
        m: i32,
        nums2: &mut Vec<i32>,
        n: i32,
        expected: Vec<i32>,
    ) {
        Solution::merge(nums1, m, nums2, n);
        assert_eq!(nums1, &expected);
    }
}
