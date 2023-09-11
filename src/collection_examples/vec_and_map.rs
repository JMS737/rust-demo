use std::collections::HashMap;

pub fn vector_iteration() {
    let mut v = vec![1, 2, 3, 4, 5];

    println!("My list:");
    for i in &v {
        println!("Item: {i}");
    }

    // Mutate the values in the list.
    for i in &mut v {
        *i *= *i;
    }

    println!("My mutated list:");

    for i in &v {
        println!("Item: {i}");
    }
}

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

pub fn public_sum(values: &[i32]) -> i32 {
    let sum = sum(values);

    println!("Sum of {:?} is {}", values, sum);

    sum
}

fn sum(values: &[i32]) -> i32 {
    let mut total = 0;

    values
        .iter()
        .map(|p| {
            println!("{p}");
            p
        })
        .all(|_| true);
    values.iter().sum()

    // for i in values {
    //     total += i;
    // }

    // total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_median() {
        let v = vec![1, 2, 3];
        assert_eq!(median(&v), 2.0);

        let v = vec![1, 2, 3, 4];
        assert_eq!(median(&v), 2.5);

        let v = vec![1, 3, 5, 5, 3];
        assert_eq!(median(&v), 3.0);
    }

    #[test]
    fn mode() {
        let v = vec![1, 2, 3, 1];
        assert_eq!(super::mode(&v), 1);

        let v = vec![1, 2, 2, 2, 3, 3, 4];
        assert_eq!(super::mode(&v), 2);

        let v = vec![1, 3, 5, 5, 4];
        assert_eq!(super::mode(&v), 5);
    }

    // Note how private functions can be tested in these unit tests because they're defined in the same file.
    #[test]
    fn check_sum() {
        let v = vec![1, 2, 3, 1];
        assert_eq!(sum(&v), 7);

        let v = vec![1, 2, 3, 4];
        assert_eq!(sum(&v), 10);

        let v = vec![1, 3, 5, 5, 3];
        assert_eq!(sum(&v), 17);
    }
}
