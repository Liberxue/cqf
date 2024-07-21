extern crate cqf;
use cqf::strategies::butterfly::ButterflySpread;

#[test]
fn test_butterfly_spread() {
    let s = 100.0;
    let k1 = 95.0;
    let k2 = 100.0;
    let k3 = 105.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let butterfly_price = ButterflySpread::price(s, k1, k2, k3, r, sigma, t);
    assert!(butterfly_price > 0.0);
}
