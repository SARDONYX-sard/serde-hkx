# serde-hkx CLI

## About `extra_fmt` version

Special version with json and yaml output in addition to hkx and XML.
The json and yaml representation specifications have not yet been stabilized.

Please consider that json and yaml output from this version will be incompatible with the next version.
This is because the representation will be slightly different depending on what version of the Rust library we use.
This is not a problem if you convert to hkx.

This is a proprietary specification using the library, but unlike XML, it outputs the abstract syntax tree itself, so there is minimal loss of data when converting back to hkx.
