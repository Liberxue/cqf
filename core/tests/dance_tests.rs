extern crate core;
use core::models::black_scholes::BlackScholesModel;
use core::strategies::dance::Dance;
use core::strategies::OptionStrategy;

#[test]
fn test_dance() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k1 = 95.0;
    let k2 = 100.0;
    let k3 = 105.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let dance = Dance::new(&model, s, k1, k2, k3, r, sigma, t);
    let price = dance.price();
    println!("Dance Option Price: {:.2}", price);
    assert!(price > 0.0 && price < 100.0);
}
