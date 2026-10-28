# Fraud detection: use Benford for basic sanity check against fraud
This Rust program is intended to help analysts -- anyone looking at a lot of numbers -- to run a simple sanity-check from the CLI to gauge whether there might be fraudulent numbers in some PDF file under review.  The source of numbers could be anything -- share prices for stocks, corporate budget numbers, whatever.  The wider the distribution of numbers, the better the analysis.

DISCLAIMER: this is not a bulletproof way of detecting fraudulent numbers.  It merely is one way to note what might be aberrant numbers in some distribution.

# Benford's Law
See the [wikipedia article](https://en.wikipedia.org/wiki/Benford's_law) for learning more about the calculations we do in this extension.


# How to Run the Rust program
For the February, 2026, report from the Bureau of Labor, we can run our analysis as follows:
```
cargo run --release -- examples/empsit_03062026.pdf
```
You should see something like this in your terminal:
```
=== Analysis Complete ===
Total numbers analyzed: 6979

Digit | Observed Freq | Benford Freq  | Distribution Comparison (Observed: #, Benford: *)
------+---------------+---------------+---------------------------------------------------
  1   |   28.6% (1994) |    30.1%     | Obs: ###################
      |               |               | Ben: ********************
------+---------------+---------------+---------------------------------------------------
  2   |   18.3% (1277) |    17.6%     | Obs: ############
      |               |               | Ben: ************
------+---------------+---------------+---------------------------------------------------
  3   |   15.0% (1048) |    12.5%     | Obs: ##########
      |               |               | Ben: ********
------+---------------+---------------+---------------------------------------------------
  4   |    9.6% ( 667) |     9.7%     | Obs: ######
      |               |               | Ben: ******
------+---------------+---------------+---------------------------------------------------
  5   |    8.5% ( 595) |     7.9%     | Obs: ######
      |               |               | Ben: *****
------+---------------+---------------+---------------------------------------------------
  6   |    7.3% ( 512) |     6.7%     | Obs: #####
      |               |               | Ben: ****
------+---------------+---------------+---------------------------------------------------
  7   |    4.9% ( 345) |     5.8%     | Obs: ###
      |               |               | Ben: ****
------+---------------+---------------+---------------------------------------------------
  8   |    4.4% ( 310) |     5.1%     | Obs: ###
      |               |               | Ben: ***
------+---------------+---------------+---------------------------------------------------
  9   |    3.3% ( 231) |     4.6%     | Obs: ##
      |               |               | Ben: ***
------+---------------+---------------+---------------------------------------------------

=== Nigrini's Mean Absolute Deviation (MAD) ===
MAD Score: 0.00991
Conformity: Acceptable Conformity

=== Chi-Square Goodness-of-Fit Test ===
Chi-Square Statistic (χ²): 89.9750
Critical Threshold (α = 0.05, df = 8): 15.51
Result: EXCEEDS threshold (89.9750 > 15.51). The dataset significantly deviates from Benford's Law.

=== Individual Digit Z-Score Analysis ===
Sorted by absolute deviation magnitude (Critical |Z| = 1.96, α = 0.05)

 Digit | Obs Count | Exp Count | Obs Pct | Exp Pct |  Z-Score  | Flagged for Review 
-------+-----------+-----------+---------+---------+-----------+--------------------
   3   |     1048  |     871.9 |   15.0% |   12.5% |    +6.355  | YES [FLAGGED]
   9   |      231  |     319.3 |    3.3% |    4.6% |    -5.032  | YES [FLAGGED]
   7   |      345  |     404.7 |    4.9% |    5.8% |    -3.033  | YES [FLAGGED]
   1   |     1994  |    2100.9 |   28.6% |   30.1% |    -2.776  | YES [FLAGGED]
   8   |      310  |     357.0 |    4.4% |    5.1% |    -2.526  | YES [FLAGGED]
   6   |      512  |     467.2 |    7.3% |    6.7% |    +2.121  | YES [FLAGGED]
   5   |      595  |     552.6 |    8.5% |    7.9% |    +1.857  | No
   2   |     1277  |    1228.9 |   18.3% |   17.6% |    +1.495  | No
   4   |      667  |     676.3 |    9.6% |    9.7% |    -0.357  | No
------------------------------------------------------------------------------------
```
