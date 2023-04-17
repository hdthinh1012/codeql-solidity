cargo build --release

cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
codeql query format -i ql\lib\codeql/solidity\ast\internal\TreeSitter.qll

Remove-Item -Recurse -Force extractor-pack
mkdir extractor-pack | Out-Null
Copy-Item codeql-extractor.yml, ql\lib\solidity.dbscheme, ql\lib\solidity.dbscheme.stats extractor-pack #  ql\lib\solidity.dbscheme.stats will be copy from a solidity database after running codeql database measure
Copy-Item -Recurse tools extractor-pack
# Copy-Item -Recurse downgrades extractor-pack
mkdir extractor-pack\tools\win64 | Out-Null
Copy-Item target\release\solidity-extractor.exe extractor-pack\tools\win64\extractor.exe
Copy-Item target\release\solidity-autobuilder.exe extractor-pack\tools\win64\autobuilder.exe
