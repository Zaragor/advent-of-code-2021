use crate::extensions::import_file;

pub fn day1() -> i32 {
    let depths = import_file::<i32>("src/input/day1.txt");
    let mut past_depth = depths[0];
    let mut steps_deeper = 0;
    for depth in depths {
        if depth > past_depth {
            steps_deeper = steps_deeper+ 1;
        }
        past_depth = depth;
    }
    return steps_deeper;
}


pub fn day1_2() -> i32 {
    let depths = import_file::<i32>("src/input/day1.txt");
    let average_depths: Vec<i32> = depths.windows(3).map(|window| { window.iter().sum::<i32>() }).collect();
    let mut past_depth = average_depths[0];
    let mut steps_deeper = 0;
    for depth in average_depths {
        if depth > past_depth {
            steps_deeper = steps_deeper + 1;
        }
        past_depth = depth;
    }
    return steps_deeper;
}