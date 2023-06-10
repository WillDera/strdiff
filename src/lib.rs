//! Strdiff is a library for measuring edit distance between two or more sequence of strings using
//! string metrics such as Damerau-Levenshtein Distance and Levenshtein Distance
//!
//! This interface implements endpoints defined in strdiff.wai
//!
//!
wai_bindgen_rust::export!("strdiff.wai");
use crate::strdiff::{Error, Inputs, Outputs};
mod demerau_lev_distance;
mod lev_distance;
use demerau_lev_distance::Dlv;
use lev_distance::Lv;

pub struct Strdiff;

impl strdiff::Strdiff for Strdiff {
    fn lvd(a: Inputs, b: Inputs) -> Result<Outputs, Error> {
        match (a, b) {
            (Inputs::String(a), Inputs::String(b)) => {
                let strdiff = lev_distance::LevStrdiff;
                let args = (a, b);
                let result = strdiff.entry(args);

                Ok(Outputs::U8(result))
            }
            (Inputs::StringList(a), Inputs::StringList(b)) => {
                let strdiff = lev_distance::LevStrdiff;
                let args = (a, b);
                let result = strdiff.entry(args).unwrap();

                Ok(Outputs::U8List(result))
            }
            _ => Err("Error: source and target should be of same data type".to_string()),
        }
    }
    fn dlvd(a: Inputs, b: Inputs) -> Result<Outputs, Error> {
        match (a, b) {
            (Inputs::String(a), Inputs::String(b)) => {
                let strdiff = demerau_lev_distance::DemerauStrdiff;
                let args = (a, b);
                let result = strdiff.entry(args);

                Ok(Outputs::U8(result))
            }
            (Inputs::StringList(a), Inputs::StringList(b)) => {
                let strdiff = demerau_lev_distance::DemerauStrdiff;
                let args = (a, b);
                let result = strdiff.entry(args);

                Ok(Outputs::U8List(result))
            }
            _ => Err("Error: source and target should be of same data type".to_string()),
        }
    }
}
