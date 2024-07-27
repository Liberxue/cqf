use crate::models::{OptionParameters, OptionPricingModel};
use rand_distr::num_traits::real::Real;

// <https://www.kent.ac.uk/learning/documents/slas-documents/Binomial_models.pdf >
// <https://www.le.ac.uk/users/dsgp1/COURSES/DERIVATE/BINOPTION.PDF  >
pub struct BinomialTreeModel {
    /// Number of steps in the binomial tree model.
    pub steps: usize,
}

impl BinomialTreeModel {
    /// Creates a new `BinomialTreeModel` with a specified number of steps.
    ///
    /// # Arguments
    ///
    /// * `steps` - Number of steps in the binomial tree model.
    pub fn new(steps: usize) -> Self {
        Self { steps }
    }
}

impl Default for BinomialTreeModel {
    fn default() -> Self {
        Self { steps: 100 } // Default number of steps is 100
    }
}

impl OptionPricingModel for BinomialTreeModel {
    /// Calculates the call option price using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated call option price.
    fn call_price(&self, params: &OptionParameters) -> f64 {
        let n = self.steps; // Number of steps in the binomial tree
        let dt = params.t / (n as f64); // Time step size
        let u = Real::exp(params.sigma * (dt as f64).sqrt()); // Up factor
        let d = 1.0 / u; // Down factor
        let q = (Real::exp(params.r * dt as f64) - d) / (u - d); // Risk-neutral probability
        let mut prices = vec![0.0; n + 1];

        // Terminal prices
        for i in 0..=n {
            prices[i] = (params.s * u.powi((n - i) as i32) * d.powi(i as i32) - params.k).max(0.0);
        }

        // Backward induction
        for j in (0..n).rev() {
            for i in 0..=j {
                prices[i] =
                    Real::exp(-params.r * dt as f64) * (q * prices[i] + (1.0 - q) * prices[i + 1]);
            }
        }

        prices[0]
    }

    /// Calculates the put option price using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated put option price.
    fn put_price(&self, params: &OptionParameters) -> f64 {
        let n = self.steps; // Number of steps in the binomial tree
        let dt = params.t / (n as f64); // Time step size
        let u = Real::exp(params.sigma * (dt as f64).sqrt()); // Up factor
        let d = 1.0 / u; // Down factor
        let q = (Real::exp(params.r * dt as f64) - d) / (u - d); // Risk-neutral probability
        let mut prices = vec![0.0; n + 1];
        for i in 0..=n {
            prices[i] = (params.k - params.s * u.powi((n - i) as i32) * d.powi(i as i32)).max(0.0);
        }
        // Backward induction
        for j in (0..n).rev() {
            for i in 0..=j {
                prices[i] =
                    Real::exp(-params.r * dt as f64) * (q * prices[i] + (1.0 - q) * prices[i + 1]);
            }
        }

        prices[0]
    }

    /// Calculates the delta of the option using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated delta.
    fn delta(&self, params: &OptionParameters) -> f64 {
        let n = self.steps;
        let dt = params.t / (n as f64);
        let u = Real::exp(params.sigma * (dt as f64).sqrt());
        let d = 1.0 / u;

        let up_params = OptionParameters {
            s: params.s * u,
            ..params.clone()
        };
        let params = OptionParameters {
            s: params.s * d,
            ..params.clone()
        };

        let down_params = OptionParameters {
            s: params.s * d,
            ..params.clone()
        };

        let delta_up = self.call_price(&up_params);
        let delta_down = self.call_price(&down_params);

        (delta_up - delta_down) / (params.s * (u - d))
    }

    /// Calculates the gamma of the option using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated gamma.
    fn gamma(&self, params: &OptionParameters) -> f64 {
        let n = self.steps;
        let dt = params.t / (n as f64);
        let u = Real::exp(params.sigma * (dt as f64).sqrt());
        let d = 1.0 / u;

        let delta_up = self.delta(&OptionParameters {
            s: params.s * u,
            ..params.clone()
        });
        let delta_down = self.delta(&OptionParameters {
            s: params.s * d,
            ..params.clone()
        });

        (delta_up - delta_down) / (0.5 * params.s * (u - d))
    }
    /// Calculates the theta of the option using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated theta.
    fn theta(&self, params: &OptionParameters) -> f64 {
        let epsilon = 1e-5;
        let new_params = OptionParameters {
            t: params.t - epsilon,
            ..params.clone()
        };
        let call_price_t1 = self.call_price(params);
        let call_price_t2 = self.call_price(&new_params);

        (call_price_t2 - call_price_t1) / epsilon
    }
    /// Calculates the vega of the option using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calcula           ted vega.
    fn vega(&self, params: &OptionParameters) -> f64 {
        let epsilon = 1e-5;

        let call_price_sigma1 = self.call_price(params);
        let call_price_sigma2 = self.call_price(&OptionParameters {
            sigma: params.sigma + epsilon,
            ..params.clone()
        });

        (call_price_sigma2 - call_price_sigma1) / epsilon
    }

    /// Calculates the rho of the option using the binomial tree model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated rho.
    fn rho(&self, params: &OptionParameters) -> f64 {
        let epsilon = 1e-5;
        let new_params = OptionParameters {
            r: params.r + epsilon,
            ..params.clone()
        };
        let call_price_r1 = self.call_price(params);
        let call_price_r2 = self.call_price(&new_params);

        (call_price_r2 - call_price_r1) / epsilon
    }
}
