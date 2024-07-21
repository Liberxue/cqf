 The Certificate in Quantitative Finance | CQF For Rust Example


# Example -> Option Strategies

## Single Leg Option

**Documentation:**
[Investopedia - Single Leg Option](https://www.investopedia.com/terms/s/single-option.aspx)

**Example:**

```rust
let call_price = SingleLegOption::call(&model, s, k, r, sigma, t);
let put_price = SingleLegOption::put(&model, s, k, r, sigma, t);
```

## Butterfly Spread

**Documentation:**
[Investopedia - Butterfly Spread](https://www.investopedia.com/terms/b/butterflyspread.asp)

**Example:**

```rust
let butterfly_spread = ButterflySpread::new(&model, s, k1, k2, k3, r, sigma, t);
let price = butterfly_spread.price();
```

## Vertical Spread

**Documentation:**
[Investopedia - Vertical Spread](https://www.investopedia.com/terms/v/verticalspread.asp)

**Example:**

```rust
let bull_call_spread = VerticalSpread::new(&model, s, k1, k2, r, sigma, t, true);
let bear_put_spread = VerticalSpread::new(&model, s, k1, k2, r, sigma, t, false);
```
## Covered Call

**Documentation:**
[Investopedia - Covered Call](https://www.investopedia.com/terms/c/coveredcall.asp)

**Example:**

```rust
let covered_call = CoveredCall::new(&model, s, k, r, sigma, t);
let price = covered_call.price();
```
## Collar

**Documentation:**
[Investopedia - Collar](https://www.investopedia.com/terms/c/collar.asp)

**Example:**

```rust
let collar = Collar::new(&model, s, k1, k2, r, sigma, t);
let price = collar.price();
```

## Straddle

**Documentation:**
[Investopedia - Straddle](https://www.investopedia.com/terms/s/straddle.asp)

**Example:**

```rust
let straddle = Straddle::new(&model, s, k, r, sigma, t);
let price = straddle.price();
```

## Strangle

**Documentation:**
[Investopedia - Strangle](https://www.investopedia.com/terms/s/strangle.asp)

**Example:**

```rust
let strangle = Strangle::new(&model, s, k1, k2, r, sigma, t);
let price = strangle.price();
```

## Calendar Spread

**Documentation:**
[Investopedia - Calendar Spread](https://www.investopedia.com/terms/c/calendarspread.asp)

**Example:**

```rust
let calendar_spread = CalendarSpread::new(&model, s, k, r, sigma, t1, t2);
let price = calendar_spread.price();
```

## Diagonal Spread

**Documentation:**
[Investopedia - Diagonal Spread](https://www.investopedia.com/terms/d/diagonalspread.asp)

**Example:**

```rust
let diagonal_spread = DiagonalSpread::new(&model, s, k1, k2, r, sigma, t1, t2);
let price = diagonal_spread.price();
```

## Condor

**Documentation:**
[Investopedia - Condor](https://www.investopedia.com/terms/c/condor.asp)

**Example:**

```rust
let condor = Condor::new(&model, s, k1, k2, k3, k4, r, sigma, t);
let price = condor.price();
```

## Iron Butterfly

**Documentation:**
[Investopedia - Iron Butterfly](https://www.investopedia.com/terms/i/ironbutterfly.asp)

**Example:**

```rust
let iron_butterfly = IronButterfly::new(&model, s, k1, k2, k3, r, sigma, t);
let price = iron_butterfly.price();
```

## Iron Condor

**Documentation:**
[Investopedia - Iron Condor](https://www.investopedia.com/terms/i/ironcondor.asp)

**Example:**

```rust
let iron_condor = IronCondor::new(&model, s, k1, k2, k3, k4, r, sigma, t);
let price = iron_condor.price();
```

## Dance

**Documentation:**
[Investopedia - Dance](https://www.investopedia.com/terms/d/dance.asp)

**Example:**

```rust
let dance = Dance::new(&model, s, k1, k2, k3, r, sigma, t);
let price = dance.price();
```
# Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or new features.

