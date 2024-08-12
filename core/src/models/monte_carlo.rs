extern crate rand;
use crate::models::{OptionParameters, OptionPricingModel};
use rand::Rng;
use rand_distr::StandardNormal;

/// A Monte Carlo simulation model for pricing European call and put options.
pub struct MonteCarloModel {
    /// The number of simulations to run for the Monte Carlo method.
    pub simulations: usize,

    /// The epsilon value used for finite difference calculations in Greeks.
    pub epsilon: f64,
}

impl OptionPricingModel for MonteCarloModel {
    /// Calculates the price of a European call option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated price of the European call option.
    fn call_price(&self, params: &OptionParameters) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z: f64 = rng.sample(StandardNormal);
            let st = params.s
                * ((params.r - 0.5 * params.sigma.powi(2)) * params.t
                    + params.sigma * params.t.sqrt() * z)
                    .exp();
            payoff_sum += (st - params.k).max(0.0);
        }

        (payoff_sum / self.simulations as f64) * (-params.r * params.t).exp()
    }

    /// Calculates the price of a European put option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated price of the European put option.
    fn put_price(&self, params: &OptionParameters) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z: f64 = rng.sample(StandardNormal);
            let st = params.s
                * ((params.r - 0.5 * params.sigma.powi(2)) * params.t
                    + params.sigma * params.t.sqrt() * z)
                    .exp();
            payoff_sum += (params.k - st).max(0.0);
        }

        (payoff_sum / self.simulations as f64) * (-params.r * params.t).exp()
    }

    /// Calculates the Delta of the option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated Delta of the option.
    fn delta(&self, params: &OptionParameters) -> f64 {
        let mut new_params = params.clone();
        new_params.s = params.s + self.epsilon;
        let price_up = self.call_price(&new_params);
        new_params.s = params.s - self.epsilon;
        let price_down = self.call_price(&new_params);
        (price_up - price_down) / (2.0 * self.epsilon)
    }

    /// Calculates the Gamma of the option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated Gamma of the option.
    fn gamma(&self, params: &OptionParameters) -> f64 {
        let mut new_params = params.clone();
        new_params.s = params.s + self.epsilon;
        let delta_up = self.delta(&new_params);
        new_params.s = params.s - self.epsilon;
        let delta_down = self.delta(&new_params);
        (delta_up - delta_down) / (2.0 * self.epsilon)
    }

    /// Calculates the Vega of the option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated Vega of the option.
    fn vega(&self, params: &OptionParameters) -> f64 {
        let mut new_params = params.clone();
        new_params.sigma = params.sigma + self.epsilon;
        let price_up = self.call_price(&new_params);
        new_params.sigma = params.sigma - self.epsilon;
        let price_down = self.call_price(&new_params);
        (price_up - price_down) / (2.0 * self.epsilon)
    }

    /// Calculates the Theta of the option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated Theta of the option.
    fn theta(&self, params: &OptionParameters) -> f64 {
        let day_epsilon = 1.0 / 365.0; // One day
        let mut new_params = params.clone();
        let price_now = self.call_price(params);
        new_params.t = params.t - day_epsilon;
        let price_future = self.call_price(&new_params);
        (price_future - price_now) / day_epsilon
    }

    /// Calculates the Rho of the option using Monte Carlo simulation.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for the option.
    ///
    /// # Returns
    ///
    /// Returns the estimated Rho of the option.
    fn rho(&self, params: &OptionParameters) -> f64 {
        let mut new_params = params.clone();
        new_params.r = params.r + self.epsilon;
        let price_up = self.call_price(&new_params);
        new_params.r = params.r - self.epsilon;
        let price_down = self.call_price(&new_params);
        (price_up - price_down) / (2.0 * self.epsilon)
    }
}

