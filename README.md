# Numerals
This little utility converts valid Roman numerals to Arabic, and vice-versa

## Conversion from Roman numerals
There are a few rules to observe in checking the validity of a roman number:
- Having two subtraction in a row is illegal:
     `IXC` does not equal `91 (C - (X - I))`
- Having four similar numerals in a row is illegal:
     `400` should be written as `CD (D - C)`, rather than `CCCC`
- But `IIII` is fine:
     This is typically used by watchmakers to make the reading of the number
     4 easy to read upside down.
- `L` and `D` cannot be repeated:
    `LL` should be `C`, and `DD` should be `M`
- If a certain sequence can be represented with another symbol, it is illegal:
    `LC` should be `L`

The input is expected to be ASCII, although there exist unicode characters
for roman numerals. Apostrophus and Vinculum are not supported.

## Conversion from Arabic numerals
Due to the rule limiting 4 consecutive Roman numerals, the greatest Arabic
numeral is 3999.

# Usage
From the command line, use so:

    $ numerals IXC
    Invalid sequence
    
    $ numerals IC
    99

    $ numerals 198
    CXCVIII

# Known issues
Any value ending in `9`, short of `9` itself, will accept invalid input or
return invalid output.  
For instance, the value `XCIX` is accepted and returned as 99, although it
should reject it and return `IC`.
