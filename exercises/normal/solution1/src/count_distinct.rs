use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hash_set = HashSet::<&str>::new();
    // println!("input_str: {}", input_str);
    {
        input_str
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .for_each(|char| {
                hash_set.insert(*char);
            });
    }

    hash_set.len()
}
