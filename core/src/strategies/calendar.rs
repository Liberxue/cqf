use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct CalendarSpread<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t1: f64,
    pub t2: f64,
}

impl<'a, T: OptionPricingModel> CalendarSpread<'a, T> {
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t1: f64, t2: f64) -> Self {
        Self {
            model,
            s,
            k,
            r,
            sigma,
            t1,
            t2,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for CalendarSpread<'a, T> {
    fn price(&self) -> f64 {
        let near_leg = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t1);
        let far_leg = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t2);
        far_leg - near_leg
    }
}
