extern crate cqf;

use cqf::models::black_scholes::BlackScholesModel;
use cqf::strategies::collar::Collar;
use cqf::strategies::OptionStrategy;

#[test]
fn test_collar() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k1 = 95.0;
    let k2 = 105.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let collar = Collar::new(&model, s, k1, k2, r, sigma, t);
    let price = collar.price();
    println!("Collar Option Price: {:.2}", price);
    assert!(price < 0.0 && price > -10.0); 
}

