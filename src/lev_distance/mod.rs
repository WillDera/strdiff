use std::cmp::min;

pub fn entry(source: Vec<String>, dest: Vec<String>) -> Result<u8, String> {
    if source.is_empty() || dest.is_empty() {
        return Err(String::from("Incomplete params"));
    };

    let result = if source.len() == 1 && dest.len() == 1 {
        Ok(levdist(source[0].clone(), dest[0].clone()))
    } else if (source.len() > 1 && dest.len() > 1) && (source.len() == dest.len()) {
        todo!()
    } else {
        Err(String::from("Param length mismatch"))
    };

    result
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
