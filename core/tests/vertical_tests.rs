extern crate core;
use core::models::black_scholes::BlackScholesModel;
use core::models::OptionParameters;
use core::strategies::vertical::VerticalSpread;
use core::strategies::OptionStrategy;

#[test]
fn test_bull_call_spread() {
    let model = BlackScholesModel;
    let params_long = OptionParameters {
        s: 100.0,
        k: 105.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let params_short = OptionParameters {
        s: 100.0,
        k: 110.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };

    let bull_call_spread = VerticalSpread::new(&model, params_long, params_short, true);
    let price = bull_call_spread.price();
    println!("Bull Call Spread Option Price: {:.2}", price);
    assert!(price > -10.0 && price < 10.0);
}

#[test]
fn test_bear_put_spread() {
    let model = BlackScholesModel;
    let params_long = OptionParameters {
        s: 100.0,
        k: 105.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let params_short = OptionParameters {
        s: 100.0,
        k: 110.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let bear_put_spread = VerticalSpread::new(&model, params_long, params_short, false);
    let price = bear_put_spread.price();
    println!("Bear Put Spread Option Price: {:.2}", price);
    assert!(price > -10.0 && price < 10.0);
}
