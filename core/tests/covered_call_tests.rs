extern crate core;

use core::models::black_scholes::BlackScholesModel;
use core::models::OptionParameters;
use core::strategies::covered_call::CoveredCall;
use core::strategies::OptionStrategy;

#[test]
fn test_covered_call() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let covered_call = CoveredCall::new(&model, params);
    let price = covered_call.price();
    println!("Covered Call Option Price: {:.2}", price);
    assert!(price > 0.0 && price < 100.0);
}
