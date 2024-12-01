use std::collections::HashMap;
use std::fs;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn generate_vectors(contents: String) -> Result<(Vec<i32>, Vec<i32>), std::io::Error> {
    let mut v: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    for line in contents.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        v.push(words[0].parse::<i32>().unwrap());
        v2.push(words[1].parse::<i32>().unwrap());
    }
    Ok((v, v2))
}

fn sort_and_compare_distances(v: &mut Vec<i32>, v2: &mut Vec<i32>) -> i32 {
    // Need to sort the vectors to compare smallest to largest.
    let mut distances: i32 = 0;
    v.sort();
    v2.sort();

    for i in 0..v.len() {
        distances += (v[i] - v2[i]).abs();
    }

    distances
}

fn prompt1() {
    let contents = read_file("src/prompt.txt").unwrap();
    let (mut v, mut v2) = generate_vectors(contents).unwrap();

    let distance = sort_and_compare_distances(&mut v, &mut v2);

    println!("The distance between the two vectors is: {}", distance);
}

fn generate_similarity_hash(r_list: &Vec<i32>) -> HashMap<i32, i32> {
    let mut similarity_hash: HashMap<i32, i32> = HashMap::new();

    for nums in r_list {
        let count = similarity_hash.entry(*nums).or_insert(0);
        *count += 1;
    }

    similarity_hash
}

fn calcuate_similiarity_distance(l_vec: &Vec<i32>, r_vec: &Vec<i32>) -> i32 {
    let mut distance: i32 = 0;
    let mut r_hash = generate_similarity_hash(r_vec);

    // Multipy each value in left vector by value pair in right vector.
    // i.e., [3,4,2] and {3: 2, 4: 1, 2: 1} = 3*2 + 4*1 + 2*1 = 14

    for num in l_vec {
        // If the value is not in the hash, then the count is 0.
        let count = r_hash.entry(*num).or_insert(0);

        // Multiply the value in the left vector by the count in the right vector.
        distance += num * *count;
    }

    distance
}

fn prompt2() {
    let contents = read_file("src/prompt.txt").unwrap();
    let (v, v2) = generate_vectors(contents).unwrap();

    let similarity_distance = calcuate_similiarity_distance(&v, &v2);

    println!(
        "The distance between the two vectors is: {}",
        similarity_distance
    );
}

fn main() {
    prompt1();
    prompt2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_vectors() {
        let contents = "1 2\n3 4\n5 6".to_string();
        let (v, v2) = generate_vectors(contents).unwrap();
        assert_eq!(v, vec![1, 3, 5]);
        assert_eq!(v2, vec![2, 4, 6]);
    }
    #[test]
    fn test_example_input() {
        let contents = read_file("src/test.txt").unwrap();
        let (mut v, mut v2) = generate_vectors(contents).unwrap();
        let distance = sort_and_compare_distances(&mut v, &mut v2);

        assert_eq!(distance, 11);
    }

    #[test]
    fn test_example_similarity() {
        let contents = read_file("src/test.txt").unwrap();
        let (v, v2) = generate_vectors(contents).unwrap();
        let similarity_distance = calcuate_similiarity_distance(&v, &v2);

        assert_eq!(similarity_distance, 31);
    }
}
