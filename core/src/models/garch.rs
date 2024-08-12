use crate::models::{OptionParameters, OptionPricingModel};

/// A GARCH(1,1) model for option pricing.
pub struct GarchModel {
    /// Number of steps in the model.
    pub steps: usize,
    /// GARCH model parameters.
    pub omega: f64,
    pub alpha: f64,
    pub beta: f64,
    /// Epsilon value for numerical differentiation.
    pub epsilon: f64,
}

impl GarchModel {
    /// Creates a new `GarchModel` with specified parameters.
    ///
    /// # Arguments
    ///
    /// * `steps` - Number of steps in the model.
    /// * `omega` - GARCH model parameter omega.
    /// * `alpha` - GARCH model parameter alpha.
    /// * `beta` - GARCH model parameter beta.
    pub fn new(steps: usize, omega: f64, alpha: f64, beta: f64, epsilon: f64) -> Self {
        Self {
            steps,
            omega,
            alpha,
            beta,
            epsilon,
        }
    }
}

impl Default for GarchModel {
    fn default() -> Self {
        Self {
            steps: 100,
            omega: 0.1,
            alpha: 0.1,
            beta: 0.8,
            epsilon: 1e-5,
        }
    }
}

impl OptionPricingModel for GarchModel {
    /// Calculates the call option price using the GARCH(1,1) model.
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
        let mut sigma2 = vec![params.sigma * params.sigma; n + 1];
        let mut prices = vec![0.0; n + 1];
        let mut u = vec![0.0; n + 1];
        let mut d = vec![0.0; n + 1];
        let mut q = vec![0.0; n + 1];

        for i in 1..=n {
            sigma2[i] =
                self.omega + self.alpha * params.sigma * params.sigma + self.beta * sigma2[i - 1];
            u[i] = f64::exp(sigma2[i].sqrt() * (dt as f64).sqrt());
            d[i] = 1.0 / u[i];
            q[i] = (f64::exp(params.r * dt as f64) - d[i]) / (u[i] - d[i]);
        }

        for i in 0..=n {
            prices[i] = (params.s * u[n - i].powi(i as i32) * d[n - i].powi((n - i) as i32)
                - params.k)
                .max(0.0);
        }

        for j in (0..n).rev() {
            for i in 0..=j {
                prices[i] = f64::exp(-params.r * dt as f64)
                    * (q[j + 1] * prices[i] + (1.0 - q[j + 1]) * prices[i + 1]);
            }
        }

        prices[0]
    }

    /// Calculates the put option price using the GARCH(1,1) model.
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
        let mut sigma2 = vec![params.sigma * params.sigma; n + 1];
        let mut prices = vec![0.0; n + 1];
        let mut u = vec![0.0; n + 1];
        let mut d = vec![0.0; n + 1];
        let mut q = vec![0.0; n + 1];

        for i in 1..=n {
            sigma2[i] =
                self.omega + self.alpha * params.sigma * params.sigma + self.beta * sigma2[i - 1];
            u[i] = f64::exp(sigma2[i].sqrt() * (dt as f64).sqrt());
            d[i] = 1.0 / u[i];
            q[i] = (f64::exp(params.r * dt as f64) - d[i]) / (u[i] - d[i]);
        }

        for i in 0..=n {
            prices[i] = (params.k
                - params.s * u[n - i].powi(i as i32) * d[n - i].powi((n - i) as i32))
            .max(0.0);
        }

        for j in (0..n).rev() {
            for i in 0..=j {
                prices[i] = f64::exp(-params.r * dt as f64)
                    * (q[j + 1] * prices[i] + (1.0 - q[j + 1]) * prices[i + 1]);
            }
        }

        prices[0]
    }

    /// Calculates the delta of the option using the GARCH(1,1) model.
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
        let sigma2 = params.sigma * params.sigma;
        let u = f64::exp(sigma2.sqrt() * (dt as f64).sqrt());
        let d = 1.0 / u;

        let up_params = OptionParameters {
            s: params.s * u,
            ..params.clone()
        };
        let down_params = OptionParameters {
            s: params.s * d,
            ..params.clone()
        };

        let call_up = self.call_price(&up_params);
        let call_down = self.call_price(&down_params);

        let delta = (call_up - call_down) / (params.s * (u - d));
        delta.min(1.0).max(-1.0)
    }
    /// Calculates the gamma of the option using the GARCH(1,1) model.
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
        let u = f64::exp(params.sigma * (dt as f64).sqrt());
        let d = 1.0 / u;

        let up_params = OptionParameters {
            s: params.s * u,
            ..params.clone()
        };
        let down_params = OptionParameters {
            s: params.s * d,
            ..params.clone()
        };

        let delta_up = self.delta(&up_params);
        let delta_down = self.delta(&down_params);

        (delta_up - delta_down) / (0.5 * params.s * (u - d))
    }

    /// Calculates the theta of the option using the GARCH(1,1) model.
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

    /// Calculates the vega of the option using the GARCH(1,1) model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated vega.
    fn vega(&self, params: &OptionParameters) -> f64 {
        let epsilon = self.epsilon;
        let new_params = OptionParameters {
            sigma: params.sigma + epsilon,
            ..params.clone()
        };
        let call_price_sigma1 = self.call_price(params);
        let call_price_sigma2 = self.call_price(&new_params);

        (call_price_sigma2 - call_price_sigma1) / epsilon
    }

    /// Calculates the rho of the option using the GARCH(1,1) model.
    ///
    /// # Arguments
    ///
    /// * `params` - A reference to `OptionParameters` containing the parameters for the option.
    ///
    /// # Returns
    ///
    /// The calculated rho.
    fn rho(&self, params: &OptionParameters) -> f64 {
        let epsilon = self.epsilon;
        let new_params = OptionParameters {
            r: params.r + epsilon,
            ..params.clone()
        };
        let call_price_r1 = self.call_price(params);
        let call_price_r2 = self.call_price(&new_params);

        let rho = (call_price_r2 - call_price_r1) / epsilon;
        rho.max(0.0)
    }
}

