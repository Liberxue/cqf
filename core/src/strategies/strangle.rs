use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a strangle option strategy.
///
/// A strangle involves buying a call and a put option with different strike prices but the same expiration date.
/// This structure calculates the combined price of the call and put options using the provided option pricing model.
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the call option.
/// - `k2`: The strike price of the put option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the options (in years).
pub struct Strangle<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the call option.
    pub k1: f64,

    /// The strike price of the put option.
    pub k2: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Strangle<'a, T> {
    /// Creates a new `Strangle` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the call option.
    /// * `k2` - The strike price of the put option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Strangle`.
    pub fn new(model: &'a T, s: f64, k1: f64, k2: f64, r: f64, sigma: f64, t: f64) -> Self {
        Self {
            model,
            s,
            k1,
            k2,
            r,
            sigma,
            t,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for Strangle<'a, T> {
    /// Calculates the price of the strangle option strategy.
    ///
    /// The price of the strangle is the sum of the prices of the call and put options with different strike prices but the same expiration date.
    ///
    /// # Returns
    ///
    /// Returns the combined price of the call and put options.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::Strangle;
    /// let model = BlackScholesModel;
    /// let strangle = Strangle::new(&model, 100.0, 110.0, 90.0, 0.05, 0.2, 1.0);
    /// let strangle_price = strangle.price();
    /// println!("Strangle Price: {}", strangle_price);
    fn price(&self) -> f64 {
        let call_price = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t);
        let put_price = self
            .model
            .put_price(self.s, self.k2, self.r, self.sigma, self.t);
        call_price + put_price
    }
}
