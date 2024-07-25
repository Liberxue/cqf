use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a strangle option strategy.
///
/// A strangle involves buying a call and a put option with different strike prices but the same expiration date.
/// This structure calculates the combined price of the call and put options using the provided option pricing model.
pub struct Strangle<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the call option.
    pub params_call: OptionParameters,

    /// The parameters for the put option.
    pub params_put: OptionParameters,
}

impl<'a, T: OptionPricingModel> Strangle<'a, T> {
    /// Creates a new `Strangle` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params_call` - The parameters for the call option.
    /// * `params_put` - The parameters for the put option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Strangle`.
    pub fn new(model: &'a T, params_call: OptionParameters, params_put: OptionParameters) -> Self {
        Self {
            model,
            params_call,
            params_put,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for Strangle<'a, T> {
    /// Calculates the price of the strangle option strategy.
    ///
    /// The price of the strangle is the sum of the prices of the call and put options with different strike prices but the same expiration date.
    ///
    /// # Returns
    ///
    /// Returns the combined price of the call and put options.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::Strangle;
    /// let model = BlackScholesModel;
    /// let params_call = OptionParameters {
    ///     s: 100.0,
    ///     k: 110.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let params_put = OptionParameters {
    ///     s: 100.0,
    ///     k: 90.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let strangle = Strangle::new(&model, params_call, params_put);
    /// let strangle_price = strangle.price();
    /// println!("Strangle Price: {}", strangle_price);
    fn price(&self) -> f64 {
        let call_price = self.model.call_price(&self.params_call);
        let put_price = self.model.put_price(&self.params_put);
        call_price + put_price
    }
}
