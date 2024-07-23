use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents an `IronCondor` option strategy.
///
/// The `IronCondor` strategy is a type of option spread strategy that combines a bull put spread with a bear call spread.
/// This strategy is designed to profit from low volatility in the underlying asset. It is composed of four legs:
/// - A short call option with strike price `k2` (lower call strike).
/// - A long call option with strike price `k3` (higher call strike).
/// - A short put option with strike price `k1` (lower put strike).
/// - A long put option with strike price `k4` (higher put strike).
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the long put option (lower strike).
/// - `k2`: The strike price of the short put option (center strike).
/// - `k3`: The strike price of the short call option (center strike).
/// - `k4`: The strike price of the long call option (higher strike).
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of all options (in years).
pub struct IronCondor<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the long put option (lower strike).
    pub k1: f64,

    /// The strike price of the short put option (center strike).
    pub k2: f64,

    /// The strike price of the short call option (center strike).
    pub k3: f64,

    /// The strike price of the long call option (higher strike).
    pub k4: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of all options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> IronCondor<'a, T> {
    /// Creates a new `IronCondor` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the long put option (lower strike).
    /// * `k2` - The strike price of the short put option (center strike).
    /// * `k3` - The strike price of the short call option (center strike).
    /// * `k4` - The strike price of the long call option (higher strike).
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of all options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `IronCondor`.
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        k3: f64,
        k4: f64,
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
            k4,
            r,
            sigma,
            t,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for IronCondor<'a, T> {
    /// Calculates the price of the `IronCondor` option strategy.
    ///
    /// The `IronCondor` strategy is composed of four legs:
    /// - A short call option with strike price `k3`.
    /// - A long call option with strike price `k4`.
    /// - A short put option with strike price `k2`.
    /// - A long put option with strike price `k1`.
    ///
    /// The price of the strategy is calculated as:
    ///
    /// \[
    /// \text{Price} = (P_{\text{short}} - P_{\text{long}}) + (C_{\text{short}} - C_{\text{long}})
    /// \]
    ///
    /// Where:
    /// - \( P_{\text{short}} \) is the price of the short put option with strike price `k2`,
    /// - \( P_{\text{long}} \) is the price of the long put option with strike price `k1`,
    /// - \( C_{\text{short}} \) is the price of the short call option with strike price `k3`,
    /// - \( C_{\text{long}} \) is the price of the long call option with strike price `k4`.
    ///
    /// # Returns
    ///
    /// Returns the price of the `IronCondor` option strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::IronCondor;
    /// let model = BlackScholesModel;
    /// let iron_condor = IronCondor::new(&model, 100.0, 90.0, 95.0, 105.0, 110.0, 0.05, 0.2, 0.5);
    /// let price = iron_condor.price();
    /// println!("Iron Condor Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the short put option with strike price `k2`.
        let put_price1 = self
            .model
            .put_price(self.s, self.k2, self.r, self.sigma, self.t);

        // Calculate the price of the long put option with strike price `k1`.
        let put_price2 = self
            .model
            .put_price(self.s, self.k1, self.r, self.sigma, self.t);

        // Calculate the price of the short call option with strike price `k3`.
        let call_price1 = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);

        // Calculate the price of the long call option with strike price `k4`.
        let call_price2 = self
            .model
            .call_price(self.s, self.k4, self.r, self.sigma, self.t);

        // The total price of the Iron Condor strategy is the sum of the price differences of the puts and calls.
        put_price1 - put_price2 + call_price1 - call_price2
    }
}
