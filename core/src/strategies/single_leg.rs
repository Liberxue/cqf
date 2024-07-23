use crate::models::OptionPricingModel;
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

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the option.
    pub k: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the option (in years).
    pub t: f64,

    /// A boolean flag indicating whether the option is a call (`true`) or a put (`false`).
    pub is_call: bool,
}

impl<'a, T: OptionPricingModel> SingleLegOption<'a, T> {
    /// Creates a new `SingleLegOption` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k` - The strike price of the option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the option.
    /// * `is_call` - A boolean flag indicating whether the option is a call (`true`) or a put (`false`).
    ///
    /// # Returns
    ///
    /// Returns a new in stance of `SingleLegOption`.
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t: f64, is_call: bool) -> Self {
        Self {
            model,
            s,
            k,
            r,
            sigma,
            t,
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
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::SingleLegOption;
    /// let model = BlackScholesModel;
    /// let call_option = SingleLegOption::new(&model, 100.0,  105.0, 0.05, 0.2, 1.0, true);
    /// let put_option = SingleLegOption::new(&model, 100.0, 95.0, 0.05, 0.2, 1.0, false);
    /// let call_price = call_option.price();
    /// let put_price = put_option.price();
    /// println!("Call Option Price: {}", call_price);
    /// println!("Put Option Price: {}", put_price);
    fn price(&self) -> f64 {
        if self.is_call {
            self.model
                .call_price(self.s, self.k, self.r, self.sigma, self.t)
        } else {
            self.model
                .put_price(self.s, self.k, self.r, self.sigma, self.t)
        }
    }
}
