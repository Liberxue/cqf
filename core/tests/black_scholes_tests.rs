extern crate core;

use core::models::black_scholes::BlackScholesModel;

use core::models::{OptionParameters, OptionPricingModel};

#[test]
fn test_black_scholes_call() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let call_price = model.call_price(&params);
    assert!((call_price - 10.45).abs() < 0.1);
}

#[test]
fn test_black_scholes_put() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let put_price = model.put_price(&params);
    assert!((put_price - 5.57).abs() < 0.1);
}

#[test]
fn test_delta() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let delta = model.delta(&params);
    assert!(
        (delta - 0.6368).abs() < 0.01,
        "Delta should be approximately 0.6368"
    );
}

#[test]
fn test_gamma() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let gamma = model.gamma(&params);
    assert!(
        (gamma - 0.0188).abs() < 0.001,
        "Gamma should be approximately 0.0188"
    );
}

#[test]
fn test_vega() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let vega = model.vega(&params);
    assert!(
        (vega - 37.69).abs() < 0.2,
        "Vega should be approximately 37.69"
    );
}

#[test]
fn test_theta() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let theta = model.theta(&params);
    assert!(
        (theta - -0.0188).abs() < 0.02,
        "Theta should be approximately -0.0188"
    );
}

#[test]
fn test_rho() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let rho = model.rho(&params);
    assert!(
        (rho - 0.5323).abs() < 0.1,
        "Rho should be approximately 0.5323"
    );
}
