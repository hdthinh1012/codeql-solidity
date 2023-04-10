cargo build --release

cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
codeql query format -i ql\lib\codeql/solidity\ast\internal\TreeSitter.qll

rm -Recurse -Force extractor-pack
mkdir extractor-pack | Out-Null
cp codeql-extractor.yml, ql\lib\solidity.dbscheme, ql\lib\solidity.dbscheme.stats extractor-pack
cp -Recurse tools extractor-pack
cp -Recurse downgrades extractor-pack
mkdir extractor-pack\tools\win64 | Out-Null
cp target\release\solidity-extractor.exe extractor-pack\tools\win64\extractor.exe
cp target\release\solidity-autobuilder.exe extractor-pack\tools\win64\autobuilder.exe
