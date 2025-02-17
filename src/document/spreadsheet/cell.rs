use std::fmt::Display;

use chrono::{DateTime, Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CellType {
    Empty,
    Raw,
    Number,
    StyledNumber(usize),
    Shared(usize),
    Styled(usize),
}
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum CellValue {
    Null,
    Bool(bool),
    Int(i64),
    Byte(u8),
    Double(f64),
    String(String),
    //DateTime(DateTime<Local>),
    DateTime(NaiveDateTime, String),
    Raw(String),
}

impl Display for CellValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {       
        match self {
            CellValue::Null =>  write!(f, "{}", ""),
            CellValue::String(v) =>  write!(f, "{}", v),
            CellValue::Raw(v) => write!(f, "{}", v),
            CellValue::Bool(_v) => panic!("unsupported cell type: bool"),
            CellValue::Double(v) => write!(f, "{}", v),
            CellValue::DateTime(datetime, format) => write!(f, "{}", datetime.format(&format)),
            _ => unimplemented!(),
        }
    }
}

impl Default for CellValue {
    fn default() -> Self {
        CellValue::Null
    }
}

pub trait ToCellValue {
    fn to_cell_value(self) -> CellValue;
}

impl<T: ToCellValue> From<T> for CellValue {
    fn from(v: T) -> CellValue {
        v.to_cell_value()
    }
}
impl<'a, T: Clone + ToCellValue> ToCellValue for &'a T {
    fn to_cell_value(self) -> CellValue {
        self.clone().to_cell_value()
    }
}
macro_rules! impl_to_cell_value {
    ($ty:ty, $target:ident) => {
        paste::paste! {
            impl ToCellValue for $ty {
                fn to_cell_value(self) -> CellValue {
                    CellValue::[<$target>](self as _)
                }
            }
        }
    };
}

impl_to_cell_value!(u8, Byte);
impl_to_cell_value!(i64, Int);
impl_to_cell_value!(i32, Int);
impl_to_cell_value!(u32, Int);
impl_to_cell_value!(u16, Int);
impl_to_cell_value!(i16, Int);
impl_to_cell_value!(i8, Int);
impl_to_cell_value!(bool, Bool);
impl_to_cell_value!(f32, Double);
impl_to_cell_value!(f64, Double);
impl_to_cell_value!(String, String);
