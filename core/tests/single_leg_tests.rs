extern crate core;

use core::models::black_scholes::BlackScholesModel;
use core::models::OptionParameters;
use core::strategies::single_leg::SingleLegOption;
use core::strategies::OptionStrategy;

#[test]
fn test_single_leg_call() {
    let model = BlackScholesModel;
    let parmas = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let single_leg_call = SingleLegOption::new(&model, parmas, true);
    let price = single_leg_call.price();
    assert!((price - 10.45).abs() < 0.1);
}

#[test]
fn test_single_leg_put() {
    let model = BlackScholesModel;
    let parmas = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let single_leg_put = SingleLegOption::new(&model, parmas, false);
    let price = single_leg_put.price();
    assert!((price - 5.57).abs() < 0.1);
}
