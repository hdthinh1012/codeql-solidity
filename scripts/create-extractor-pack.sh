#!/bin/bash
set -eux

if [[ "$OSTYPE" == "linux-gnu"* ]]; then
  platform="linux64"
elif [[ "$OSTYPE" == "darwin"* ]]; then
  platform="osx64"
else
  echo "Unknown OS"
  exit 1
fi

cargo build --release

cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll

rm -rf extractor-pack
mkdir -p extractor-pack
cp -r codeql-extractor.yml tools ql/lib/solidity.dbscheme extractor-pack/
mkdir -p extractor-pack/tools/${platform}
cp target/release/solidity-extractor extractor-pack/tools/${platform}/extractor
cp target/release/solidity-autobuilder extractor-pack/tools/${platform}/autobuilder
