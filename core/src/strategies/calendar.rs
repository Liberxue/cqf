use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a calendar spread option strategy.
///
/// A calendar spread involves buying and selling call options with the same strike price
/// but different expiration dates. The strategy profits from differences in time value
/// between the two options.
pub struct CalendarSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the call options.
    pub k: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the near-term call option (in years).
    pub t1: f64,

    /// The time to maturity of the far-term call option (in years).
    pub t2: f64,
}

impl<'a, T: OptionPricingModel> CalendarSpread<'a, T> {
    /// Creates a new `CalendarSpread` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k` - The strike price of the call options.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t1` - The time to maturity of the near-term call option.
    /// * `t2` - The time to maturity of the far-term call option.
    ///
    /// # Returns
    ///
    /// Returns a new instance of `CalendarSpread`.
    pub fn new(model: &'a T, s: f64, k: f64, r: f64, sigma: f64, t1: f64, t2: f64) -> Self {
        Self {
            model,
            s,
            k,
            r,
            sigma,
            t1,
            t2,
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
    /// let spread = CalendarSpread::new(&model, 100.0, 100.0, 0.05, 0.2, 0.1, 0.5);
    /// let price = spread.price();
    /// println!("Calendar Spread Price: {}", price);
    fn price(&self) -> f64 {
        let near_leg = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t1);
        let far_leg = self
            .model
            .call_price(self.s, self.k, self.r, self.sigma, self.t2);

        // Calculate the price of the calendar spread
        far_leg - near_leg
    }
}
