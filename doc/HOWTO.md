# Developer information

This document contains information about common development tasks.

## Note
Due to some Linking issues of native C code when compiling extractor in Windows, the projects is only confirmed to compile successfully in WSL2.  

Who installed codeql CLI in Windows environment must open 2 VSCode instances, one in WSL2 to be able to run `codeql database create` command using symbolic-linked solidity extractor, one in Windows to run Solidity QL queries (higher performances). 

## Building the tools from source

[Install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
cargo build --release
```

## Generating the database schema and QL library

The generated `ql/lib/solidity.dbscheme` and `ql/lib/codeql/solidity/ast/internal/TreeSitter.qll` files are included in the repository, but they can be re-generated as follows:

```bash
# Run the generator
cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
# Then auto-format the QL library
codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
```

## Building a CodeQL database for a Solidity program

First, get an extractor pack:

Run `scripts/create-extractor-pack.sh` (Linux/Mac) or `scripts\create-extractor-pack.ps1` (Windows PowerShell) and the pack will be created in the `extractor-pack` directory.

Then run

```bash
codeql database create <database-path> -l solidity -s <project-source-path> --search-path <extractor-pack-path>
```

## Create symbolic link from codeql cli directory to ./extractor-path
```
cd ~/codeql-home/codeql/codeql
#or in case of WSL
cd /mnt/c/Users/<user-home>/codeql-home/codeql/codeql

ln -s <extractor-pack-path> solidity
```
Check resolve languages
```
codeql resolve languages

"""
cpp (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/cpp)
csharp (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/csharp)
csv (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/csv)
go (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/go)
html (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/html)
java (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/java)
javascript (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/javascript)
properties (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/properties)
python (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/python)
ruby (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/ruby)
solidity (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/solidity)
xml (/mnt/c/Users/hdthinh1012/codeql-home/codeql/codeql/xml)
"""
```

## Create databases examples

Using remix-ide sample code
```
ls /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/source/dummy-solidity/

"""
Ballot.sol  Owner.sol  Storage.sol
"""

codeql database create /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/database/dummy-solidity -l solidity -s /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/source/dummy-solidity

ls -la /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/database/dummy-solidity/

"""
total 8
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 .
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 ..
-rwxrwxrwx 1 hdthinh1012 hdthinh1012   16 Apr  7 22:51 baseline-info.json
-rwxrwxrwx 1 hdthinh1012 hdthinh1012  288 Apr  7 22:51 codeql-database.yml
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 db-solidity
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 diagnostic
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 log
-rwxrwxrwx 1 hdthinh1012 hdthinh1012 3243 Apr  7 22:51 src.zip
"""
```
### Fix lacking solidity.dbscheme.stats

Run command `codeql dataset measure` after generating raw QL dataset directory in <database-directory>/db-solidity. After that running any queries that without importing solidity module yet will be success.


## Current Issues

Using vscode-codeql-starter, running queries that import module 'solidity' will be errored as cannot resolve module 'solidity' and the CodeQL extension recommend to use qlpack.cmd both to publish packs contain core libraries for solidity (like `hdthinh1012/solidity-all`) and import to the packs contain the current user queries.

## Running qltests

Run

```bash
codeql test run <test-path> --search-path <extractor-pack-path>
```
