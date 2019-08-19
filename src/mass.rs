use crate::value::{Unit};

// ========================================
// Mass module
// ========================================
#[derive(Copy, Clone)]
pub enum MassUnit {
    Kilogram,
    Gram,
}

impl Unit for MassUnit {
    type UnitEnum = MassUnit;

    fn ratio(&self) -> f64
    {
        match self {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram => 0.001,
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            MassUnit::Kilogram => String::from("kg"),
            MassUnit::Gram => String::from("g"),
        }
    }
}