use std::ascii::AsciiExt;
use std::cmp::*;

macro_rules! chars {
    ($str:expr) => (
        &$str.chars().collect::<Vec<char>>()
    );
}

pub fn compute_match<'a>(samples: &[&'a str], query: &str) -> Vec<&'a str> {
    single_thread(samples, query)
}

fn single_thread<'a>(samples: &[&'a str], query: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for (index, sample) in samples.iter().enumerate() {
        let sample_score = score(sample, query);
        if sample_score > 0_f64 {
            result.push((index, sample_score));
        }
    }

    result.sort_by(|&(_, a), &(_, b)| b.partial_cmp(&a).unwrap());
    result.iter().map(|&(index, _)| samples[index]).collect()
}

fn score(sample: &str, query: &str) -> f64 {
    if query.is_empty() {
        return 1_f64;
    }
    if sample.is_empty() {
        return 0_f64;
    }

    let query = chars!(query);
    let sample = chars!(sample);

    match compute_longest_match_length(sample, query) {
        None => 0_f64,
        Some(length) => {
            let score: f64 = query.len() as f64 / length as f64;
            score / sample.len() as f64
        }
    }
}

fn compute_longest_match_length(string: &[char], query: &[char]) -> Option<usize> {
    get_length_of_match(get_longest_match(string, query))
}

fn get_longest_match(string: &[char], query: &[char]) -> Option<(usize, usize)> {
    let first_char_of_query = query[0];
    let remaining_chars_of_query = &query[1..];
    let all_indices_of_query = find_char_in_string(string, first_char_of_query);
    let smallest_possible_match_length = query.len();
    let mut longest_match: Option<(usize, usize)> = None;

    for &index_of_query in &all_indices_of_query {
        if let Some(last_index_of_match) = find_end_of_match(string, remaining_chars_of_query, index_of_query) {
            let bounds = Some((index_of_query, last_index_of_match));
            let match_length = get_length_of_match(bounds).unwrap();
            if longest_match.is_none() || match_length < get_length_of_match(longest_match).unwrap() {
                longest_match = bounds;
                if match_length == smallest_possible_match_length {
                    break;
                }
            }
        }
    }
    longest_match
}

fn get_length_of_match(bounds: Option<(usize, usize)>) -> Option<usize> {
    match bounds {
        Some((lb, ub)) => Some(ub - lb + 1),
        None => None
    }
}

fn find_end_of_match(string: &[char], query: &[char], first_index: usize) -> Option<usize> {
    match get_match_indices(string, query, first_index) {
        Some(indices) => match indices.len() {
            0 => None,
            n => Some(indices[n - 1])
        },
        None => None
    }
}

fn get_match_indices(string: &[char], query: &[char], first_index: usize) -> Option<Vec<usize>> {
    let mut result = Vec::new();
    result.push(first_index);

    let mut last_index = first_index + 1;

    for char_of_query in query.iter() {
        let this_substring = &string[last_index..];

        match this_substring.iter()
                .enumerate()
                .filter(|&(_, el)| chars_equal(&char_of_query, el))
                .map(|(i, _)| i)
                .next() {
            Some(index) => {
                last_index += index + 1;
                result.push(last_index - 1);
            },
            None => {
                return None;
            }
        }
    }
    Some(result)
}

fn find_char_in_string(string: &[char], char: char) -> Vec<usize> {
    string.iter()
        .enumerate()
        .filter(|&(_, el)| chars_equal(&char, el))
        .map(|(i, _)| i)
        .collect()
}

fn chars_equal(a: &char, b: &char) -> bool {
    a == b || *a == b.to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::{chars_equal, find_char_in_string, get_match_indices, find_end_of_match, score, single_thread};

    #[test]
    fn chars_equal_test() {
        assert!(chars_equal(&'a', &'a'));
        assert!(chars_equal(&'a', &'A'));
        assert!(chars_equal(&'A', &'A'));
        assert!(!chars_equal(&'b', &'a'));
    }

    #[test]
    fn find_char_in_string_test() {
        assert_eq!(find_char_in_string(chars!("00100"), '1'), vec![2]);
        assert_eq!(find_char_in_string(chars!("10100"), '1'), vec![0, 2]);
        assert_eq!(find_char_in_string(chars!("00100"), '2'), vec![]);
    }

    #[test]
    fn get_match_indices_test() {
        assert_eq!(get_match_indices(chars!("asdf"), chars!("sdf"), 0).unwrap(), vec![0, 1, 2, 3]);
        assert_eq!(get_match_indices(chars!("000"), chars!("12"), 0), None);
    }

    #[test]
    fn find_end_of_match_test() {
        assert_eq!(find_end_of_match(chars!("a"), chars!("a"), 0), None);
        assert_eq!(find_end_of_match(chars!("ba"), chars!("a"), 1), None);
        assert_eq!(find_end_of_match(chars!("aaa"), chars!("aa"), 0), Some(2));
        assert_eq!(find_end_of_match(chars!("aaa"), chars!("b"), 0), None);
        assert_eq!(find_end_of_match(chars!("this is a long match"), chars!("this is a match"), 0), None);
        assert_eq!(find_end_of_match(chars!("this is a long match"), chars!("his is a match"), 0), Some(19));
    }

    #[test]
    fn basic_scoring() {
        assert_eq!(score("", "a"), 0.0);
        assert_eq!(score("a", ""), 1.0);
        assert_eq!(score("short", "longer"), 0.0);
        assert_eq!(score("a", "b"), 0.0);
        assert_eq!(score("ab", "ac"), 0.0);

        assert!(score("a", "a") > 0.0);
        assert!(score("ab", "a") > 0.0);
        assert!(score("ba", "a") > 0.0);
        assert!(score("bab", "a") > 0.0);
        assert!(score("babababab", "aaaa") > 0.0);

        assert_eq!(score("a", "a"), 1_f64 / "a".len() as f64);
        assert_eq!(score("ab", "ab"), 0.5);
        assert_eq!(score("a long string", "a long string"), 1_f64 / "a long string".len() as f64);
        assert_eq!(score("spec/search_spec.rb", "sear"), 1_f64 / "spec/search_spec.rb".len() as f64);
    }

    #[test]
    fn character_matching() {
        assert!(score("/! symbols $^", "/!$^") > 0.0);

        assert_eq!(score("a", "A"), 0.0);
        assert_eq!(score("A", "a"), 1.0);
        assert_eq!(score("A", "A"), 1.0);

        assert_eq!(score("a", "aa"), 0.0);
    }

    #[test]
    fn match_equality() {
        assert!(score("selecta.gemspec", "asp") > score("algorithm4_spec.rb", "asp"));
        assert!(score("README.md", "em") > score("benchmark.rb", "em"));
        assert!(score("search.rb", "sear") > score("spec/search_spec.rb", "sear"));

        assert!(score("fbb", "fbb") > score("foo bar baz", "fbb"));
        assert!(score("foo", "foo") > score("longer foo", "foo"));
        assert!(score("foo", "foo") > score("foo longer", "foo"));
        assert!(score("1/2/3/4", "1/2/3") > score("1/9/2/3/4", "1/2/3"));

        assert!(score("long 12 long", "12") > score("1 long 2", "12"));

        assert_eq!(score("121padding2", "12"), 1.0 / "121padding2".len() as f64);
        assert_eq!(score("1padding212", "12"), 1.0 / "1padding212".len() as f64);
    }

    #[test]
    fn single_thread_test() {
        println!("{:?}", single_thread(&[&"abcd", &"bcde"], "bcd"));
    }
}
