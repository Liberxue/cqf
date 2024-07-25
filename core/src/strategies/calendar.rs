use crate::models::{OptionParameters, OptionPricingModel};
use crate::strategies::OptionStrategy;

/// Represents a calendar spread option strategy.
///
/// A calendar spread involves buying and selling call options with the same strike price
/// but different expiration dates. The strategy profits from differences in time value
/// between the two options.
pub struct CalendarSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The parameters for the near-term call option.
    pub near_params: OptionParameters,

    /// The parameters for the far-term call option.
    pub far_params: OptionParameters,
}

impl<'a, T: OptionPricingModel> CalendarSpread<'a, T> {
    /// Creates a new `CalendarSpread` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `near_params` - The parameters for the near-term call option.
    /// * `far_params` - The parameters for the far-term call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `CalendarSpread`.
    pub fn new(model: &'a T, near_params: OptionParameters, far_params: OptionParameters) -> Self {
        Self {
            model,
            near_params,
            far_params,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for CalendarSpread<'a, T> {
    /// Calculates the price of the calendar spread option strategy.
    ///
    /// The calendar spread price is determined by the formula:
    ///
    /// \[
    /// \text{Price} = C_{\text{far}} - C_{\text{near}}
    /// \]
    ///
    /// Where:
    /// - \( C_{\text{far}} \) is the price of the far-term call option with expiration \( t2 \),
    /// - \( C_{\text{near}} \) is the price of the near-term call option with expiration \( t1 \).
    ///
    /// # Returns
    ///
    /// Returns the price of the calendar spread strategy.
    ///
    /// # Example
    ///
    /// use crate::models::BlackScholesModel;
    /// use crate::strategies::CalendarSpread;
    /// let model = BlackScholesModel;
    /// let near_params = OptionParameters {
    ///     s: 100.0,
    ///     k: 100.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.1,
    /// };
    /// let far_params = OptionParameters {
    ///     s: 100.0,
    ///     k: 100.0,
    ///     r: 0.05,
    ///     sigma: 0.2,
    ///     t: 0.5,
    /// };
    /// let spread = CalendarSpread::new(&model, near_params, far_params);
    /// let price = spread.price();
    /// println!("Calendar Spread Price: {}", price);
    fn price(&self) -> f64 {
        let near_leg = self.model.call_price(&self.near_params);
        let far_leg = self.model.call_price(&self.far_params);

        // Calculate the price of the calendar spread
        far_leg - near_leg
    }
}
