# Option Strategies

## Single Leg Option

**Introduction:**
A single leg option involves either buying or selling one call or one put option. It is the simplest form of option trading strategy.

**Mathematical Formulas:**

For a call option:
\[ C = S_0 \Phi(d_1) - K e^{-rT} \Phi(d_2) \]

For a put option:
\[ P = K e^{-rT} \Phi(-d_2) - S_0 \Phi(-d_1) \]

where:
\[ d_1 = \frac{\ln(S_0 / K) + (r + \sigma^2 / 2) T}{\sigma \sqrt{T}} \]
\[ d_2 = d_1 - \sigma \sqrt{T} \]

- [x] Implement Single Leg Option
- [x] Write tests for Single Leg Option

## Butterfly Spread

**Introduction:**
A butterfly spread is a neutral option strategy combining bull and bear spreads, with a fixed risk and capped profit. It's constructed using four options with three different strike prices.

**Mathematical Formula:**
\[ \text{Butterfly Spread Price} = C(K_1) - 2C(K_2) + C(K_3) \]

where \( C(K) \) is the call option price with strike \( K \).

- [x] Implement Butterfly Spread
- [x] Write tests for Butterfly Spread

## Vertical Spread

**Introduction:**
A vertical spread involves buying and selling two options of the same type (calls or puts) with different strike prices but the same expiration date.

**Mathematical Formulas:**

For a bull call spread:
\[ \text{Bull Call Spread Price} = C(K_1) - C(K_2) \]

For a bear put spread:
\[ \text{Bear Put Spread Price} = P(K_1) - P(K_2) \]

- [x] Implement Vertical Spread
- [x] Write tests for Vertical Spread

## Covered Call

**Introduction:**
A covered call involves holding a long position in a stock and selling a call option on the same stock to generate income from the option premium.

**Mathematical Formula:**
\[ \text{Covered Call Price} = S_0 - C(K) \]

- [x] Implement Covered Call
- [x] Write tests for Covered Call

## Collar

**Introduction:**
A collar strategy involves holding the underlying stock, buying a protective put, and writing a covered call.

**Mathematical Formula:**
\[ \text{Collar Price} = P(K_1) - C(K_2) \]

- [x] Implement Collar
- [x] Write tests for Collar

## Straddle

**Introduction:**
A straddle involves buying a call and a put option with the same strike price and expiration date. It profits from large movements in the stock price.

**Mathematical Formula:**
\[ \text{Straddle Price} = C(K) + P(K) \]

- [x] Implement Straddle
- [x] Write tests for Straddle

## Strangle

**Introduction:**
A strangle involves buying a call and a put option with different strike prices but the same expiration date. It profits from large movements in the stock price.

**Mathematical Formula:**
\[ \text{Strangle Price} = C(K_1) + P(K_2) \]

- [x] Implement Strangle
- [x] Write tests for Strangle

## Calendar Spread

**Introduction:**
A calendar spread involves buying and selling two options of the same type (calls or puts) with the same strike price but different expiration dates.

**Mathematical Formula:**
\[ \text{Calendar Spread Price} = C(K, T_2) - C(K, T_1) \]

where \( T_2 > T_1 \).

- [x] Implement Calendar Spread
- [x] Write tests for Calendar Spread

## Diagonal Spread

**Introduction:**
A diagonal spread involves buying and selling two options of the same type (calls or puts) with different strike prices and different expiration dates.

**Mathematical Formula:**
\[ \text{Diagonal Spread Price} = C(K_1, T_2) - C(K_2, T_1) \]

where \( T_2 > T_1 \).

- [x] Implement Diagonal Spread
- [x] Write tests for Diagonal Spread

## Condor

**Introduction:**
A condor is a neutral strategy that profits from low volatility. It involves buying and selling four options with different strike prices but the same expiration date.

**Mathematical Formula:**
\[ \text{Condor Price} = C(K_1) - C(K_2) + C(K_3) - C(K_4) \]

- [x] Implement Condor
- [x] Write tests for Condor

## Iron Butterfly

**Introduction:**
An iron butterfly is a limited risk, limited profit options trading strategy that is designed to benefit from low volatility in the underlying asset.

**Mathematical Formula:**
\[ \text{Iron Butterfly Price} = C(K_2) + P(K_2) - C(K_3) - P(K_1) \]

- [x] Implement Iron Butterfly
- [x] Write tests for Iron Butterfly

## Iron Condor

**Introduction:**
An iron condor is an options strategy that involves buying and selling call and put options with different strike prices but the same expiration date. It profits from low volatility in the underlying asset.

**Mathematical Formula:**
\[ \text{Iron Condor Price} = C(K_2) - C(K_3) + P(K_2) - P(K_1) \]

- [x] Implement Iron Condor
- [x] Write tests for Iron Condor

## Dance

**Introduction:**
A dance strategy involves buying multiple call options with different strike prices. It is designed to profit from large upward movements in the stock price.

**Mathematical Formula:**
\[ \text{Dance Price} = C(K_1) + C(K_2) + C(K_3) \]

- [x] Implement Dance
- [x] Write tests for Dance

