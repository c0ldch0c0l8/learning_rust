use std::collections::HashMap;

pub fn get_central_tendency(vec: &Vec<i32>) -> (i32, i32, i32) {
    for i in vec {
        print!("{} ", i);
    }
    println!();

    let mut sum: i32 = 0;
    for i in vec {
        sum += i;
    }

    let avg: i32 = sum / vec.len() as i32;

    let median: i32 = if vec.len() % 2 == 0 { // if there is an even number of elements
        vec[(vec.len() / 2) - 1] // minus 1 because vecs start at 0
    } else {
        vec[(vec.len() - 1) / 2] // minus 1 so we get an even number
    };

    let mut count_map:HashMap<i32, i32> = HashMap::new();
    for n in vec {
        count_map.entry(*n).or_insert(0);

        let value_for_n: i32 = match count_map.get(n) {
            Some(i) => *i,
            None => 0
        };

        count_map.insert(*n,  value_for_n + 1);
    }

    let ( mut highest_value, mut highest_key) = (0, 0);
    let mut mode = 0;
    for (key, value) in count_map {
        if value > highest_value {
            highest_value = value;
            highest_key = key;
        }
        
        mode = highest_key;
    };

    // currently, if there multiple modes, only first is displayed...

    (avg, median, mode)
}
