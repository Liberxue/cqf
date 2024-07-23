use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a condor option strategy.
///
/// A condor spread strategy involves a combination of call options with different strike prices.
/// It is a market-neutral strategy that aims to profit from low volatility in the underlying asset.
/// The condor spread consists of four legs: buying a lower strike call, selling two middle strike calls,
/// and buying a higher strike call.
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the lowest strike call (first leg).
/// - `k2`: The strike price of the second lowest strike call (second leg).
/// - `k3`: The strike price of the second highest strike call (third leg).
/// - `k4`: The strike price of the highest strike call (fourth leg).
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of all the options (in years).
pub struct Condor<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the lowest strike call.
    pub k1: f64,

    /// The strike price of the second lowest strike call.
    pub k2: f64,

    /// The strike price of the second highest strike call.
    pub k3: f64,

    /// The strike price of the highest strike call.
    pub k4: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of all the options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> Condor<'a, T> {
    /// Creates a new `Condor` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the lowest strike call.
    /// * `k2` - The strike price of the second lowest strike call.
    /// * `k3` - The strike price of the second highest strike call.
    /// * `k4` - The strike price of the highest strike call.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of all the options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Condor`.
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

impl<'a, T: OptionPricingModel> OptionStrategy for Condor<'a, T> {
    /// Calculates the price of the condor option strategy.
    ///
    /// The condor strategy price is calculated as:
    ///
    /// \[
    /// \text{Price} = C1 - C2 + C3 - C4
    /// \]
    ///
    /// Where:
    /// - \( C1 \) is the price of the call option with strike price \( k1 \),
    /// - \( C2 \) is the price of the call option with strike price \( k2 \),
    /// - \( C3 \) is the price of the call option with strike price \( k3 \),
    /// - \( C4 \) is the price of the call option with strike price \( k4 \).
    ///
    /// This formula reflects the cost of implementing the condor spread strategy, which
    /// involves buying a lower strike call, selling two middle strike calls, and buying
    /// a higher strike call.
    ///
    /// # Returns
    ///
    /// Returns the price of the condor strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::Condor;
    /// let model = BlackScholesModel;
    /// let condor = Condor::new(&model, 100.0, 90.0, 95.0, 105.0, 110.0, 0.05, 0.2, 0.5);
    /// let price = condor.price();
    /// println!("Condor Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the call option with the lowest strike price.
        let c1 = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t);

        // Calculate the price of the call option with the second lowest strike price.
        let c2 = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);

        // Calculate the price of the call option with the second highest strike price.
        let c3 = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);

        // Calculate the price of the call option with the highest strike price.
        let c4 = self
            .model
            .call_price(self.s, self.k4, self.r, self.sigma, self.t);

        // The condor strategy price is the cost of the long call positions minus the proceeds from the short calls.
        c1 - c2 + c3 - c4
    }
}
