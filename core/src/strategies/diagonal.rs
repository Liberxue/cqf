use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a `DiagonalSpread` option strategy.
///
/// The `DiagonalSpread` strategy involves buying and selling call options with different strike prices and different expiration dates.
/// This strategy aims to benefit from the changes in the underlying asset's price and time decay, and is commonly used to capture price movements while managing risk.
///
/// # Fields
/// - `model`: The option pricing model used to price the call options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the short (near-term) call option.
/// - `k2`: The strike price of the long (far-term) call option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t1`: The time to maturity of the short (near-term) call option (in years).
/// - `t2`: The time to maturity of the long (far-term) call option (in years).
pub struct DiagonalSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the short (near-term) call option.
    pub k1: f64,

    /// The strike price of the long (far-term) call option.
    pub k2: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the short (near-term) call option (in years).
    pub t1: f64,

    /// The time to maturity of the long (far-term) call option (in years).
    pub t2: f64,
}

impl<'a, T: OptionPricingModel> DiagonalSpread<'a, T> {
    /// Creates a new `DiagonalSpread` option strategy instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the short (near-term) call option.
    /// * `k2` - The strike price of the long (far-term) call option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t1` - The time to maturity of the short (near-term) call option.
    /// * `t2` - The time to maturity of the long (far-term) call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `DiagonalSpread`.
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        r: f64,
        sigma: f64,
        t1: f64,
        t2: f64,
    ) -> Self {
        Self {
            model,
            s,
            k1,
            k2,
            r,
            sigma,
            t1,
            t2,
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
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::DiagonalSpread;
    /// let model = BlackScholesModel;
    /// let diagonal_spread = DiagonalSpread::new(&model, 100.0, 90.0, 110.0, 0.05, 0.2, 0.1, 0.5);
    /// let price = diagonal_spread.price();
    /// println!("Diagonal Spread Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the near-term (short) call option with strike price `k1` and time to maturity `t1`.
        let near_leg = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t1);

        // Calculate the price of the far-term (long) call option with strike price `k2` and time to maturity `t2`.
        let far_leg = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t2);

        // The total price of the Diagonal Spread strategy is the difference between the long and short call option prices.
        far_leg - near_leg
    }
}
