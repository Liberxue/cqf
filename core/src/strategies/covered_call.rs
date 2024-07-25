use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a covered call option strategy.
///
/// A covered call strategy involves holding a long position in the underlying asset
/// while simultaneously selling a call option on the same asset. This strategy is used
/// to generate additional income (premium) from the call option, while potentially
/// capping the maximum profit from the asset's price appreciation.
pub struct CoveredCall<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the call option.
    pub model: &'a T,

    /// Parameters for the call option.
    pub params: OptionParameters,
}

impl<'a, T: OptionPricingModel> CoveredCall<'a, T> {
    /// Creates a new `CoveredCall` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params` - The parameters for the call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `CoveredCall`.
    pub fn new(model: &'a T, params: OptionParameters) -> Self {
        Self { model, params }
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
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::CoveredCall;
    /// let model = BlackScholesModel;
    /// let params = OptionParameters {
    ///     s: 100.0,
    ///     k: 105.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let covered_call = CoveredCall::new(&model, params);
    /// let price = covered_call.price();
    /// println!("Covered Call Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the call option with strike price `k`.
        let call_price = self.model.call_price(&self.params);

        // The covered call strategy price is the current value of the underlying asset minus
        // the price of the call option.
        self.params.s - call_price
    }
}
