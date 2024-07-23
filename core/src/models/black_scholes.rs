use crate::models::OptionPricingModel;

/// A Black-Scholes model for pricing European call and put options.
pub struct BlackScholesModel;

impl OptionPricingModel for BlackScholesModel {
    /// Calculates the price of a European call option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `s` - The current stock price.
    /// * `k` - The strike price of the option.
    /// * `r` - The risk-free interest rate (annualized).
    /// * `sigma` - The volatility of the stock (annualized).
    /// * `t` - The time to maturity in years.
    ///
    /// # Returns
    ///
    /// Returns the price of the European call option.
    ///
    /// # Example
    ///
    /// let model = BlackScholesModel;
    /// let call_price = model.call_price(100.0, 100.0, 0.05, 0.2, 1.0);
    /// println!("Call Price: {}", call_price);
    fn call_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let d1 = (1.0 / (sigma * t.sqrt())) * ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t);
        let d2 = d1 - sigma * t.sqrt();
        s * standard_normal_cdf(d1) - k * (-r * t).exp() * standard_normal_cdf(d2)
    }

    /// Calculates the price of a European put option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `s` - The current stock price.
    /// * `k` - The strike price of the option.
    /// * `r` - The risk-free interest rate (annualized).
    /// * `sigma` - The volatility of the stock (annualized).
    /// * `t` - The time to maturity in years.
    ///
    /// # Returns
    ///
    /// Returns the price of the European put option.
    ///
    /// # Example
    ///
    /// let model = BlackScholesModel;
    /// let put_price = model.put_price(100.0, 100.0, 0.05, 0.2, 1.0);
    /// println!("Put Price: {}", put_price);
    fn put_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let d1 = (1.0 / (sigma * t.sqrt())) * ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t);
        let d2 = d1 - sigma * t.sqrt();
        k * (-r * t).exp() * standard_normal_cdf(-d2) - s * standard_normal_cdf(-d1)
    }
}

/// Calculates the cumulative distribution function (CDF) of the standard normal distribution.
///
/// # Arguments
///
/// * `x` - The value for which to compute the CDF.
///
/// # Returns
///
/// Returns the CDF value for the standard normal distribution.
fn standard_normal_cdf(x: f64) -> f64 {
    (1.0 + erf(x / 2.0_f64.sqrt())) / 2.0
}

/// Computes the error function (erf), which is used in the standard normal CDF calculation.
///
/// # Arguments
///
/// * `x` - The value for which to compute the error function.
///
/// # Returns
///
/// Returns the value of the error function for the given `x`.
///
/// # Notes
///
/// The implementation uses an approximation to the error function (erf) to compute the CDF.
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}
