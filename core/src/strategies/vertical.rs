use crate::models::OptionPricingModel;
use crate::strategies::OptionStrategy;

/// Represents a vertical spread option strategy.
///
/// A vertical spread involves buying and selling options with the same expiration date but different strike prices.
/// It can be a bull spread (using calls) or a bear spread (using puts).
///
/// # Fields
/// - `model`: The option pricing model used to price the options.
/// - `s`: The current price of the underlying asset.
/// - `k1`: The strike price of the long option.
/// - `k2`: The strike price of the short option.
/// - `r`: The risk-free interest rate (annualized).
/// - `sigma`: The volatility of the underlying asset (annualized).
/// - `t`: The time to maturity of the options (in years).
/// - `is_bull`: A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
pub struct VerticalSpread<'a, T: OptionPricingModel> {
    /// The option pricing model used to price the options.
    pub model: &'a T,

    /// The current price of the underlying asset.
    pub s: f64,

    /// The strike price of the long option.
    pub k1: f64,

    /// The strike price of the short option.
    pub k2: f64,

    /// The risk-free interest rate (annualized).
    pub r: f64,

    /// The volatility of the underlying asset (annualized).
    pub sigma: f64,

    /// The time to maturity of the options (in years).
    pub t: f64,

    /// A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
    pub is_bull: bool,
}

impl<'a, T: OptionPricingModel> VerticalSpread<'a, T> {
    /// Creates a new `VerticalSpread` instance.
    ///
    /// # Arguments
    ///
    /// * `model` - The option pricing model to be used.
    /// * `s` - The current price of the underlying asset.
    /// * `k1` - The strike price of the long option.
    /// * `k2` - The strike price of the short option.
    /// * `r` - The risk-free interest rate.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `t` - The time to maturity of the options.
    /// * `is_bull` - A boolean indicating whether it's a bull spread (true for calls) or a bear spread (false for puts).
    ///
    /// # Returns
    ///
    /// Returns a new instance of `VerticalSpread`.
    pub fn new(
        model: &'a T,
        s: f64,
        k1: f64,
        k2: f64,
        r: f64,
        sigma: f64,
        t: f64,
        is_bull: bool,
    ) -> Self {
        Self {
            model,
            s,
            k1,
            k2,
            r,
            sigma,
            t,
            is_bull,
        }
    }
}

impl<'a, T: OptionPricingModel> OptionStrategy for VerticalSpread<'a, T> {
    /// Calculates the price of the vertical spread strategy.
    ///
    /// For a bull spread (using calls), it calculates the difference between the call prices with strike prices `k1` and `k2`.
    /// For a bear spread (using puts), it calculates the difference between the put prices with strike prices `k1` and `k2`.
    ///
    /// # Returns
    ///
    /// Returns the net cost of the vertical spread.
    ///
    /// # Example
    ///
    /// use core::models::BlackScholesModel;
    /// use core::strategies::VerticalSpread;
    /// let model = BlackScholesModel;
    /// let vertical_spread = VerticalSpread::new(&model, 100.0, 105.0, 110.0, 0.05, 0.2, 1.0, true);
    /// let spread_price = vertical_spread.price();
    /// println!("Vertical Spread Price: {}", spread_price);
    fn price(&self) -> f64 {
        if self.is_bull {
            // Bull spread using calls: long call with strike k1 and short call with strike k2
            let call_price_long = self
                .model
                .call_price(self.s, self.k1, self.r, self.sigma, self.t);
            let call_price_short = self
                .model
                .call_price(self.s, self.k2, self.r, self.sigma, self.t);
            call_price_long - call_price_short
        } else {
            // Bear spread using puts: long put with strike k1 and short put with strike k2
            let put_price_long = self
                .model
                .put_price(self.s, self.k1, self.r, self.sigma, self.t);
            let put_price_short = self
                .model
                .put_price(self.s, self.k2, self.r, self.sigma, self.t);
            put_price_long - put_price_short
        }
    }
}
