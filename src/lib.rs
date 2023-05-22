wai_bindgen_rust::export!("strdiff.wai");
mod lev_distance;

pub struct Strdiff;

impl strdiff::Strdiff for Strdiff {
    fn lv(_a: Vec<String>, _b: Vec<String>) -> u8 {
        todo!()
    }
}

// impl strdiff::Strdiff for Strdiff {
//     fn lv(_a: Vec<String>, _b: Vec<&str>)
// }
