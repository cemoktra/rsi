pub trait ValueTrait<T> {
    fn value(&self) -> f64;
    fn base_value(&self) -> f64;
    fn unit(&self) -> T;
    fn unit_string(&self) -> String;
    fn convert(&self, unit: T) -> Value<T>;
}

#[derive(Copy, Clone)]
pub struct Value<T> {
    pub unit: T,
    pub value: f64,
}