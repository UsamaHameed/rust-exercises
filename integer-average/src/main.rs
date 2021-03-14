use std::collections::HashMap;
use indexmap::IndexMap;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 1, 4, 4, 1];
    println!("average = {:.3}", average(&numbers));

    println!("mean = {}", mean(&numbers));

    println!("mode = {:?}", mode(&numbers));

}

fn average(numbers: &Vec<i32>) -> f32 {
    let mut sum = 0.0;
    for &i in numbers {
        sum = sum + (i as f32)
    }
    let size = numbers.len() as f32;
    sum / size
}

fn mean(numbers: &Vec<i32>) -> i32 {
    let mut copy = numbers.to_vec();
    copy.sort();
    let size = numbers.len();
    let middle = size / 2;
    copy[middle]
}

fn mode(numbers: &Vec<i32>) -> &i32 {
    let mut map = IndexMap::new();

    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    map.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Error")
}