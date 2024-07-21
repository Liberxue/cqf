use crate::models::black_scholes::BlackScholesModel;

pub struct SingleLegOption;

impl SingleLegOption {
    pub fn call(s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        BlackScholesModel::call(s, k, r, sigma, t)
    }

    pub fn put(s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        BlackScholesModel::put(s, k, r, sigma, t)
    }
}
