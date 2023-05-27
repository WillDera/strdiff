use std::cmp::min;
use std::fmt::Debug;

#[derive(Debug)]
pub struct LevStrdiff;

pub trait Lv<Args> {
    type Output;
    fn entry(&self, args: Args) -> Self::Output;
}

impl Lv<(String, String)> for LevStrdiff {
    type Output = u8;

    fn entry(&self, args: (String, String)) -> Self::Output {
        levdist(args.0, args.1)
    }
}

impl Lv<(Vec<String>, Vec<String>)> for LevStrdiff {
    type Output = Result<Vec<u8>, String>;

    fn entry(&self, args: (Vec<String>, Vec<String>)) -> Self::Output {
        if args.0.is_empty() || args.1.is_empty() {
            return Err("Incomplete params".to_string());
        };

        if args.0.len() == 1 && args.1.len() == 1 {
            let result = levdist(args.0[0].clone(), args.1[0].clone());

            return Ok(vec![result]);
        } else if (args.0.len() > 1 && args.1.len() > 1) && (args.0.len() == args.1.len()) {
            let result = levdist_vec(args.0, args.1);

            return Ok(result);
        } else {
            return Err(String::from("Param length mismatch"));
        };
    }
}

fn levdist_vec(source: Vec<String>, dest: Vec<String>) -> Vec<u8> {
    let results: Vec<u8> = source
        .iter()
        .zip(dest.iter())
        .map(|(s, d)| levdist(s.to_string(), d.to_string()))
        .collect();

    results
}

fn levdist(source: String, dest: String) -> u8 {
    if source.is_empty() {
        return dest.len() as u8;
    };

    if dest.is_empty() {
        return source.len() as u8;
    };

    if source[..1] == dest[..1] {
        return levdist(source[1..].to_string(), dest[1..].to_string());
    };

    let min_init = min(
        levdist(source[1..].to_string(), dest.to_string()),
        levdist(source.to_string(), dest[1..].to_string()),
    );

    1 + min(
        min_init,
        levdist(source[1..].to_string(), dest[1..].to_string()),
    )
}

#[test]
fn test_levdist() {
    let initial = levdist("Hello".to_string(), "Hello".to_string());
    let second = levdist("Hello".to_string(), "Halla".to_string());
    let third = levdist("Abdul Hasan".to_string(), "Abdil Husain".to_string());

    assert_eq!(initial, 0);
    assert_eq!(second, 2);
    assert_eq!(third, 3);
}

#[test]
fn test_levdist_vec() {
    let test = LevStrdiff;
    let args = ("hello".to_string(), "hella".to_string());
    let result = test.entry(args);

    let vargs = (vec!["hello".to_string()], vec!["hal".to_string()]);
    let vresult = test.entry(vargs);

    assert_eq!(result, 1);
    assert_eq!(vresult.unwrap(), vec![3]);
}
