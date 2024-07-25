pub mod black_scholes;
pub mod monte_carlo;
pub use black_scholes::BlackScholesModel;
pub use monte_carlo::MonteCarloModel;
/// Parameters for option pricing models.  ref: https://www.macroption.com/option-greeks-excel/
///
/// # Fields
///
/// * `s` - The current stock price.
/// * `k` - The strike price of the option.
/// * `r` - The risk-free interest rate (annualized).
/// * `sigma` - The volatility of the stock (annualized).
/// * `t` - The time to maturity in years.
#[derive(Clone)]
pub struct OptionParameters {
    pub s: f64,
    pub k: f64,
    pub r: f64,
    pub sigma: f64,
    pub t: f64,
}

/// A trait for option pricing models.
///
/// This trait defines the methods required for calculating option prices and the Greeks
/// (Delta, Gamma, Vega, Theta, and Rho).
pub trait OptionPricingModel {
    /// Calculates the price of a European call option.
    fn call_price(&self, params: &OptionParameters) -> f64;

    /// Calculates the price of a European put option.
    fn put_price(&self, params: &OptionParameters) -> f64;

    /// Calculates the Delta of the option.
    fn delta(&self, params: &OptionParameters) -> f64;

    /// Calculates the Gamma of the option.
    fn gamma(&self, params: &OptionParameters) -> f64;

    /// Calculates the Vega of the option.
    fn vega(&self, params: &OptionParameters) -> f64;

    /// Calculates the Theta of the option.
    fn theta(&self, params: &OptionParameters) -> f64;

    /// Calculates the Rho of the option.
    fn rho(&self, params: &OptionParameters) -> f64;
}
