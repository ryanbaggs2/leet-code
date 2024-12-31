/// This was my second attempt for this, it has an O(n^2) time complexity, but it works.
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // For loop O(n)
    // Get index of first num
    for (first_index, num) in nums.iter().enumerate() {
        let difference = target - num;
        
        // Iteration is O(n)
        // Get index of second num, we start from the right in case there is the same value at 
        // another index, example: [3, 2, 5, 3] target 6
        let second_index = nums
            .iter()
            .rposition(|x| { x == &difference})
            .unwrap() as i32;
        
        // If indexes are the same we want to use another value
        if first_index as i32 == second_index {
            continue
        }
        
        return vec![first_index as i32, second_index];
    }
    
    vec![]
}

/// This was my initial solution, but it was missing the check if the first and second indexes
/// were the same, and was not searching for the position starting from the right. This has an 
/// O(n * log(n)) time complexity, both sorting and finding, so overall is O(n * log(n))
#[allow(dead_code)]
pub fn two_sum_binary(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // Sort is O(n * log(n))
    // Make a sorted copy of nums
    let mut sorted = nums.to_vec();
    sorted.sort();

    for (first_index, num) in nums.iter().enumerate() {
        let difference = target - num;
        
        // Binary search is O(log(n))
        // Search for the value
        if sorted.binary_search(&difference).is_ok() {
            // Iteration is O(n)
            // Get index of second num, we start from the right in case there is the same value at 
            // another index, example: [3, 2, 5, 3] target 6
            let second_index = nums
                .iter()
                .rposition(|x| { x == &difference})
                .unwrap() as i32;

            // If indexes are the same we want to use another value
            if first_index as i32 == second_index {
                continue
            }
            
            return vec![first_index as i32, second_index]
        }
    }
    
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_one() {
        let result = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_two() {
        let result = two_sum(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case_three() {
        let result = two_sum(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_four() {
        let result = two_sum_binary(vec![2, 7, 11, 15], 9);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn case_five() {
        let result = two_sum_binary(vec![3, 2, 4], 6);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn case_six() {
        let result = two_sum_binary(vec![3, 3], 6);
        assert_eq!(result, vec![0, 1]);
    }
}
