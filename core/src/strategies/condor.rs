use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a condor option strategy.
///
/// A condor spread strategy involves a combination of call options with different strike prices.
/// It is a market-neutral strategy that aims to profit from low volatility in the underlying asset.
/// The condor spread consists of four legs: buying a lower strike call, selling two middle strike calls,
/// and buying a higher strike call.
pub struct Condor<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the lowest strike call option.
    pub params1: OptionParameters,

    /// The strike price of the second lowest strike call.
    pub k2: f64,

    /// The strike price of the second highest strike call.
    pub k3: f64,

    /// The strike price of the highest strike call.
    pub k4: f64,
}

impl<'a, T: OptionPricingModel> Condor<'a, T> {
    /// Creates a new `Condor` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params1` - The parameters for the lowest strike call.
    /// * `k2` - The strike price of the second lowest strike call.
    /// * `k3` - The strike price of the second highest strike call.
    /// * `k4` - The strike price of the highest strike call.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `Condor`.
    pub fn new(model: &'a T, params1: OptionParameters, k2: f64, k3: f64, k4: f64) -> Self {
        Self {
            model,
            params1,
            k2,
            k3,
            k4,
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
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::Condor;
    /// let model = BlackScholesModel;
    /// let params1 = OptionParameters {
    ///     s: 100.0,
    ///     k: 90.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let condor = Condor::new(&model, params1, 95.0, 105.0, 110.0);
    /// let price = condor.price();
    /// println!("Condor Strategy Price: {}", price);
    fn price(&self) -> f64 {
        // Calculate the price of the call option with the lowest strike price.
        let c1 = self.model.call_price(&self.params1);

        // Create parameters for the other call options.
        let params2 = OptionParameters {
            k: self.k2,
            ..self.params1
        };
        let params3 = OptionParameters {
            k: self.k3,
            ..self.params1
        };
        let params4 = OptionParameters {
            k: self.k4,
            ..self.params1
        };

        // Calculate the price of the call options with the other strike prices.
        let c2 = self.model.call_price(&params2);
        let c3 = self.model.call_price(&params3);
        let c4 = self.model.call_price(&params4);

        // The condor strategy price is the cost of the long call positions minus the proceeds from the short calls.
        c1 - c2 + c3 - c4
    }
}
