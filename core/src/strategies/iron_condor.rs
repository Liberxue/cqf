use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents an `IronCondor` option strategy.
///
/// The `IronCondor` strategy is a type of option spread strategy that combines a bull put spread with a bear call spread.
/// This strategy is designed to profit from low volatility in the underlying asset. It is composed of four legs:
/// - A short call option with strike price `k2` (lower call strike).
/// - A long call option with strike price `k3` (higher call strike).
/// - A short put option with strike price `k1` (lower put strike).
/// - A long put option with strike price `k4` (higher put strike).
pub struct IronCondor<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the long put option (lower strike).
    pub params1: OptionParameters,

    /// The parameters for the short put option (center strike).
    pub params2: OptionParameters,

    /// The parameters for the short call option (center strike).
    pub params3: OptionParameters,

    /// The parameters for the long call option (higher strike).
    pub params4: OptionParameters,
}

impl<'a, T: OptionPricingModel> IronCondor<'a, T> {
    /// Creates a new `IronCondor` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params1` - The parameters for the long put option (lower strike).
    /// * `params2` - The parameters for the short put option (center strike).
    /// * `params3` - The parameters for the short call option (center strike).
    /// * `params4` - The parameters for the long call option (higher strike).
    ///
    /// # Returns
    ///
    /// Returns a new instance of `IronCondor`.
    pub fn new(
        model: &'a T,
        params1: OptionParameters,
        params2: OptionParameters,
        params3: OptionParameters,
        params4: OptionParameters,
    ) -> Self {
        Self {
            model,
            params1,
            params2,
            params3,
            params4,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for IronCondor<'a, T> {
    /// Calculates the price of the `IronCondor` option strategy.
    ///
    /// The `IronCondor` strategy is composed of four legs:
    /// - A short call option with strike price `params3.k`.
    /// - A long call option with strike price `params4.k`.
    /// - A short put option with strike price `params2.k`.
    /// - A long put option with strike price `params1.k`.
    ///
    /// The price of the strategy is calculated as:
    ///
    /// \[
    /// \text{Price} = (P_{\text{short}} - P_{\text{long}}) + (C_{\text{short}} - C_{\text{long}})
    /// \]
    ///
    /// Where:
    /// - \( P_{\text{short}} \) is the price of the short put option with strike price `params2.k`,
    /// - \( P_{\text{long}} \) is the price of the long put option with strike price `params1.k`,
    /// - \( C_{\text{short}} \) is the price of the short call option with strike price `params3.k`,
    /// - \( C_{\text{long}} \) is the price of the long call option with strike price `params4.k`.
    ///
    /// # Returns
    ///
    /// Returns the price of the `IronCondor` option strategy.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::IronCondor;
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
    ///     k: 95.0,
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
    /// let params4 = OptionParameters {
    ///     s: 100.0,
    ///     k: 110.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let iron_condor = IronCondor::new(&model, params1, params2, params3, params4);
    /// let price = iron_condor.price();
    /// println!("Iron Condor Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the short put option with strike price `params2.k`.
        let put_price1 = self.model.put_price(&self.params2);

        // Calculate the price of the long put option with strike price `params1.k`.
        let put_price2 = self.model.put_price(&self.params1);

        // Calculate the price of the short call option with strike price `params3.k`.
        let call_price1 = self.model.call_price(&self.params3);

        // Calculate the price of the long call option with strike price `params4.k`.
        let call_price2 = self.model.call_price(&self.params4);

        // The total price of the Iron Condor strategy is the sum of the price differences of the puts and calls.
        put_price1 - put_price2 + call_price1 - call_price2
    }
}
