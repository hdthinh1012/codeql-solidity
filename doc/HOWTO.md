# Developer information

This document contains information about common development tasks.

## Note
Due to some Linking issues of native C code when compiling extractor (Rust sources) in Windows, the projects is only confirmed to compile successfully in WSL2.  

Who installed codeql CLI in Windows environment must open 2 VSCode instances, one in WSL2 to be able to run `codeql database create` command using symbolic-linked solidity extractor, one in Windows to run Solidity QL queries (higher performances). 

## (Linux/WSL2) Building the tools from source

[Install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
cargo build --release
```

## (Linux/WSL2) Generating the database schema and QL library

The generated `ql/lib/solidity.dbscheme` and `ql/lib/codeql/solidity/ast/internal/TreeSitter.qll` files are included in the repository, but they can be re-generated as follows:

```bash
# Run the generator
cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
# Then auto-format the QL library
codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
```

## (Linux/WSL2) Building a CodeQL database for a Solidity program

First, get an extractor pack:

Run `scripts/create-extractor-pack.sh` (Linux/Mac) or `scripts\create-extractor-pack.ps1` (Windows PowerShell) and the pack will be created in the `extractor-pack` directory.

Then run

```bash
codeql database create <database-path> -l solidity -s <project-source-path> --search-path <extractor-pack-path>
```

### (Windows) Create symbolic link from codeql cli directory to ./extractor-path to running the command `codeql database create -l solidity --source-root=<source-path> <database-path>` like all other supported languages.
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

## (Linux/WSL2) Create databases examples

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
### (Linux/WSL2) Creating solidity.dbscheme.stats required for any QL queries to run successfully on the solidity databases

Run command `codeql dataset measure` after generating raw QL dataset directory in <database-directory>/db-solidity. After that running any queries that without importing solidity module yet will be success.

## Adding core libraries QLL files

Using vscode-codeql-starter, running queries that import module 'solidity' will be errored as cannot resolve module 'solidity' and the CodeQL extension recommend to use qlpack.cmd both to publish packs contain core libraries for solidity (like `hdthinh1012/solidity-all`) and import to the packs contain the current user queries.

# Trying to make the solidity-queries/solidity-database/solidity-libraries-module working with the vscode-codeql-starter workspace

Using [codeql-packs](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-packs) combined with [codeql-workspace.yml](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-workspaces#the-codeql-workspaceyml-file) to achieve the ultimate goals of writing queries anywhere importing QL libraries from everywhere.

### (Windows) Create symbolic link from codeql cli directory to ./extractor-path

### Creating some qlpack.yml and codeql-workspaces.yml to detect queries from codeql-custom-queries-solidity and ql/solidity (symbolic linked to Solidity_CodeQL_Library_Projects)

### Copy ql/shared folder containing 4 folders of language-diagnostic QL core packs: codeql/ssa, codeql/regex, codeql/util, codeql/tutorial to ql/solidity for packs "hdthinh1012/solidity-all"  

# Current Problems
Adding 'solidity' the 'ql' aka codeql-repo (which contains core libraries pack for the languages) via symbolic-link in Windows, the codeql-workspace.yml will not able to detects the codeql/util, codeql/ssa, codeql/regex identified in 'solidity-all' qlpacks, it must somehow the real code, via raw copy-paste (which cause issue since not using version control).  

The solution is in 'codeql-repo' create 'dev-solidity' branch then adding the Solidity_CodeQL_Library_Projects as a git submodule, which satisfied both real-file condition for the codeql workspace resolvers and using version control to avoid headache later on.

## Running qltests

Run

```bash
codeql test run <test-path> --search-path <extractor-pack-path>
```
