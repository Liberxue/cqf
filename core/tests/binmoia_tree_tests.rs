extern crate core;
use core::models::{BinomialTreeModel, OptionParameters, OptionPricingModel};

#[test]
fn test_call_price() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let price = model.call_price(&params);
    println!("Call Price: {}", price);
    assert!(price >= 0.0);
}

#[test]
fn test_put_price() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let price = model.put_price(&params);
    println!("Put Price: {}", price);
    assert!(price >= 0.0);
}

#[test]
fn test_delta() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let delta = model.delta(&params);
    println!("Delta: {}", delta);
    assert!(delta >= -1.0 && delta <= 1.0);
}

#[test]
fn test_gamma() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let gamma = model.gamma(&params);
    assert!(gamma >= 0.0);
}

#[test]
fn test_theta() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let theta = model.theta(&params);
    println!("Theta: {}", theta);
    assert!(theta <= 0.0);
}

#[test]
fn test_vega() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let vega = model.vega(&params);
    assert!(vega >= 0.0);
}

#[test]
fn test_rho() {
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let model = BinomialTreeModel::default();
    let rho = model.rho(&params);
    assert!(rho >= 0.0);
}
