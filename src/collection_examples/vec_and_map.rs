use std::collections::HashMap;

pub fn median(values: &[i32]) -> f64 {
    let mut sorted_values: Vec<i32> = Vec::from(values);
    sorted_values.sort();

    let mid = sorted_values.len() / 2;

    match sorted_values.len() % 2 == 0 {
        true => (sorted_values[mid] + sorted_values[mid - 1]) as f64 / 2.0,
        false => sorted_values[mid] as f64,
    }
}

pub fn mode(values: &[i32]) -> i32 {
    let mut value_map: HashMap<i32, i32> = HashMap::new();

    for &i in values {
        let count = value_map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut result: i32 = 0;

    for (key, value) in value_map {
        if value > max_count {
            result = key;
            max_count = value;
        }
    }

    result
}
