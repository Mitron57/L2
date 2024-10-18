use std::collections::{HashMap, HashSet};

fn find_anagrams(words: &[&str]) -> HashMap<String, Vec<String>> {
    let mut anagrams = HashMap::new();
    for &word in words {
        let lowered = word.to_lowercase();
        let mut sorted = lowered.chars().collect::<Vec<char>>();
        sorted.sort_unstable();
        let sorted = sorted.iter().collect::<String>();
        anagrams.entry(sorted).or_insert(Vec::new()).push(lowered);
    }
    anagrams
        .into_iter()
        .flat_map(|(_, group)| {
            if group.len() > 1 {
                Some((group[0].clone(), group))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let words = vec![
        "пятак",
        "пятка",
        "тяпка",
        "листок",
        "слиток",
        "столик",
        "apple",
    ];
    let anagrams = find_anagrams(&words);
    for (key, group) in &anagrams {
        println!("{}: {:?}", key, group);
    }
}
