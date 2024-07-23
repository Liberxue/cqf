use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a `Dance` option strategy.
///
/// The `Dance` strategy involves creating a combination of call options with different strike prices.
/// It can be used to take advantage of price movements and volatility in the underlying asset by combining
/// multiple call options into a single strategy. This can provide a more complex exposure to the underlying asset's movements.
///
/// # Fields
/// - `model`: The option pricing model used to price the call options.
/// - `s`: The current price of the underlying asset.
/// - `k1`, `k2`, `k3`: The strike prices of the call options.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the call options (in years).
pub struct Dance<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the first call option.
    pub k1: f64,

    /// The strike price of the second call option.
    pub k2: f64,

    /// The strike price of the third call option.
    pub k3: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the call options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Dance<'a, T> {
    /// Creates a new `Dance` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the first call option.
    /// * `k2` - The strike price of the second call option.
    /// * `k3` - The strike price of the third call option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the call options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Dance`.
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
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::Dance;
    /// let model = BlackScholesModel;
    /// let dance_strategy = Dance::new(&model, 100.0, 90.0, 100.0, 110.0, 0.05, 0.2, 0.5);
    /// let price = dance_strategy.price();
    /// println!("Dance Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of each call option with strike prices `k1`, `k2`, and `k3`.
        let call_price1 = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t);
        let call_price2 = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);
        let call_price3 = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);

        // The total price of the Dance strategy is the sum of the call option prices.
        call_price1 + call_price2 + call_price3
    }
}
