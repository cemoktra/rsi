//#[crate_id = "si_units"];
//#[crate_type = "lib"];

pub mod value;

// base si units
pub mod length;
pub mod time;
pub mod mass;
//pub mod electric_current;
//pub mod temperature;
//pub mod substance_amount;
//pub mod luminosity;

// derived si unites
pub mod area;
pub mod volume;
pub mod velocity;

// calculations
pub mod calc;