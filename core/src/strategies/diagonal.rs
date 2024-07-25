use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a `DiagonalSpread` option strategy.
///
/// The `DiagonalSpread` strategy involves buying and selling call options with different strike prices and different expiration dates.
/// This strategy aims to benefit from the changes in the underlying asset's price and time decay, and is commonly used to capture price movements while managing risk.
pub struct DiagonalSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call options.
    pub model: &'a T,

    /// The parameters for the short (near-term) call option.
    pub near_params: OptionParameters,

    /// The parameters for the long (far-term) call option.
    pub far_params: OptionParameters,
}

impl<'a, T: OptionPricingModel> DiagonalSpread<'a, T> {
    /// Creates a new `DiagonalSpread` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `near_params` - The parameters for the short (near-term) call option.
    /// * `far_params` - The parameters for the long (far-term) call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `DiagonalSpread`.
    pub fn new(model: &'a T, near_params: OptionParameters, far_params: OptionParameters) -> Self {
        Self {
            model,
            near_params,
            far_params,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for DiagonalSpread<'a, T> {
    /// Calculates the price of the `DiagonalSpread` option strategy.
    ///
    /// The `DiagonalSpread` strategy is composed of two legs:
    /// - A short (near-term) call option with strike price `k1` and time to maturity `t1`.
    /// - A long (far-term) call option with strike price `k2` and time to maturity `t2`.
    ///
    /// The price of the strategy is the difference between the price of the far-term call option and the near-term call option:
    ///
    /// \[
    /// \text{Price} = C_{\text{far}} - C_{\text{near}}
    /// \]
    ///
    /// Where:
    /// - \( C_{\text{far}} \) is the price of the long (far-term) call option with strike price `k2`,
    /// - \( C_{\text{near}} \) is the price of the short (near-term) call option with strike price `k1`.
    ///
    /// # Returns
    ///
    /// Returns the price of the `DiagonalSpread` option strategy.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::DiagonalSpread;
    /// let model = BlackScholesModel;
    /// let near_params = OptionParameters {
    ///     s: 100.0,
    ///     k: 90.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.1,
    /// };
    /// let far_params = OptionParameters {
    ///     s: 100.0,
    ///     k: 110.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let diagonal_spread = DiagonalSpread::new(&model, near_params, far_params);
    /// let price = diagonal_spread.price();
    /// println!("Diagonal Spread Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the near-term (short) call option.
        let near_leg = self.model.call_price(&self.near_params);

        // Calculate the price of the far-term (long) call option.
        let far_leg = self.model.call_price(&self.far_params);

        // The total price of the Diagonal Spread strategy is the difference between the long and short call option prices.
        far_leg - near_leg
    }
}
