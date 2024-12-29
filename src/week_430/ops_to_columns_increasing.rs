use std::collections::VecDeque;

pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
    let mut operations = 0;
    let flat = grid.clone().into_iter().flatten().collect::<Vec<i32>>();
    let mut rotated_grid: Vec<Vec<i32>> = vec![];
    let row_len = grid[0].len();
    
    // Create a rotated grid
    for i in 0..grid[0].len() {
        rotated_grid.push(
            flat
                .clone()
                .into_iter()
                .skip(i)
                .step_by(row_len)
                .collect::<Vec<i32>>()
        )
    }
    
    // Count the operations
    for column in rotated_grid {
        let mut last: Option<i32> = None;
        
        for mut value in column {
            if last.is_none() {
                last = Some(value);
                continue
            }
            
            while value <= last.unwrap() {
                value += 1;
                operations += 1;
            }
            
            last = Some(value);
        }
    }
        
    operations
}