use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a collar option strategy.
///
/// A collar strategy involves holding a long position in the underlying asset
/// and simultaneously buying a protective put option and selling a covered call option.
/// This strategy limits both potential gains and losses.
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the put option (lower strike price).
/// - `k2`: The strike price of the call option (higher strike price).
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of both the put and call options (in years).
pub struct Collar<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the put option.
    pub k1: f64,

    /// The strike price of the call option.
    pub k2: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of both the put and call options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Collar<'a, T> {
    /// Creates a new `Collar` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the put option.
    /// * `k2` - The strike price of the call option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of both the put and call options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Collar`.
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

impl<'a, T: OptionPricingModel> OptionStrategy for Collar<'a, T> {
    /// Calculates the price of the collar option strategy.
    ///
    /// The collar strategy price is calculated as:
    ///
    /// \[
    /// \text{Price} = P - C
    /// \]
    ///
    /// Where:
    /// - \( P \) is the price of the put option with strike price \( k1 \),
    /// - \( C \) is the price of the call option with strike price \( k2 \).
    ///
    /// This formula reflects the cost of implementing the collar strategy, which
    /// involves buying a protective put and selling a covered call.
    ///
    /// # Returns
    ///
    /// Returns the price of the collar strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::Collar;
    /// let model = BlackScholesModel;
    /// let collar = Collar::new(&model, 100.0, 95.0, 105.0, 0.05, 0.2, 0.5);
    /// let price = collar.price();
    /// println!("Collar Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the protective put option.
        let put_price = self
            .model
            .put_price(self.s, self.k1, self.r, self.sigma, self.t);

        // Calculate the price of the covered call option.
        let call_price = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);

        // The collar strategy price is the cost of the put minus the proceeds from the call.
        put_price - call_price
    }
}
