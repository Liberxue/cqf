extern crate core;

use core::models::monte_carlo::MonteCarloModel;
use core::models::{OptionParameters, OptionPricingModel};

#[test]
fn test_call_price() {
    let model = MonteCarloModel {
        simulations: 100000,
        epsilon: 0.01,
    };
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let call_price = model.call_price(&params);
    assert!((call_price - 10.45).abs() < 1.0);
}

#[test]
fn test_put_price() {
    let model = MonteCarloModel {
        simulations: 100000,
        epsilon: 0.01,
    };
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let put_price = model.put_price(&params);
    assert!((put_price - 5.57).abs() < 1.0);
}

