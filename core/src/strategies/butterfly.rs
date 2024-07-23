use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct ButterflySpread<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k1: f64,
    pub k2: f64,
    pub k3: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
}

impl<'a, T: OptionPricingModel> ButterflySpread<'a, T> {
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        k3: f64,
        r: f64,
        sigma: f64,
        t: f64,
    ) -> Self {
        Self {
            model,
            s,
            k1,
            k2,
            k3,
            r,
            sigma,
            t,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for ButterflySpread<'a, T> {
    fn price(&self) -> f64 {
        let c1 = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t);
        let c2 = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);
        let c3 = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);
        c1 - 2.0 * c2 + c3
    }
}
