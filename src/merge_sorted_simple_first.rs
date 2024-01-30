pub struct Solution(());

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut num1_index = (m - 1) as usize;
        let mut num2_index = (n - 1) as usize;
        let mut result_index = (m + n - 1) as usize;

        let mut num1 = match m > 0 {
            true => nums1[num1_index],
            false => i32::MIN,
        };
        let mut num2 = match n > 0 {
            true => nums2[num2_index],
            false => i32::MIN,
        };

        loop {
            if num1 > num2 {
                nums1.swap(num1_index, result_index);
                if num1_index == 0 {
                    num1 = i32::MIN;
                } else {
                    num1_index -= 1;
                    num1 = nums1[num1_index];
                }
            } else {
                std::mem::swap(&mut nums2[num2_index], &mut nums1[result_index]);
                if num2_index == 0 {
                    num2 = i32::MIN;
                } else {
                    num2_index -= 1;
                    num2 = nums2[num2_index];
                }
            }

            if result_index == 0 {
                break;
            }
            result_index -= 1;
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
        &mut vec![-10,-10,-9,-9,-9,-9,-8,-8,-8,-8,-8,-7,-7,-7,-7,-7,-7,-7,-7,-6,-6,-6,-6,-5,-5,-5,-5,-5,-4,-4,-4,-4,-4,-3,-3,-3,-2,-2,-2,-2,-2,-2,-2,-1,-1,-1,0,0,0,0,0,1,1,1,2,2,2,2,2,2,2,2,3,3,3,3,4,4,4,4,4,4,4,5,5,5,5,5,5,6,6,6,6,6,7,7,7,7,7,7,7,8,8,8,8,9,9,9,9], 99, vec![-10,-10,-10,-10,-9,-9,-9,-9,-9,-9,-9,-8,-8,-8,-8,-8,-8,-8,-7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-7,-6,-6,-6,-6,-6,-6,-6,-6,-6,-6,-6,-5,-5,-5,-5,-5,-5,-5,-5,-4,-4,-4,-4,-4,-4,-4,-4,-3,-3,-3,-3,-3,-2,-2,-2,-2,-2,-2,-2,-2,-2,-1,-1,-1,-1,-1,0,0,0,0,0,0,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,4,5,5,5,5,5,5,5,5,6,6,6,6,6,6,6,6,6,7,7,7,7,7,7,7,7,7,7,7,8,8,8,8,8,9,9,9,9,9,9,9,9]; "failed test-case 6")]
    #[test_case(&mut vec![0, 1, 2, 4, 0, 0, 0], 4, &mut vec![3, 3, 3], 3, vec![0, 1, 2, 3, 3, 3, 4]; "simple example of failure")]
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
