use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a `Dance` option strategy.
///
/// The `Dance` strategy involves creating a combination of call options with different strike prices.
/// It can be used to take advantage of price movements and volatility in the underlying asset by combining
/// multiple call options into a single strategy. This can provide a more complex exposure to the underlying asset's movements.
pub struct Dance<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call options.
    pub model: &'a T,

    /// The parameters for the first call option.
    pub params1: OptionParameters,

    /// The parameters for the second call option.
    pub params2: OptionParameters,

    /// The parameters for the third call option.
    pub params3: OptionParameters,
}

impl<'a, T: OptionPricingModel> Dance<'a, T> {
    /// Creates a new `Dance` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params1` - The parameters for the first call option.
    /// * `params2` - The parameters for the second call option.
    /// * `params3` - The parameters for the third call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Dance`.
    pub fn new(
        model: &'a T,
        params1: OptionParameters,
        params2: OptionParameters,
        params3: OptionParameters,
    ) -> Self {
        Self {
            model,
            params1,
            params2,
            params3,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for Dance<'a, T> {
    /// Calculates the price of the `Dance` option strategy.
    ///
    /// The `Dance` strategy price is calculated as the sum of the prices of three call options
    /// with different strike prices \( k1 \), \( k2 \), and \( k3 \). This combination aims to benefit
    /// from various scenarios of price movements in the underlying asset.
    ///
    /// \[
    /// \text{Price} = C(k1) + C(k2) + C(k3)
    /// \]
    ///
    /// Where:
    /// - \( C(k1) \) is the price of the call option with strike price \( k1 \),
    /// - \( C(k2) \) is the price of the call option with strike price \( k2 \),
    /// - \( C(k3) \) is the price of the call option with strike price \( k3 \).
    ///
    /// # Returns
    ///
    /// Returns the total price of the `Dance` option strategy.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::Dance;
    /// let model = BlackScholesModel;
    /// let params1 = OptionParameters {
    ///     s: 100.0,
    ///     k: 90.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let params2 = OptionParameters {
    ///     s: 100.0,
    ///     k: 100.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let params3 = OptionParameters {
    ///     s: 100.0,
    ///     k: 110.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let dance_strategy = Dance::new(&model, params1, params2, params3);
    /// let price = dance_strategy.price();
    /// println!("Dance Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of each call option with strike prices `k1`, `k2`, and `k3`.
        let call_price1 = self.model.call_price(&self.params1);
        let call_price2 = self.model.call_price(&self.params2);
        let call_price3 = self.model.call_price(&self.params3);

        // The total price of the Dance strategy is the sum of the call option prices.
        call_price1 + call_price2 + call_price3
    }
}
