pub struct BlackScholesModel;

impl BlackScholesModel {
    pub fn call(s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let d1 = (1.0 / (sigma * t.sqrt())) * ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t);
        let d2 = d1 - sigma * t.sqrt();
        s * standard_normal_cdf(d1) - k * (-r * t).exp() * standard_normal_cdf(d2)
    }

    pub fn put(s: f64, k: f64, r: f64, sigma: f64, t: f64) -> f64 {
        let d1 = (1.0 / (sigma * t.sqrt())) * ((s / k).ln() + (r + 0.5 * sigma.powi(2)) * t);
        let d2 = d1 - sigma * t.sqrt();
        k * (-r * t).exp() * standard_normal_cdf(-d2) - s * standard_normal_cdf(-d1)
    }
}

fn standard_normal_cdf(x: f64) -> f64 {
    (1.0 + erf(x / 2.0_f64.sqrt())) / 2.0
}

fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    // Save the sign of x
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    // A&S formula 7.1.26
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

