use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a vertical spread option strategy.
///
/// A vertical spread involves buying and selling options with the same expiration date but different strike prices.
/// It can be a bull spread (using calls) or a bear spread (using puts).
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the long option.
/// - `k2`: The strike price of the short option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the options (in years).
/// - `is_bull`: A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
pub struct VerticalSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the long option.
    pub params_long: OptionParameters,

    /// The parameters for the short option.
    pub params_short: OptionParameters,

    /// A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
    pub is_bull: bool,
}

impl<'a, T: OptionPricingModel> VerticalSpread<'a, T> {
    /// Creates a new `VerticalSpread` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params_long` - The parameters for the long option.
    /// * `params_short` - The parameters for the short option.
    /// * `is_bull` - A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
    ///
    /// # Returns
    ///
    /// Returns a new instance of `VerticalSpread`.
    pub fn new(
        model: &'a T,
        params_long: OptionParameters,
        params_short: OptionParameters,
        is_bull: bool,
    ) -> Self {
        Self {
            model,
            params_long,
            params_short,
            is_bull,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for VerticalSpread<'a, T> {
    /// Calculates the price of the vertical spread strategy.
    ///
    /// For a bull spread (using calls), it calculates the difference between the call prices with different strike prices.
    /// For a bear spread (using puts), it calculates the difference between the put prices with different strike prices.
    ///
    /// # Returns
    ///
    /// Returns the net cost of the vertical spread.
    ///
    /// # Example
    ///
    /// use core::models::{BlackScholesModel, OptionParameters};
    /// use core::strategies::VerticalSpread;
    /// let model = BlackScholesModel;
    /// let params_long = OptionParameters {
    ///     s: 100.0,
    ///     k: 105.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let params_short = OptionParameters {
    ///     s: 100.0,
    ///     k: 110.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let vertical_spread = VerticalSpread::new(&model, params_long, params_short, true);
    /// let spread_price = vertical_spread.price();
    /// println!("Vertical Spread Price: {}", spread_price);
    fn price(&self) -> f64 {
        if self.is_bull {
            // Bull spread using calls: long call and short call with different strike prices
            let call_price_long = self.model.call_price(&self.params_long);
            let call_price_short = self.model.call_price(&self.params_short);
            call_price_long - call_price_short
        } else {
            // Bear spread using puts: long put and short put with different strike prices
            let put_price_long = self.model.put_price(&self.params_long);
            let put_price_short = self.model.put_price(&self.params_short);
            put_price_long - put_price_short
        }
    }
}
