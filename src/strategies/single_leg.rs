use crate::models::OptionPricingModel;

pub struct SingleLegOption;

impl SingleLegOption {
    pub fn call(model: &impl OptionPricingModel, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        model.call_price(s, k, r, sigma, t)
    }

    pub fn put(model: &impl OptionPricingModel, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        model.put_price(s, k, r, sigma, t)
    }
}

