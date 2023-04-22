#!/bin/bash
set -eux
CARGO=cargo
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  platform="linux64"
  if which cross; then
    CARGO=cross
  fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
  platform="osx64"
else
  echo "Unknown OS"
  exit 1
fi

(cd extractor && "$CARGO" build --release)

# If building via cross, the binaries will be in extractor/target/<triple>/release
# If building via cargo, the binaries will be in extractor/target/release
BIN_DIR=extractor/target/release
if [[ "$CARGO" == "cross" ]]; then
  BIN_DIR=extractor/target/x86_64-unknown-linux-gnu/release
fi

"$BIN_DIR/codeql-extractor-solidity" generate --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll

codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll

rm -rf extractor-pack
mkdir -p extractor-pack
cp -r codeql-extractor.yml downgrades tools ql/lib/solidity.dbscheme ql/lib/solidity.dbscheme.stats extractor-pack/
mkdir -p extractor-pack/tools/${platform}
cp "$BIN_DIR/codeql-extractor-solidity" extractor-pack/tools/${platform}/extractor
