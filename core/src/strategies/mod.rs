pub mod butterfly;
pub mod calendar;
pub mod collar;
pub mod condor;
pub mod covered_call;
pub mod dance;
pub mod diagonal;
pub mod iron_butterfly;
pub mod iron_condor;
pub mod single_leg;
pub mod straddle;
pub mod strangle;
pub mod vertical;

pub trait OptionStrategy {
    fn price(&self) -> f64;
}
