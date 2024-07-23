use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a covered call option strategy.
///
/// A covered call strategy involves holding a long position in the underlying asset
/// while simultaneously selling a call option on the same asset. This strategy is used
/// to generate additional income (premium) from the call option, while potentially
/// capping the maximum profit from the asset's price appreciation.
///
/// # Fields
/// - `model`: The option pricing model used to price the call option.
/// - `s`: The current price of the underlying asset.
/// - `k`: The strike price of the call option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the call option (in years).
pub struct CoveredCall<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call option.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the call option.
    pub k: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the call option (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> CoveredCall<'a, T> {
    /// Creates a new `CoveredCall` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k` - The strike price of the call option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `CoveredCall`.
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

impl<'a, T: OptionPricingModel> OptionStrategy for CoveredCall<'a, T> {
    /// Calculates the price of the covered call option strategy.
    ///
    /// The covered call strategy price is calculated as:
    ///
    /// \[
    /// \text{Price} = S - C
    /// \]
    ///
    /// Where:
    /// - \( S \) is the current price of the underlying asset,
    /// - \( C \) is the price of the call option with strike price \( k \).
    ///
    /// This formula reflects the net profit or loss from implementing the covered call strategy,
    /// which is the current value of the underlying asset minus the premium received for selling
    /// the call option.
    ///
    /// # Returns
    ///
    /// Returns the price of the covered call strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::CoveredCall;
    /// let model = BlackScholesModel;
    /// let covered_call = CoveredCall::new(&model, 100.0, 105.0, 0.05, 0.2, 0.5);
    /// let price = covered_call.price();
    /// println!("Covered Call Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the call option with strike price `k`.
        let call_price = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t);

        // The covered call strategy price is the current value of the underlying asset minus
        // the price of the call option.
        self.s - call_price
    }
}
