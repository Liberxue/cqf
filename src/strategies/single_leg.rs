use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct SingleLegOption<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
    pub is_call: bool,
}

impl<'a, T: OptionPricingModel> SingleLegOption<'a, T> {
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t: f64, is_call: bool) -> Self {
        Self { model, s, k, r, sigma, t, is_call }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for SingleLegOption<'a, T> {
    fn price(&self) -> f64 {
        if self.is_call {
            self.model.call_price(self.s, self.k, self.r, self.sigma, self.t)
        } else {
            self.model.put_price(self.s, self.k, self.r, self.sigma, self.t)
        }
    }
}

