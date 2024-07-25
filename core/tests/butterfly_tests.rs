extern crate core;

use core::models::{BlackScholesModel, OptionParameters};
use core::strategies::butterfly::ButterflySpread;
use core::strategies::OptionStrategy;

#[test]
fn test_butterfly_spread() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 95.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let butterfly_spread = ButterflySpread::new(&model, params, 100.0, 105.0);
    let price = butterfly_spread.price();
    println!("Butterfly Spread Option Price: {:.2}", price);
    assert!(price > 0.0 && price < 10.0);
}
