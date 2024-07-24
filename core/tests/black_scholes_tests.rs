extern crate core;

use core::models::black_scholes::BlackScholesModel;
use core::models::OptionPricingModel;

#[test]
fn test_black_scholes_call() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let call_price = model.call_price(s, k, r, sigma, t);
    assert!((call_price - 10.45).abs() < 0.1);
}

#[test]
fn test_black_scholes_put() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let put_price = model.put_price(s, k, r, sigma, t);
    assert!((put_price - 5.57).abs() < 0.1);
}

#[test]
fn test_delta() {
    let model = BlackScholesModel;
    let delta = model.delta(100.0, 100.0, 0.05, 0.2, 1.0);
    dbg!(delta);
    dbg!((delta - 0.63).abs());
    assert!(
        (delta - 0.63).abs() < 0.1,
        "Delta should be approximately 0.63"
    );
}

#[test]
fn test_gamma() {
    let model = BlackScholesModel;
    let gamma = model.gamma(100.0, 100.0, 0.05, 0.2, 1.0);
    assert!(
        (gamma - 0.02).abs() < 0.01,
        "Gamma should be approximately 0.02"
    );
}

#[test]
fn test_vega() {
    let model = BlackScholesModel;
    let vega = model.vega(100.0, 100.0, 0.05, 0.2, 1.0);
    assert!(
        (vega - 0.29).abs() < 0.1,
        "Vega should be approximately 0.29"
    );
}

#[test]
fn test_theta() {
    let model = BlackScholesModel;
    let theta = model.theta(100.0, 100.0, 0.05, 0.2, 1.0);
    assert!(
        (theta - -0.04).abs() < 0.02,
        "Theta should be approximately -0.04"
    );
}

#[test]
fn test_rho() {
    let model = BlackScholesModel;
    let rho = model.rho(100.0, 100.0, 0.05, 0.2, 1.0);
    assert!((rho - 8.77).abs() < 1.0, "Rho should be approximately 8.77");
}
