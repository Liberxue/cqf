use crate::models::OptionPricingModel;
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
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the long (lower strike) put option.
/// - `k2`: The strike price of the short (center strike) call and put options.
/// - `k3`: The strike price of the long (higher strike) call option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of all options (in years).
pub struct IronButterfly<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the long (lower strike) put option.
    pub k1: f64,

    /// The strike price of the short (center strike) call and put options.
    pub k2: f64,

    /// The strike price of the long (higher strike) call option.
    pub k3: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of all options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> IronButterfly<'a, T> {
    /// Creates a new `IronButterfly` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the long (lower strike) put option.
    /// * `k2` - The strike price of the short (center strike) call and put options.
    /// * `k3` - The strike price of the long (higher strike) call option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of all options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `IronButterfly`.
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        k3: f64,
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
            r,
            sigma,
            t,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for IronButterfly<'a, T> {
    /// Calculates the price of the `IronButterfly` option strategy.
    ///
    /// The `IronButterfly` strategy is composed of four legs:
    /// - A short (center strike) call option with strike price `k2`.
    /// - A short (center strike) put option with strike price `k2`.
    /// - A long call option with strike price `k3`.
    /// - A long put option with strike price `k1`.
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
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::IronButterfly;
    /// let model = BlackScholesModel;
    /// let iron_butterfly = IronButterfly::new(&model, 100.0, 95.0, 100.0, 105.0, 0.05, 0.2, 0.5);
    /// let price = iron_butterfly.price();
    /// println!("Iron Butterfly Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the short (center strike) call option with strike price `k2`.
        let call_price = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);

        // Calculate the price of the short (center strike) put option with strike price `k2`.
        let put_price = self
            .model
            .put_price(self.s, self.k2, self.r, self.sigma, self.t);

        // Calculate the price of the long (higher strike) call option with strike price `k3`.
        let long_call_price = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);

        // Calculate the price of the long (lower strike) put option with strike price `k1`.
        let long_put_price = self
            .model
            .put_price(self.s, self.k1, self.r, self.sigma, self.t);

        // The total price of the Iron Butterfly strategy is the sum of the prices of the short options minus the prices of the long options.
        call_price + put_price - long_call_price - long_put_price
    }
}
