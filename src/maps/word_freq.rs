use regex::Regex;
use std::fs::File;
use std::io::Read;

// NOTE: Execute file reads from project root (not /src/),
// otherwise a file not found error may occur
pub fn file_reader(file: &str) -> String {
    let mut file_contents = String::new();
    //match File::open("./maps/word-freq-data.txt") {
    //match File::open("../../tech-docs/src/content/docs/cs/dsa/foundations.md") {
    match File::open(file) {
        Ok(mut file) => match file.read_to_string(&mut file_contents) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error opening file: {}", err);
        }
    }
    file_contents
}

/** Parses file contents and returns a list of words */
fn parse_words(input: String) -> Vec<String> {
    let re = Regex::new(r"\b\w+\b").unwrap();

    // Imperative style
    //let mut result = Vec::new();
    //for mat in re.find_iter(&input) {
    //    result.push(mat.as_str().to_string());
    //}
    //result

    // Functional style
    re.find_iter(&input)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

/** Takes a file path as string slice and a number that dictates the size
of the final output lists;
The actual map component processes the list of words and inserts each entry,
incrementing the count when encountering duplicates;
The function then outputs the map back to a list that can be sorted for
different analytical purposes */
pub fn word_freq(file: &str, num: usize) {
    use std::collections::HashMap;

    let contents = file_reader(file);
    let parsed = parse_words(contents);
    let mut map: HashMap<String, u16> = HashMap::new();
    // Map the input
    for w in parsed.iter() {
        let word = w.to_lowercase();
        let count = map.get(&word);
        let new_count = count.unwrap_or(&0) + 1;
        map.insert(word, new_count);
    }
    // Analyze the map
    //let mut max_count = 0;
    //let mut max_word = String::new();
    //for e in map.iter() {
    //    if *e.1 >= max_count {
    //        max_count = *e.1;
    //        max_word = e.0.clone();
    //    }
    //}
    //println!(
    //    "The word \"{}\" occurs the most at {} times",
    //    max_word, max_count
    //);

    // Sort the map by value in descending order
    let mut sorted: Vec<(String, u16)> = map.iter().map(|(k, v)| (k.clone(), *v)).collect();
    sorted.sort_by_key(|&(_, v)| std::cmp::Reverse(v));

    println!(
        "{} total words with {} unique words",
        parsed.len(),
        sorted.len()
    );

    // Print the top 10 sorted key-value pairs (ensure it doesn't go out of bounds)
    for (rank, (word, count)) in sorted.iter().take(num).enumerate() {
        println!("{}: '{}' appears {} times", rank + 1, word, count);
    }
    println!();

    // (Re-)sorts by length (ascending)
    sorted.sort_by_key(|(word, _)| word.len());

    // Prints the last X number of entries
    let total = sorted.len();
    //let start = if total > num { total - num } else { 0 }; // Avoid underflow
    // clippy wants:
    let start = total.saturating_sub(num);

    let mut i = start;
    while i < total {
        println!("{}: '{}' appears {} times", i + 1, sorted[i].0, sorted[i].1);
        i += 1;
    }
}
