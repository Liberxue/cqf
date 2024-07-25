use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a single leg of an option (either a call or a put).
///
/// This structure is used to price a single option leg. It can be either a call or a put option,
/// depending on the `is_call` flag. The pricing is done using the specified option pricing model.
///
/// # Fields
/// - `model`: The option pricing model used to price the option.
/// - `s`: The current price of the underlying asset.
/// - `k`: The strike price of the option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the option (in years).
/// - `is_call`: A boolean flag indicating whether the option is a call (`true`) or a put (`false`).
pub struct SingleLegOption<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the option.
    pub model: &'a T,

    /// The parameters for the option.
    pub params: OptionParameters,

    /// A boolean flag indicating whether the option is a call (`true`) or a put (`false`).
    pub is_call: bool,
}

impl<'a, T: OptionPricingModel> SingleLegOption<'a, T> {
    /// Creates a new `SingleLegOption` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `params` - The parameters for the option.
    /// * `is_call` - A boolean flag indicating whether the option is a call (`true`) or a put (`false`).
    ///
    /// # Returns
    ///
    /// Returns a new instance of `SingleLegOption`.
    pub fn new(model: &'a T, params: OptionParameters, is_call: bool) -> Self {
        Self {
            model,
            params,
            is_call,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for SingleLegOption<'a, T> {
    /// Calculates the price of the single option leg.
    ///
    /// Depending on the value of `is_call`, this method will calculate and return either the price
    /// of a call option or a put option using the provided option pricing model.
    ///
    /// # Returns
    ///
    /// Returns the price of the option (call or put) based on the `is_call` flag.
    ///
    /// # Example
    ///
    /// use crate::models::{BlackScholesModel, OptionParameters};
    /// use crate::strategies::SingleLegOption;
    /// let model = BlackScholesModel;
    /// let params_call = OptionParameters {
    ///     s: 100.0,
    ///     k: 105.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let params_put = OptionParameters {
    ///     s: 100.0,
    ///     k: 95.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 1.0,
    /// };
    /// let call_option = SingleLegOption::new(&model, params_call, true);
    /// let put_option = SingleLegOption::new(&model, params_put, false);
    /// let call_price = call_option.price();
    /// let put_price = put_option.price();
    /// println!("Call Option Price: {}", call_price);
    /// println!("Put Option Price: {}", put_price);
    fn price(&self) -> f64 {
        if self.is_call {
            self.model.call_price(&self.params)
        } else {
            self.model.put_price(&self.params)
        }
    }
}
