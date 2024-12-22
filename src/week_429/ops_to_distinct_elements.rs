use::std::collections::HashSet;

pub fn minimum_operations(nums: Vec<i32>) -> i32 {
    // reverse nums
    let mut nums = nums;
    nums.reverse();

    // make hashset
    let mut set: HashSet<i32> = HashSet::new();

    // iter over values in nums, adding to hashset
    // if false, get number of elements in hashset
    for value in &nums {
        if !set.insert(*value) {
            break;
        }
    }

    // Calculate number of steps
    let difference = nums.len() - set.len();
    let mut steps: usize = difference / 3;
    if difference % 3 > 0 {
        steps += 1;
    }

    steps as i32
}