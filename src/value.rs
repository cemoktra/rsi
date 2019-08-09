use std::ops::{Add,AddAssign,Sub,SubAssign};
use std::fmt;

pub trait Unit
{
    type UnitEnum;
    fn ratio(&self) -> f64;
    fn abbr(&self) -> String;
}

#[derive(Copy, Clone)]
pub struct Value<T> {
    unit: T,
    value: f64,
}

impl<T:Unit + Copy> Value<T> {
    pub fn new(unit: T, value: f64) -> Value<T>
    {
        Value {
            unit: unit, 
            value: value
        }
    }

    pub fn value(&self) -> f64
    {
        self.value
    }

    pub fn base_value(&self) -> f64
    {
        self.value * self.unit.ratio()
    }
    
    pub fn convert(&self, unit: T) -> Value<T>
    {
        Value::new(unit, (self.value * self.unit.ratio()) / unit.ratio())
    }
}

impl<T: Unit + Copy> Add for Value<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.unit.ratio() > other.unit.ratio()
        {
            Value::new(self.unit, ((self.value * self.unit.ratio()) + (other.value * other.unit.ratio())) / self.unit.ratio())
        } else {
            Value::new(self.unit, ((self.value * self.unit.ratio()) + (other.value * other.unit.ratio())) / other.unit.ratio())
        }
    }
}

impl<T: Unit + Copy> Sub for Value<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.unit.ratio() > other.unit.ratio()
        {
            Value::new(self.unit, ((self.value * self.unit.ratio()) - (other.value * other.unit.ratio())) / self.unit.ratio())
        } else {
            Value::new(self.unit, ((self.value * self.unit.ratio()) - (other.value * other.unit.ratio())) / other.unit.ratio())
        }
    }
}

impl<T: Unit + Copy> AddAssign for Value<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Value::new(self.unit, ((self.value * self.unit.ratio()) + (other.value * other.unit.ratio())) / self.unit.ratio())
    }
}

impl<T: Unit + Copy> SubAssign for Value<T> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value::new(self.unit, ((self.value * self.unit.ratio()) - (other.value * other.unit.ratio())) / self.unit.ratio())
    }
}


impl<T: Unit + Copy>  std::fmt::Display for Value<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), self.unit.abbr())
    }
}
