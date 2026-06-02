# Fraud detection: use Benford for basic sanity check against fraud
This Rust program is intended to help analysts -- anyone looking at a lot of numbers -- to run a simple sanity-check from the CLI to gauge whether there might be fraudulent numbers in some PDF file under review.  The source of numbers could be anything -- share prices for stocks, corporate budget numbers, whatever.  The wider the distribution of numbers, the better the analysis.

DISCLAIMER: this is not a bulletproof way of detecting fraudulent numbers.  It merely is one way to note what might be aberrant numbers in some distribution.

# Benford's Law
See the [wikipedia article](https://en.wikipedia.org/wiki/Benford's_law) for learning more about the calculations we do in this extension.


# How to Run the Rust program
