use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct DemerauStrdiff;

pub trait Dlv<Args> {
    type Output;
    fn entry(&self, args: Args) -> Self::Output;
}

impl Dlv<(String, String)> for DemerauStrdiff {
    type Output = u8;

    fn entry(&self, args: (String, String)) -> Self::Output {
        todo!()
    }
}

impl Dlv<(Vec<String>, Vec<String>)> for DemerauStrdiff {
    type Output = Result<Vec<u8>, String>;

    fn entry(&self, args: (Vec<String>, Vec<String>)) -> Self::Output {
        todo!()
    }
}

fn demeraudist(source: String, dest: String) -> u8 {
    let a_bytes = source.as_bytes();
    let b_bytes = dest.as_bytes();
    let sigma: HashSet<u8> = a_bytes.iter().chain(b_bytes.iter()).copied().collect();
    let mut da: HashMap<u8, usize> = sigma.into_iter().map(|c| (c, 0)).collect();
    let max_dist = source.len() + dest.len();
    let mut d: Vec<Vec<usize>> = vec![vec![max_dist; dest.len() + 2]; source.len() + 2];
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();

    // helper(a_bytes, b_bytes, &mut da, &mut d, source.len(), dest.len()) as u8
    helper(
        a_bytes,
        b_bytes,
        &mut da,
        &mut d,
        source.len(),
        dest.len(),
        &mut memo,
    ) as u8
}

fn helper(
    a: &[u8],
    b: &[u8],
    da: &mut HashMap<u8, usize>,
    d: &mut Vec<Vec<usize>>,
    i: usize,
    j: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&result) = memo.get(&(i, j)) {
        return result;
    }

    let result = if i == 0 && j == 0 {
        a.len() + b.len()
    } else if i == 0 {
        j
    } else if j == 0 {
        i
    } else {
        let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
        let substitution = helper(a, b, da, d, i - 1, j - 1, memo) + cost;
        let insertion = helper(a, b, da, d, i, j - 1, memo) + 1;
        let deletion = helper(a, b, da, d, i - 1, j, memo) + 1;

        let transposition = if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
            helper(a, b, da, d, i - 2, j - 2, memo) + 1
        } else {
            usize::MAX
        };

        substitution.min(insertion).min(deletion).min(transposition)
    };

    memo.insert((i, j), result);
    result
}

#[test]
fn test_demeraudist() {
    let initial = demeraudist("Hello".to_string(), "geHellio".to_string());
    let second = demeraudist("Hello".to_string(), "Halla".to_string());
    let third = demeraudist("Abdul Hasan".to_string(), "Abdil Husain".to_string());

    assert_eq!(initial, 3);
    assert_eq!(second, 4);
    assert_eq!(third, 5);
}
