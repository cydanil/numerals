# Convert roman numerals to Arabic 
This little utility converts valid Roman numerals to Arabic.

There are a few rules to observe in checking the validity of a roman number:
- Having two subtraction in a row is illegal:
     `IXC` does not equal `91 (C - (X - I))`
- Having four similar numerals in a rwo is illegal:
     `400` should be written as `CD (D - C)`, rather than `CCCC`
- But `IIII` is fine:
     This is typically used by watchmakers to make the reading of the number
     4 easy to read upside down.

The input is expected to be ASCII, although there exist unicode characters
for roman numerals. Apostrophus and Vinculum are not supported.

# Usage
From the command line, use so:

    $ numerals IXC
    Invalid sequence
    
    $ numerals XCIX
    99
