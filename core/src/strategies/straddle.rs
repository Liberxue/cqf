use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a straddle option strategy.
///
/// A straddle involves buying both a call and a put option with the same strike price and expiration date.
/// This structure calculates the combined price of the call and put options using the provided option pricing model.
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k`: The strike price of both the call and put options.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the options (in years).
pub struct Straddle<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the options.
    pub params: OptionParameters,
}

impl<'a, T: OptionPricingModel> Straddle<'a, T> {
    /// Creates a new `Straddle` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params` - The parameters for the options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Straddle`.
    pub fn new(model: &'a T, params: OptionParameters) -> Self {
        Self { model, params }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for Straddle<'a, T> {
    /// Calculates the price of the straddle option strategy.
    ///
    /// The price of the straddle is the sum of the prices of the call and put options with the same strike price and expiration date.
    ///
    /// # Returns
    ///
    /// Returns the combined price of the call and put options.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::Straddle;
    /// let model = BlackScholesModel;
    /// let params = OptionParameters {
    ///     s: 100.0,
    ///     k: 100.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let straddle = Straddle::new(&model, params);
    /// let straddle_price = straddle.price();
    /// println!("Straddle Price: {}", straddle_price);
    fn price(&self) -> f64 {
        let call_price = self.model.call_price(&self.params);
        let put_price = self.model.put_price(&self.params);
        call_price + put_price
    }
}
