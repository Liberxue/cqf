extern crate rand;
use crate::models::OptionPricingModel;
use rand::Rng;

/// A Monte Carlo simulation model for option pricing.
pub struct MonteCarloModel {
    /// The number of simulations to run for estimating option prices.
    pub simulations: usize,
}

impl OptionPricingModel for MonteCarloModel {
    /// Calculates the price of a European call option using Monte Carlo simulation.
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
    /// Returns the estimated price of the European call option.
    ///
    /// # Example
    ///
    /// ```
    /// let model = MonteCarloModel { simulations: 10000 };
    /// let call_price = model.call_price(100.0, 100.0, 0.05, 0.2, 1.0);
    /// println!("Call Price: {}", call_price);
    /// ```
    fn call_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            // Generate a random sample from the standard normal distribution.
            let z: f64 = rng.sample(rand::distributions::StandardNormal);
            // Calculate the simulated stock price at maturity.
            let st = s * ((r - 0.5 * sigma.powi(2)) * t + sigma * t.sqrt() * z).exp();
            // Accumulate the payoff for the call option.
            payoff_sum += (st - k).max(0.0);
        }

        // Discount the average payoff to present value.
        (payoff_sum / self.simulations as f64) * (-r * t).exp()
    }

    /// Calculates the price of a European put option using Monte Carlo simulation.
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
    /// Returns the estimated price of the European put option.
    ///
    /// # Example
    ///
    /// ```
    /// let model = MonteCarloModel { simulations: 10000 };
    /// let put_price = model.put_price(100.0, 100.0, 0.05, 0.2, 1.0);
    /// println!("Put Price: {}", put_price);
    /// ```
    fn put_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            // Generate a random sample from the standard normal distribution.
            let z: f64 = rng.sample(rand::distributions::StandardNormal);
            // Calculate the simulated stock price at maturity.
            let st = s * ((r - 0.5 * sigma.powi(2)) * t + sigma * t.sqrt() * z).exp();
            // Accumulate the payoff for the put option.
            payoff_sum += (k - st).max(0.0);
        }

        // Discount the average payoff to present value.
        (payoff_sum / self.simulations as f64) * (-r * t).exp()
    }
}

