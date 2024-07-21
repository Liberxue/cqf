pub mod butterfly;
pub mod single_leg;
pub mod vertical;
pub mod covered_call;
pub mod collar;
pub mod straddle;
pub mod strangle;
pub mod calendar;
pub mod diagonal;
pub mod condor;
pub mod iron_butterfly;
pub mod iron_condor;
pub mod dance;

pub trait OptionStrategy {
    fn price(&self) -> f64;
}

