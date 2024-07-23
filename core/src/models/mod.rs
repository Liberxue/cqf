pub mod black_scholes;
pub mod monte_carlo;

pub trait OptionPricingModel {
    fn call_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64;
    fn put_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64;
}
