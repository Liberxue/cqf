use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct Straddle<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Straddle<'a, T> {
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> Self {
        Self { model, s, k, r, sigma, t }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for Straddle<'a, T> {
    fn price(&self) -> f64 {
        let call_price = self.model.call_price(self.s, self.k, self.r, self.sigma, self.t);
        let put_price = self.model.put_price(self.s, self.k, self.r, self.sigma, self.t);
        call_price + put_price
    }
}

