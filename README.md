 The Certificate in Quantitative Finance | CQF For Rust Example


## Installation

To get started, clone the repository and build the project:

```sh
git clone https://github.com/yourusername/cqf.git
cd cqf
cargo test 


# Example

```Rust
fn main() {
    // Example parameters
    let s = 100.0; // Current stock price
    let k1 = 95.0; // Lower strike price
    let k2 = 100.0; // Middle strike price
    let k3 = 105.0; // Higher strike price
    let r = 0.05; // Risk-free interest rate
    let sigma = 0.2; // Volatility
    let t = 1.0; // Time to maturity in years

    // Calculate Butterfly Spread option price
    let butterfly_price = ButterflySpread::price(s, k1, k2, k3, r, sigma, t);
    println!("Butterfly Spread Option Price: {:.2}", butterfly_price);

    // Calculate Single Leg Call option price
    let call_price = SingleLegOption::call(s, k2, r, sigma, t);
    println!("Single Leg Call Option Price: {:.2}", call_price);

    // Calculate Single Leg Put option price
    let put_price = SingleLegOption::put(s, k2, r, sigma, t);
    println!("Single Leg Put Option Price: {:.2}", put_price);
}
```

# Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or new features.

