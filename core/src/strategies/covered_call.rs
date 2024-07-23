use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct CoveredCall<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
}

impl<'a, T: OptionPricingModel> CoveredCall<'a, T> {
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> Self {
        Self {
            model,
            s,
            k,
            r,
            sigma,
            t,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for CoveredCall<'a, T> {
    fn price(&self) -> f64 {
        let call_price = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t);
        self.s - call_price
    }
}
