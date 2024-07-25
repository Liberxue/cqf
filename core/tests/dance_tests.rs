extern crate core;
use core::models::black_scholes::BlackScholesModel;
use core::models::OptionParameters;
use core::strategies::dance::Dance;
use core::strategies::OptionStrategy;

#[test]
fn test_dance() {
    let model = BlackScholesModel;
    let params1 = OptionParameters {
        s: 100.0,
        k: 90.0,
        r: 0.05,
        sigma: 0.2,
        t: 0.5,
    };
    let params2 = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 0.5,
    };
    let params3 = OptionParameters {
        s: 100.0,
        k: 110.0,
        r: 0.05,
        sigma: 0.2,
        t: 0.5,
    };
    let dance = Dance::new(&model, params1, params2, params3);
    let price = dance.price();
    println!("Dance Option Price: {:.2}", price);
    assert!(price > 0.0 && price < 100.0);
}
