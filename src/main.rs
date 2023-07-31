struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let c = target - nums[i];
            if map.contains_key(&c) {
                return vec![map[&c], i as i32]
            } else {
                map.insert(nums[i], i as i32);
            }
        }
        return vec![0, 1];
    }

}
fn main() {
    let nums = vec![3,3];
    let target = 6;
    let result = Solution::two_sum(nums, target);
    println!("[{0}, {1}]", result[0], result[1]);
}

