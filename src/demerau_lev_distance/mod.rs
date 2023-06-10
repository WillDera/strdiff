//! Damerau-Levenshtein distance of strings or vector of strings
//!
//! Compute the string difference under the condition that a substring is
//! not edited more than once (Optimal string alignment distance)
//!
//! Algorithm gotten from wikipedia:
//! https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance (slightly modified)
//!
//! See tests (line 111) for example code
//!
use std::collections::HashMap;

#[derive(Debug)]
pub struct DemerauStrdiff;

/// Function overloading achieved with traits
pub trait Dlv<Args> {
    type Output;
    fn entry(&self, args: Args) -> Self::Output;
}

impl Dlv<(String, String)> for DemerauStrdiff {
    type Output = u8;

    fn entry(&self, args: (String, String)) -> Self::Output {
        demeraudist(args.0, args.1)
    }
}

impl Dlv<(Vec<String>, Vec<String>)> for DemerauStrdiff {
    // type Output = Result<Vec<u8>, String>;
    type Output = Vec<u8>;

    fn entry(&self, args: (Vec<String>, Vec<String>)) -> Self::Output {
        demeraudist_vec(args.0, args.1)
    }
}

/// Calculate string difference between vector of strings by iteratively executing `demeraudist()` against
/// each item in source and dest with respect to position. eg source[0] & dest[0]  
fn demeraudist_vec(source: Vec<String>, dest: Vec<String>) -> Vec<u8> {
    let result: Vec<u8> = source
        .iter()
        .zip(dest.iter())
        .map(|(s, d)| demeraudist(s.to_string(), d.to_string()))
        .collect();

    result
}

/// Calculate string difference between strings
fn demeraudist(source: String, dest: String) -> u8 {
    let source_len = source.len();
    let dest_len = dest.len();
    let mut mem: HashMap<(i32, i32), usize> = HashMap::new();

    for i in -1..=(source_len as i32) {
        mem.insert((i, -1), (i + 1) as usize);
    }

    for j in -1..=(dest_len as i32) {
        mem.insert((-1, j), (j + 1) as usize);
    }
    // helper(a_bytes, b_bytes, &mut da, &mut d, source.len(), dest.len()) as u8
    helper(
        (source_len - 1) as i32,
        (dest_len - 1) as i32,
        source.as_str(),
        dest.as_str(),
        &mut mem,
    ) as u8
}

fn helper(i: i32, j: i32, s1: &str, s2: &str, d: &mut HashMap<(i32, i32), usize>) -> usize {
    if let Some(&v) = d.get(&(i, j)) {
        return v;
    };

    let (deletion, insertion, substitution) = (
        helper(i - 1, j, s1, s2, d) + 1,
        helper(i, j - 1, s1, s2, d) + 1,
        helper(i - 1, j - 1, s1, s2, d)
            + if s1.chars().nth(i as usize) == s2.chars().nth(j as usize) {
                0
            } else {
                1
            },
    );
    let distance = std::cmp::min(std::cmp::min(deletion, insertion), substitution);

    if i > 0
        && j > 0
        && s1.chars().nth(i as usize) == s2.chars().nth((j - 1) as usize)
        && s1.chars().nth((i - 1) as usize) == s2.chars().nth(j as usize)
    {
        let transposition = helper(i - 2, j - 2, s1, s2, d)
            + if s1.chars().nth(i as usize) == s2.chars().nth((j - 1) as usize)
                && s1.chars().nth((i - 1) as usize) == s2.chars().nth(j as usize)
            {
                1
            } else {
                2
            };
        d.insert((i, j), std::cmp::min(distance, transposition));
    } else {
        d.insert((i, j), distance);
    }
    d.get(&(i, j)).unwrap().clone()
}

#[test]
fn test_demeraudist() {
    let initial = demeraudist("Hello".to_string(), "geHellio".to_string());
    let second = demeraudist("Hello".to_string(), "Halla".to_string());
    let third = demeraudist("Abdul Hasan".to_string(), "Abdil Husain".to_string());

    assert_eq!(initial, 3);
    assert_eq!(second, 2);
    assert_eq!(third, 3);
}

#[test]
fn test_dlv() {
    let entry_struct = DemerauStrdiff;
    let values = ("Zedicus Zul".to_string(), "Zedicus zUL".to_string());
    let result_str = entry_struct.entry(values);

    let values_vec = (
        vec!["Zedicus".to_string(), "Xander".to_string()],
        vec!["zeDIcsu".to_string(), "xanred".to_string()],
    );
    let result_vec = entry_struct.entry(values_vec);

    assert_eq!(result_str, 3);
    assert_eq!(result_vec, vec![4, 3]);
}
