# FuzzyRusty
A fuzzer built in Rust

## Features
- Coverage guided
- Roundtrip fuzzing
- Mutations handled by libFuzzer

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