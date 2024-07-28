**Custom Neuron Decision-Making and Visual Workflow Orchestration Quantitative**

 
[![](https://img.shields.io/badge/Rust-1.79.0+-blue)](https://releases.rs/docs/1.79.0)   

## Examples

<details>
  <summary> Models Example   </summary>
  
#### Models Example 
  
```rust
    let model = BlackScholesModel; // BinomialTreeModel OR BlackScholesModel GarchModel MonteCarloModel ...
    let params = OptionParameters {
        s: opts.s,
        k: opts.k,
        r: opts.r,
        sigma: opts.sigma,
        t: opts.t,
    };

    let call_price = model.call_price(&params);
    let put_price = model.put_price(&params);
```

</details>

<details>
  <summary> Strategies Example  </summary>
  
#### [Strategies Example](core/src/tests)
```rust
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
        t: 0.5
    };
    let dance = Dance::new(&model, params1, params2, params3);
    let price = dance.price();
    assert!(price > 0.0 && price < 100.0);
}
```
</details>


## Quantitative Models

### [Binomial Tree Model](core/src/models/binomial_tree.rs)
**U**sed for option pricing by constructing a binomial tree to represent possible paths an asset's price could take over time. It is particularly useful for valuing American options, which can be exercised at any time before expiration.


### [Black-Scholes Model](core/src/models/black_scholes.rs)
**U**sed model for pricing European options. It assumes that the price of the underlying asset follows a geometric Brownian motion with constant volatility and interest rate. The model provides a closed-form solution for option pricing.

### [Monte Carlo Model](core/src/models/monte_carlo.rs)
**U**sed to value options by simulating a large number of possible price paths for the underlying asset. It is particularly useful for valuing complex derivatives and options with path-dependent features, as it can accommodate various stochastic processes and payoff structures.


### [GARCH Model](core/src/models/garch.rs)

**U**sed for modeling financial time series data that exhibit volatility clustering. It extends the ARCH model by allowing past variances to influence current variances, providing a more flexible approach to volatility modeling.

###  GARCH/AGARCH Model More  
<details>
  <summary> Click More 100+ Model  </summary>
  
  | **Model Name**    | **Description**                                           |
  |-------------------|-----------------------------------------------------------|
  | AARCH             | Handles asymmetric volatility in time series              |
  | DVEC-GARCH        | Uses diagonal vector model to handle multivariate data volatility |
  | GARJI             | Combines GARCH model with jumps to capture sudden price changes |
  | MS-GARCH          | Combines Markov state switching with GARCH model          |
  | SPARCH            | Handles smooth transitions in volatility                  |
  | ADCC-GARCH        | Handles asymmetric dynamic conditional correlation        |
  | EGARCH            | Uses exponential function to handle asymmetric volatility |
  | GDCC-GARCH        | A generalized dynamic conditional correlation model       |
  | MV-GARCH          | Handles multivariate data volatility                      |
  | Spline-GARCH      | Uses spline functions to model volatility                 |
  | AGARCH            | An adjusted GARCH model for better fit                    |
  | EVT-GARCH         | Incorporates extreme value theory into GARCH modeling     |
  | GED-GARCH         | Uses Generalized Error Distribution for modeling          |
  | NAGARCH           | Nonlinear asymmetric GARCH model                          |
  | SQR-GARCH         | Uses squared returns in GARCH model                       |
  | ANN-ARCH          | Uses artificial neural networks with ARCH model           |
  | F-ARCH            | Fractionally integrated ARCH model                        |
  | GJR-GARCH         | Threshold GARCH model that captures leverage effect       |
  | NGARCH            | Nonlinear GARCH model                                     |
  | STARCH            | Smooth transition ARCH model                              |
  | ANST-GARCH        | Asymmetric nonlinear smooth transition GARCH model        |
  | FDCC-GARCH        | Flexible dynamic conditional correlation GARCH model      |
  | GO-GARCH          | Generalized orthogonal GARCH model                        |
  | NL-GARCH          | Nonlinear GARCH model                                     |
  | Stdev-ARCH        | Standard deviation ARCH model                             |
  | APARCH            | Asymmetric power ARCH model                               |
  | FGARCH            | Flexible GARCH model                                      |
  | GQARCH            | Quadratic GARCH model                                     |
  | NM-GARCH          | Nonparametric GARCH model                                 |
  | STGARCH           | Smooth transition GARCH model                             |
  | ARCH-M            | ARCH-in-mean model                                        |
  | FIAPARCH          | Fractionally integrated asymmetric power ARCH model       |
  | GQTARCH           | Generalized quadratic ARCH model                          |
  | OGARCH            | Orthogonal GARCH model                                    |
  | Structural GARCH  | Models structural changes in volatility                   |
  | ARCH-SM           | Stochastic mean ARCH model                                |
  | FIEGARCH          | Fractionally integrated EGARCH model                      |
  | HARCH             | Hierarchical ARCH model                                   |
  | PARCH             | Power ARCH model                                          |
  | Strong GARCH      | Robust GARCH model                                        |
  | ATGARCH           | Adaptive threshold GARCH model                            |
  | FIGARCH           | Fractionally integrated GARCH model                       |
  | HGARCH            | Heteroscedastic GARCH model                               |
  | PC-GARCH          | Principal component GARCH model                           |
  | SWARCH            | Switching ARCH model                                      |
  | Aug-GARCH         | Augmented GARCH model                                     |
  | FIREGARCH         | Fractionally integrated random effects GARCH model        |
  | HYGARCH           | Hyperbolic GARCH model                                    |
  | PGARCH            | Polynomial GARCH model                                    |
  | TGARCH            | Threshold GARCH model                                     |
  | AVGARCH           | Average GARCH model                                       |
  | Flex-GARCH        | Flexible GARCH model                                      |
  | IGARCH            | Integrated GARCH model                                    |
  | PNP-GARCH         | Penalized nonparametric GARCH model                       |
  | t-GARCH           | Student-t GARCH model                                     |
  | B-GARCH           | Bayesian GARCH model                                      |
  | GAARCH            | Generalized asymmetric ARCH model                         |
  | LARCH             | Linear ARCH model                                         |
  | QARCH             | Quadratic ARCH model                                      |
  | Tobit-GARCH       | Tobit GARCH model                                         |
  | BEKK-GARCH        | Baba, Engle, Kraft and Kroner GARCH model                 |
  | GARCH-Delta       | Delta GARCH model                                         |
  | Latent GARCH      | Latent variable GARCH model                               |
  | QTARCH            | Quantile threshold ARCH model                             |
  | TS-GARCH          | Time series GARCH model                                   |
  | CCC-GARCH         | Constant conditional correlation GARCH model              |
  | GARCH Diffusion   | Diffusion GARCH model                                     |
  | Level GARCH       | Level shift GARCH model                                   |
  | REGARCH           | Robust and efficient GARCH model                          |
  | UGARCH            | Univariate GARCH model                                    |
  | Censored-GARCH    | Censored GARCH model                                      |
  | GARCH-EAR         | GARCH model with expected average returns                 |
  | LGARCH            | Logarithmic GARCH model                                   |
  | RGARCH            | Robust GARCH model                                        |
  | VCC-GARCH         | Varying coefficient correlation GARCH model               |
  | CGARCH            | Component GARCH model                                     |
  | GARCH-Gamma       | GARCH model with gamma distribution                       |
  | LMGARCH           | Log-mean GARCH model                                      |
  | Robust GARCH      | Robust GARCH model                                        |
  | VGARCH            | Vector GARCH model                                        |
  | COGARCH           | Continuous-time GARCH model                               |
  | GARCH-M           | GARCH-in-mean model                                       |
  | Log-GARCH         | Logarithmic GARCH model                                   |
  | Root GARCH        | Root GARCH model                                          |
  | VSGARCH           | Volatility spillover GARCH model                          |
  | CorrARCH          | Correlation ARCH model                                    |
  | GARCHS            | Seasonal GARCH model                                      |
  | MAR-ARCH          | Multivariate ARCH model                                   |
  | RS-GARCH          | Regime switching GARCH model                              |
  | Weak GARCH        | Weak GARCH model                                          |
  | DAGARCH           | Diagonal ARCH model                                       |
  | GARCHSK           | GARCH model with skewness                                 |
  | MARCH             | Moving average ARCH model                                 |
  | Robust DCC-GARCH  | Robust dynamic conditional correlation GARCH model        |
  | ZARCH             | Zero-inflated ARCH model                                  |
  | DCC-GARCH         | Dynamic conditional correlation GARCH model               |
  | GARCH-t           | GARCH model with t-distribution                           |
  | Matrix EGARCH     | Matrix exponential GARCH model                            |
  | SGARCH            | Seasonal GARCH model                                      |
  | Diag MGARCH       | Diagonal multivariate GARCH model                         |
  | GARCH-X           | GARCH model with exogenous variables                      |
  | MGARCH            | Multivariate GARCH model                                  |
  | S-GARCH           | Smooth GARCH model                                        |
  | DTARCH            | Double threshold ARCH model                               |
  | GARCHX            | GARCH model with explanatory variables                    |
  | Mixture GARCH     | Mixture of GARCH models                                   |
  | Sign-GARCH        | GARCH model with sign-dependent effects                   |
</details>

# Contributing

Contributions are welcome! [Please open an issue](https://github.com/Liberxue/cqf/issues/new) or [submit PR ] (https://github.com/Liberxue/cqf/pulls) for any improvements or new features.

