extern crate cqf;
use cqf::strategies::single_leg::SingleLegOption;

#[test]
fn test_single_leg_call() {
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let call_price = SingleLegOption::call(s, k, r, sigma, t);
    assert!((call_price - 10.45).abs() < 0.01);
}

#[test]
fn test_single_leg_put() {
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let put_price = SingleLegOption::put(s, k, r, sigma, t);
    assert!((put_price - 5.57).abs() < 0.01);
}
