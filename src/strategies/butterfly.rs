use crate::models::black_scholes::BlackScholesModel;

pub struct ButterflySpread;

impl ButterflySpread {
    pub fn price(s: f64, k1: f64, k2: f64, k3: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let c1 = BlackScholesModel::call(s, k1, r, sigma, t);
        let c2 = BlackScholesModel::call(s, k2, r, sigma, t);
        let c3 = BlackScholesModel::call(s, k3, r, sigma, t);
        c1 - 2.0 * c2 + c3
    }
}
