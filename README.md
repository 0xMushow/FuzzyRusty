# FuzzyRusty
A fuzzer built in Rust

## Features
- Coverage guided
- Roundtrip fuzzing
- Mutations handled by libFuzzer
- Custom mutations to reduce the number of iterations

# Usage

To use it, run the following command:

```bash
cargo fuzz run <FUZZ_TARGET> -- -max_len=150000
```

# Coverage

You'll find the coverage report in the generated repository in the HTML format

```bash
./generate_coverage.sh <FUZZ_TARGET>
```

# Learnings from this take-home

- Custom mutations helps the fuzzer to achieve better coverage more rapidly with less iterations and better corpora
- Good numbers for `cov`, `ft`, `corp` and `execs` for the fuzzer
- The fuzzer checks all the main functions (`serialize`, `deserialize`, `is_valid_merkle_branch`)
- Different fuzzing techniques

# Other techniques

- Differential fuzzing could be interesting, although similar libraries don't seem to share the exact same functions
- Grammar fuzzing (would take more time to setup) - a bit more complex

# Tools

- For this I used libFuzzer & cargo-fuzz
- There is also LibAFL & AFL++ (maintained by SR Labs)
- There is also [Ziggy](https://github.com/srlabs/ziggy) (maintained by SR Labs too)
- I looked up this oss-fuzz [list](https://physics.bu.edu/~alxndr/oss-fuzz-corpus.html), but didn't find relevant corpora to use