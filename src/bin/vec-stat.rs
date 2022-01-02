fn main() {
    let ints = vec![2, 3, 1, 1, 40, 5, 5];
    println!("{:?}, mean = {}", ints, mean(&ints));
    println!("{:?}, median = {}", ints, median(&ints));
    println!("{:?}, most frequent = {}", ints, mode(&ints));
}

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0.0;
    let mut num = 0;
    for val in v {
        sum += f64::from(*val);
        num += 1;
    }
    sum / f64::from(num)
}

fn median(v: &Vec<i32>) -> i32 {
    let mut sorted = v.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    sorted[mid]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut freq_map = std::collections::HashMap::new();
    let mut max_val = 0;
    let mut max_key = 0;

    for val in v {
        let num = freq_map.entry(val).or_insert(0);
        *num += 1;
    }

    for (key, value) in &freq_map {
        if *value > max_val {
            max_val = *value;
            max_key = **key;
        }
    }
    max_key
}
