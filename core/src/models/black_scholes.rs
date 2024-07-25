use crate::models::{OptionParameters, OptionPricingModel};

/// A Black-Scholes model for pricing European call and put options.
/// ref: https://en.wikipedia.org/wiki/Black–Scholes_model
pub struct BlackScholesModel;

impl OptionPricingModel for BlackScholesModel {
    /// Calculates the price of a European call option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the price of the European call option.
    fn call_price(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let d2 = d1 - params.sigma * params.t.sqrt();
        params.s * standard_normal_cdf(d1)
            - params.k * (-params.r * params.t).exp() * standard_normal_cdf(d2)
    }

    /// Calculates the price of a European put option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the price of the European put option.
    fn put_price(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let d2 = d1 - params.sigma * params.t.sqrt();
        params.k * (-params.r * params.t).exp() * standard_normal_cdf(-d2)
            - params.s * standard_normal_cdf(-d1)
    }

    /// Calculates the Delta of the option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the Delta of the option.
    fn delta(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        standard_normal_cdf(d1)
    }

    /// Calculates the Gamma of the option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the Gamma of the option.
    fn gamma(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let normal_pdf = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * d1.powi(2)).exp();
        normal_pdf / (params.s * params.sigma * params.t.sqrt())
    }

    /// Calculates the Vega of the option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the Vega of the option.
    fn vega(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let normal_pdf = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * d1.powi(2)).exp();
        params.s * normal_pdf * params.t.sqrt()
    }

    /// Calculates the Theta of the option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the Theta of the option.
    fn theta(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let d2 = d1 - params.sigma * params.t.sqrt();
        let normal_pdf = (1.0 / (2.0 * std::f64::consts::PI).sqrt()) * (-0.5 * d1.powi(2)).exp();
        let theta_call = -((params.s * normal_pdf * params.sigma) / (2.0 * params.t.sqrt()))
            - params.r * params.k * (-params.r * params.t).exp() * standard_normal_cdf(d2);
        theta_call / 365.0 // Annualize to daily
    }

    /// Calculates the Rho of the option using the Black-Scholes formula.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the Rho of the option.
    fn rho(&self, params: &OptionParameters) -> f64 {
        let d1 = (1.0 / (params.sigma * params.t.sqrt()))
            * ((params.s / params.k).ln() + (params.r + 0.5 * params.sigma.powi(2)) * params.t);
        let d2 = d1 - params.sigma * params.t.sqrt();
        params.k * params.t * (-params.r * params.t).exp() * standard_normal_cdf(d2) / 100.0
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
