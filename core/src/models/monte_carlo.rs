extern crate rand;
use crate::models::OptionPricingModel;
use rand::Rng;
use rand_distr::StandardNormal;

/// A Monte Carlo simulation model for pricing European call and put options.
pub struct MonteCarloModel {
    /// The number of simulations to run for the Monte Carlo method.
    pub simulations: usize,
}

impl OptionPricingModel for MonteCarloModel {
    fn call_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z: f64 = rng.sample(StandardNormal);
            let st = s * ((r - 0.5 * sigma.powi(2)) * t + sigma * t.sqrt() * z).exp();
            payoff_sum += (st - k).max(0.0);
        }

        (payoff_sum / self.simulations as f64) * (-r * t).exp()
    }

    fn put_price(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let mut payoff_sum = 0.0;

        for _ in 0..self.simulations {
            let z: f64 = rng.sample(StandardNormal);
            let st = s * ((r - 0.5 * sigma.powi(2)) * t + sigma * t.sqrt() * z).exp();
            payoff_sum += (k - st).max(0.0);
        }

        (payoff_sum / self.simulations as f64) * (-r * t).exp()
    }

    fn delta(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let epsilon = 0.01;
        let price_up = self.call_price(s + epsilon, k, r, sigma, t);
        let price_down = self.call_price(s - epsilon, k, r, sigma, t);
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn gamma(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let epsilon = 0.01;
        let delta_up = self.delta(s + epsilon, k, r, sigma, t);
        let delta_down = self.delta(s - epsilon, k, r, sigma, t);
        (delta_up - delta_down) / (2.0 * epsilon)
    }

    fn vega(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let epsilon = 0.01;
        let price_up = self.call_price(s, k, r, sigma + epsilon, t);
        let price_down = self.call_price(s, k, r, sigma - epsilon, t);
        (price_up - price_down) / (2.0 * epsilon)
    }

    fn theta(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let epsilon = 1.0 / 365.0; // One day
        let price_now = self.call_price(s, k, r, sigma, t);
        let price_future = self.call_price(s, k, r, sigma, t - epsilon);
        (price_future - price_now) / epsilon
    }

    fn rho(&self, s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let epsilon = 0.01;
        let price_up = self.call_price(s, k, r + epsilon, sigma, t);
        let price_down = self.call_price(s, k, r - epsilon, sigma, t);
        (price_up - price_down) / (2.0 * epsilon)
    }
}

