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
```
