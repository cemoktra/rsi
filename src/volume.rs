use crate::value::Unit;

// ========================================
// Volume module
// ========================================
#[derive(Copy, Clone)]
pub enum VolumeUnit {
    CubicMillimeter,
    CubicCentimeter,
    CubicMeter,
    CubicKilometer
}

impl Unit for VolumeUnit {
    type UnitEnum = VolumeUnit;

    fn ratio(&self) -> f64
    {
        match self {
            VolumeUnit::CubicKilometer => 1000000000.0,
            VolumeUnit::CubicMeter => 1.0,
            VolumeUnit::CubicCentimeter => 0.000001,
            VolumeUnit::CubicMillimeter => 0.000000001
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            VolumeUnit::CubicKilometer => String::from("km³"),
            VolumeUnit::CubicMeter => String::from("m³"),
            VolumeUnit::CubicCentimeter => String::from("cm³"),
            VolumeUnit::CubicMillimeter => String::from("mm³")
        }
    }
}