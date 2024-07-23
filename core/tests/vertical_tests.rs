extern crate cqf;

use cqf::core::models::black_scholes::BlackScholesModel;
use cqf::core::strategies::vertical::VerticalSpread;
use cqf::core::strategies::OptionStrategy;

#[test]
fn test_bull_call_spread() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k1 = 95.0;
    let k2 = 105.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let bull_call_spread = VerticalSpread::new(&model, s, k1, k2, r, sigma, t, true);
    let price = bull_call_spread.price();
    println!("Bull Call Spread Option Price: {:.2}", price);
    assert!(price > -10.0 && price < 10.0);
}

#[test]
fn test_bear_put_spread() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k1 = 95.0;
    let k2 = 105.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let bear_put_spread = VerticalSpread::new(&model, s, k1, k2, r, sigma, t, false);
    let price = bear_put_spread.price();
    println!("Bear Put Spread Option Price: {:.2}", price);
    assert!(price > -10.0 && price < 10.0);
}
