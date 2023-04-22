cd extractor
cargo build --release
cd ..

extractor\target\release\codeql-extractor-solidity generate --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll

codeql query format -i ql\lib\codeql/solidity\ast\internal\TreeSitter.qll

rm -Recurse -Force extractor-pack
mkdir extractor-pack | Out-Null
cp codeql-extractor.yml, ql\lib\solidity.dbscheme, ql\lib\solidity.dbscheme.stats extractor-pack
cp -Recurse tools extractor-pack
cp -Recurse downgrades extractor-pack
mkdir extractor-pack\tools\win64 | Out-Null
cp extractor\target\release\codeql-extractor-solidity.exe extractor-pack\tools\win64\extractor.exe
