use std::cmp::min;
use std::fmt::Debug;

#[derive(Debug)]
struct Strdiff;

trait Lv<Args> {
    type Output;
    fn entry(&self, args: Args) -> Self::Output;
}

impl Lv<(&str, &str)> for Strdiff {
    type Output = u8;

    fn entry(&self, args: (&str, &str)) -> Self::Output {
        levdist(args.0.to_string(), args.1.to_string())
    }
}

impl Lv<(Vec<&str>, Vec<&str>)> for Strdiff {
    type Output = Result<Vec<u8>, String>;

    fn entry(&self, args: (Vec<&str>, Vec<&str>)) -> Self::Output {
        if args.0.is_empty() || args.1.is_empty() {
            return Err("Incomplete params".to_string());
        };

        if args.0.len() == 1 && args.1.len() == 1 {
            let result = levdist(args.0[0].to_string(), args.1[0].to_string());

            return Ok(vec![result]);
        } else if (args.0.len() > 1 && args.1.len() > 1) && (args.0.len() == args.1.len()) {
            todo!()
        } else {
            return Err(String::from("Param length mismatch"));
        };
    }
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
fn test_entry() {
    let source = vec!["hello".to_string(), "world".to_string()];
    let dest = vec!["foo".to_string()];

    let result = entry(source, dest);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Param length mismatch");
}

#[test]
fn test_strdiff() {
    let test = Strdiff;
    let args = ("hello", "hella");
    let result = test.entry(args);

    let vargs = (vec!["hello"], vec!["hal"]);
    let vresult = test.entry(vargs);

    assert_eq!(result, 1);
    assert_eq!(vresult.unwrap(), vec![3]);
}
