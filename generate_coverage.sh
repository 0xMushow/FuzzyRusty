#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <TARGET>" >&2
  exit 1
fi
TARGET=$1

cargo clean

export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Clink-dead-code"
export LLVM_PROFILE_FILE="$TARGET-%p.profraw"

cargo fuzz build "$TARGET" --release

FUZZBIN=$(echo fuzz/target/*/release/$TARGET)

if [[ ! -x "$FUZZBIN" ]]; then
  echo "❌ Cannot find fuzz binary at $FUZZBIN" >&2
  exit 1
fi

echo "✅ Found fuzz binary: $FUZZBIN"

"$FUZZBIN" \
  -artifact_prefix=fuzz/artifacts/$TARGET/ \
  -max_len=150000 \
  -runs=1 \
  fuzz/corpus/$TARGET

llvm-profdata merge -sparse ${TARGET}-*.profraw -o ${TARGET}.profdata

llvm-cov show \
  -Xdemangler=rustfilt \
  "$FUZZBIN" \
  -instr-profile=${TARGET}.profdata \
  --ignore-filename-regex='.cargo/registry' \
  --show-line-counts-or-regions \
  --format=html \
  --output-dir=coverage/$TARGET

echo "🎉 Coverage report is in ./coverage/$TARGET/index.html"
