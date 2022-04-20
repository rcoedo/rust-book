use std::collections::HashMap;

fn main() {
    let input = &vec![1, 3, 5, 7, 3, 5, 3, 4, 7];

    println!(
        "The median is {} and the mode is {}, the vector was {:?}",
        median(&input),
        mode(&input),
        &input
    );
}

fn median(vec: &Vec<i32>) -> i32 {
    let mut vec = vec.to_vec();
    vec.sort();

    vec[vec.len() / 2]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for i in vec {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    map.iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k.to_owned())
        .expect("Vector should not be empty")
}
