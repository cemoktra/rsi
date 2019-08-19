use crate::value::Unit;

// ========================================
// Area module
// ========================================
#[derive(Copy, Clone)]
pub enum AreaUnit {
    SquareMillimeter,
    SquareCentimeter,
    SquareMeter,
    SquareKilometer
}

impl Unit for AreaUnit {
    type UnitEnum = AreaUnit;

    fn ratio(&self) -> f64
    {
        match self {
            AreaUnit::SquareKilometer => 1000000.0,
            AreaUnit::SquareMeter => 1.0,
            AreaUnit::SquareCentimeter => 0.0001,
            AreaUnit::SquareMillimeter => 0.000001
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            AreaUnit::SquareKilometer => String::from("km²"),
            AreaUnit::SquareMeter => String::from("m²"),
            AreaUnit::SquareCentimeter => String::from("cm²"),
            AreaUnit::SquareMillimeter => String::from("mm²")
            }
    }
}