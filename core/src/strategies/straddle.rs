use crate::models::OptionPricingModel;
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

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of both the call and put options.
    pub k: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Straddle<'a, T> {
    /// Creates a new `Straddle` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k` - The strike price of both the call and put options.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Straddle`.
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> Self {
        Self {
            model,
            s,
            k,
            r,
            sigma,
            t,
        }
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
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::Straddle;
    /// let model = BlackScholesModel;
    /// let straddle = Straddle::new(&model, 100.0, 100.0, 0.05, 0.2, 1.0);
    /// let straddle_price = straddle.price();
    /// println!("Straddle Price: {}", straddle_price);
    fn price(&self) -> f64 {
        let call_price = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t);
        let put_price = self
            .model
            .put_price(self.s, self.k, self.r, self.sigma, self.t);
        call_price + put_price
    }
}
