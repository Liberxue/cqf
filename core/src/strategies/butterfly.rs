use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a butterfly spread option strategy.
///
/// A butterfly spread involves buying one call option with a low strike price,
/// selling two call options with a middle strike price, and buying one call option
/// with a high strike price. This strategy profits from minimal price movement in the
/// underlying asset.
pub struct ButterflySpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the first call option (low strike).
    pub k1: f64,

    /// The strike price of the second call option (middle strike).
    pub k2: f64,

    /// The strike price of the third call option (high strike).
    pub k3: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the options (in years).
    pub t: f64,
}

impl<'a, T: OptionPricingModel> ButterflySpread<'a, T> {
    /// Creates a new `ButterflySpread` instance.
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
    /// * `t` - The time to maturity of the options.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `ButterflySpread`.
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

impl<'a, T: OptionPricingModel> OptionStrategy for ButterflySpread<'a, T> {
    /// Calculates the price of the butterfly spread option strategy.
    ///
    /// The butterfly spread price is determined by the formula:
    ///
    /// \[
    /// \text{Price} = C_{k1} - 2 \cdot C_{k2} + C_{k3}
    /// \]
    ///
    /// Where:
    /// - \( C_{k1} \) is the price of a call option with strike price \( k1 \),
    /// - \( C_{k2} \) is the price of a call option with strike price \( k2 \),
    /// - \( C_{k3} \) is the price of a call option with strike price \( k3 \).
    ///
    /// # Returns
    ///
    /// Returns the price of the butterfly spread strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::ButterflySpread;
    /// let model = BlackScholesModel;
    /// let spread = ButterflySpread::new(&model, 100.0, 90.0, 100.0, 110.0, 0.05, 0.2, 1.0);
    /// let price = spread.price();
    /// println!("Butterfly Spread Price: {}", price);
    fn price(&self) -> f64 {
        let c1 = self
            .model
            .call_price(self.s, self.k1, self.r, self.sigma, self.t);
        let c2 = self
            .model
            .call_price(self.s, self.k2, self.r, self.sigma, self.t);
        let c3 = self
            .model
            .call_price(self.s, self.k3, self.r, self.sigma, self.t);

        // Calculate the price of the butterfly spread
        c1 - 2.0 * c2 + c3
    }
}
