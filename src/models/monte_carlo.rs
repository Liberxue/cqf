extern crate rand;
use rand::Rng;
use rand_distr::StandardNormal;
use crate::models::OptionPricingModel;

pub struct MonteCarloModel {
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
}

