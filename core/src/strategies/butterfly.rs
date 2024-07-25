use crate::models::{OptionParameters, OptionPricingModel};
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

    /// Parameters for the options in the butterfly spread strategy.
    pub params: OptionParameters,

    /// The strike price of the second call option (middle strike).
    pub k2: f64,

    /// The strike price of the third call option (high strike).
    pub k3: f64,
}

impl<'a, T: OptionPricingModel> ButterflySpread<'a, T> {
    /// Creates a new `ButterflySpread` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params` - The parameters for the options.
    /// * `k2` - The strike price of the second call option.
    /// * `k3` - The strike price of the third call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `ButterflySpread`.
    pub fn new(model: &'a T, params: OptionParameters, k2: f64, k3: f64) -> Self {
        Self {
            model,
            params,
            k2,
            k3,
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
    /// let params = OptionParameters {
    ///     s: 100.0,
    ///     k: 90.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let spread = ButterflySpread::new(&model, params, 100.0, 110.0);
    /// let price = spread.price();
    /// println!("Butterfly Spread Price: {}", price);
    fn price(&self) -> f64 {
        let params1 = self.params.clone();
        let mut params2 = self.params.clone();
        let mut params3 = self.params.clone();

        params2.k = self.k2;
        params3.k = self.k3;

        let c1 = self.model.call_price(&params1);
        let c2 = self.model.call_price(&params2);
        let c3 = self.model.call_price(&params3);

        // Calculate the price of the butterfly spread
        c1 - 2.0 * c2 + c3
    }
}
