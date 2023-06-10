wai_bindgen_rust::export!("strdiff.wai");
// use std::any::{Any, TypeId};
use crate::strdiff::{Error, Inputs, Outputs};
mod demerau_lev_distance;
mod lev_distance;
// use crate::lev_distance::LevStrdiff;
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
            _ => Err("Error: source and target should have same data type".to_string()),
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
            _ => Err("Something went wrong".to_string()),
        }
    }
}
