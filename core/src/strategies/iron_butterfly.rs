use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents an `IronButterfly` option strategy.
///
/// The `IronButterfly` strategy is a variation of the butterfly spread with the addition of a short position in a put option
/// and a long position in a call option. This strategy aims to benefit from low volatility in the underlying asset.
/// It is composed of four legs:
/// - A short (near-term) call option with strike price `k2` (center strike).
/// - A short (near-term) put option with strike price `k2` (center strike).
/// - A long call option with strike price `k3` (higher strike).
/// - A long put option with strike price `k1` (lower strike).
pub struct IronButterfly<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the long (lower strike) put option.
    pub params1: OptionParameters,

    /// The parameters for the short (center strike) call and put options.
    pub params2: OptionParameters,

    /// The parameters for the long (higher strike) call option.
    pub params3: OptionParameters,
}

impl<'a, T: OptionPricingModel> IronButterfly<'a, T> {
    /// Creates a new `IronButterfly` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params1` - The parameters for the long (lower strike) put option.
    /// * `params2` - The parameters for the short (center strike) call and put options.
    /// * `params3` - The parameters for the long (higher strike) call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `IronButterfly`.
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

impl<'a, T: OptionPricingModel> OptionStrategy for IronButterfly<'a, T> {
    /// Calculates the price of the `IronButterfly` option strategy.
    ///
    /// The `IronButterfly` strategy is composed of four legs:
    /// - A short (center strike) call option with strike price `params2.k`.
    /// - A short (center strike) put option with strike price `params2.k`.
    /// - A long call option with strike price `params3.k`.
    /// - A long put option with strike price `params1.k`.
    ///
    /// The price of the strategy is calculated as:
    ///
    /// \[
    /// \text{Price} = C_{\text{center}} + P_{\text{center}} - C_{\text{long}} - P_{\text{long}}
    /// \]
    ///
    /// Where:
    /// - \( C_{\text{center}} \) is the price of the short (center strike) call option,
    /// - \( P_{\text{center}} \) is the price of the short (center strike) put option,
    /// - \( C_{\text{long}} \) is the price of the long (higher strike) call option,
    /// - \( P_{\text{long}} \) is the price of the long (lower strike) put option.
    ///
    /// # Returns
    ///
    /// Returns the price of the `IronButterfly` option strategy.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::IronButterfly;
    /// let model = BlackScholesModel;
    /// let params1 = OptionParameters {
    ///     s: 100.0,
    ///     k: 95.0,
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
    ///     k: 105.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let iron_butterfly = IronButterfly::new(&model, params1, params2, params3);
    /// let price = iron_butterfly.price();
    /// println!("Iron Butterfly Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the short (center strike) call option with strike price `params2.k`.
        let call_price = self.model.call_price(&self.params2);

        // Calculate the price of the short (center strike) put option with strike price `params2.k`.
        let put_price = self.model.put_price(&self.params2);

        // Calculate the price of the long (higher strike) call option with strike price `params3.k`.
        let long_call_price = self.model.call_price(&self.params3);

        // Calculate the price of the long (lower strike) put option with strike price `params1.k`.
        let long_put_price = self.model.put_price(&self.params1);

        // The total price of the Iron Butterfly strategy is the sum of the prices of the short options minus the prices of the long options.
        call_price + put_price - long_call_price - long_put_price
    }
}
