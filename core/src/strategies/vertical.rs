use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

pub struct VerticalSpread<'a, T: OptionPricingModel> {
    pub model: &'a T,
    pub s: f64,
    pub k1: f64,
    pub k2: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
    pub is_bull: bool,
}

impl<'a, T: OptionPricingModel> VerticalSpread<'a, T> {
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        r: f64,
        sigma: f64,
        t: f64,
        is_bull: bool,
    ) -> Self {
        Self {
            model,
            s,
            k1,
            k2,
            r,
            sigma,
            t,
            is_bull,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for VerticalSpread<'a, T> {
    fn price(&self) -> f64 {
        if self.is_bull {
            let c1 = self
                .model
                .call_price(self.s, self.k1, self.r, self.sigma, self.t);
            let c2 = self
                .model
                .call_price(self.s, self.k2, self.r, self.sigma, self.t);
            c1 - c2
        } else {
            let p1 = self
                .model
                .put_price(self.s, self.k1, self.r, self.sigma, self.t);
            let p2 = self
                .model
                .put_price(self.s, self.k2, self.r, self.sigma, self.t);
            p1 - p2
        }
    }
}
