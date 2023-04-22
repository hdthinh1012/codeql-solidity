# CodeQL Solidity

This dummy project, not yet achieve its goal, contains 2 components:
  1. static analysis libraries and queries written in
     [CodeQL](https://codeql.github.com/docs/) that can be used to analyze such
     a database to find coding mistakes or security vulnerabilities.
  2. an extractor, written in Rust (by the Github CodeQL team), that parses Solidity source code and converts it
     into a database that can be queried using CodeQL.

The project dashboard [here](https://github.com/users/hdthinh1012/projects/1)

# Projects dependencies
## Dependencies
- [tree-sitter](https://github.com/hdthinh1012/tree-sitter)
- [tree-sitter-solidity](https://github.com/hdthinh1012/tree-sitter-solidity)


## Repositories used in this current project
- [hdthinh1012/codeql-solidity](https://github.com/hdthinh1012/codeql-solidity) `main` branch: using codeql-extractor-ruby & codeql-extractor rust source code, use to develop Solidity core libraries.
- [hdthinh1012/codeql](https://github.com/hdthinh1012/codeql) `dev` branch, QL library repository
- [hdthinh1012/vscode-codeql-starter](https://github.com/hdthinh1012/vscode-codeql-starter) `dev` branch, 'hdthinh1012/codeql' (after checkout to branch `dev`) will be replacement of the default folder 'ql' 

When contribute, your branch name will be `dev-<github-username>` and pull request is merge into the branch stated above.

# Setup CodeQL repo
Clone [hdthinh1012/codeql](https://github.com/hdthinh1012/codeql), checkout  `dev` branch.  This branch has add some path to ignore in `.gitignore`
```
# Ignore the my codeql-solidity project
solidity/**
solidity

# Ignore my custom tree-sitter folder
shared/tree-sitter/**
shared/tree-sitter
```
Create folder `shared/tree-sitter`, cd into it, clone [hdthinh1012/tree-sitter-solidity](https://github.com/hdthinh1012/tree-sitter-solidity), cd into 'tree-sitter-solidity', checkout branch 'fix-msvc'.

Change directory back to `shared/tree-sitter`, clone clone [hdthinh1012/tree-sitter](https://github.com/hdthinh1012/tree-sitter), cd into 'tree-sitter' folder, checkout branch 'fix-msvc'.

Change directory back to 'codeql-repo' root clone [hdthinh1012/codeql-solidity](https://github.com/hdthinh1012/codeql-solidity) :
```
git clone https://github.com/hdthinh1012/codeql-solidity solidity
```
The extractor code is in `solidity/extractor` folder, its Cargo.toml has specified the local dependencies path for `tree-sitter`, `tree-sitter-solidity`, `codeql-extractor` correctly, now you can build the project with Cargo command in the next step 

# Build Solidity Extractor & generate solidity.dbscheme, TreeSitter.qll
Clone [hdthinh1012/codeql-solidity](https://github.com/hdthinh1012/codeql-solidity), follow command using `cargo` to build for WSL2 or Windows below.

[Install Rust](https://www.rust-lang.org/tools/install), then run every command list in `scripts/create-extractor-pack.ps1` or `scripts/create-extractor-pack.sh` (depending on your OS) to build the extractor, generate necessary file to implement QL Libraries.

# Add Solidity extractor to CodeQL CLI
Install [CodeQL CLI](https://docs.github.com/en/code-security/codeql-cli/using-the-codeql-cli/getting-started-with-the-codeql-cli#1-download-the-codeql-cli-zip-package) locally on your machine if you haven't already. The directory path containing your CLI if you follow the CodeQL CLI install guide strictly tends to be in `<user-home>/codeql-home/codeql/codeql` (distinguish with repository `github/codeql` which is QL libraries, not the CLI).

Copy the `extractor-pack` directory you just build in the above step into `<user-home>/codeql-home/codeql/codeql`, rename it to `solidity`. Check if CLI detects solidity extractor with `codeql resolve languages`


# Create Solidity Database
Using remix-ide [sample code](https://remix.ethereum.org/)
```
ls /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/source/dummy-solidity/

Ballot.sol  Owner.sol  Storage.sol
```
Run command `codeql database create -l <language> -s <source-path> <database-path>`
```
codeql database create /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/database/dummy-solidity -l solidity -s /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/source/dummy-solidity

ls -la /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Database/solidity/database/dummy-solidity/

-rwxrwxrwx 1 hdthinh1012 hdthinh1012   16 Apr  7 22:51 baseline-info.json
-rwxrwxrwx 1 hdthinh1012 hdthinh1012  288 Apr  7 22:51 codeql-database.yml
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 db-solidity
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 diagnostic
drwxrwxrwx 1 hdthinh1012 hdthinh1012  512 Apr  7 22:51 log
-rwxrwxrwx 1 hdthinh1012 hdthinh1012 3243 Apr  7 22:51 src.zip
```

## Creating solidity.dbscheme.stats required for QL queries to run successfully on the solidity databases

Run command [`codeql dataset measure`](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-manual/dataset-measure) after generating raw QL dataset directory in `<database-directory>/db-solidity`, a solidity.dbscheme.stats will be generated, which is necessary for query step.
 
After that running any queries on the database but without importing `solidity` module yet will be success.



# Current Issues

## (Issue 1) Fixing AstNode.getLocation() override
Some class declared in TreeSitter.qll extends from AstNode, @solidity_<equivalent_node_name>:
```ql
  /** A class representing `parameter` nodes. */
  class Parameter extends @solidity_parameter, AstNode {
    ...
    /** Gets the location of the node */
    final AstNode getLocation() { solidity_parameter_location(this, result) } <- Here
    ...
  }

```

The original of AstNode
```ql
  class AstNode extends @solidity_ast_node {
    ...
    /** Gets the location of this element. */
    final L::Location getLocation() { solidity_ast_node_info(this, _, _, result) } <- Here
    ...
  }
```
The derived class currently warned by CodeQL Language Server for 2 errors:
- Lacking override keyword
- Method signature not compatible  

Currently, the solution is temporarily comment all the derived method. Though this change should be discarded and not commited.




## (Issue 2) Upgrade or Downgrade related to version diff between database schema
During setup step of QL evaluation on selected database (right click menu in VSCode extenstion), the tools run a command `codeql resolve upgrades `

Running example.ql that has `import solidity` in the first line, get the error:
```
Failed to run query: There was no upgrade path to the target dbscheme
```
Last lines in CodeQL Query Server indicates another command that belongs in the Initialize phase [`codeql resolve upgrades`](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-manual/resolve-upgrades)
```
[2023-04-14 18:44:45] Calling plumbing command: codeql resolve upgrades --search-path=C:\Users\hdthinh1012\codeql-home\codeql\codeql\solidity --target-sha=7f550b2cfadd90b7a93f075b4d7a53a429af37a2 --allow-downgrades --dbscheme=c:\Users\hdthinh1012\LVTN\CodeQL-Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme --format=json
[2023-04-14 18:44:45] [DETAILS] resolve upgrades> Searching for upgrades that apply to aa8857de371ebc4f03226075d6c26376604300cb.
[2023-04-14 18:44:45] [DETAILS] resolve upgrades> Will look for upgrades in C:\Users\hdthinh1012\codeql-home\codeql\codeql\legacy-upgrades\upgrades
[2023-04-14 18:44:45] [DETAILS] resolve upgrades> Found upgrade run ending at aa8857de371ebc4f03226075d6c26376604300cb.
[2023-04-14 18:44:45] [DETAILS] resolve upgrades> Searching for downgrades that apply to aa8857de371ebc4f03226075d6c26376604300cb.
[2023-04-14 18:44:45] [DETAILS] resolve upgrades> Found downgrade run ending at aa8857de371ebc4f03226075d6c26376604300cb
```

# Additional Note
Note for myself only
## Windows Issues
Note: Official tree-sitter project repository has not merged the pull request that fixing compiling issue for MSVC, so I have to clone and publish custom cargo create 'tree-sitter-unofficial' and 'tree-sitter-solidity-unofficial'  

### Creating custom crate tree-sitter-unofficial and tree-sitter-solidity-unofficial 

The commit [6e0c49305e003d69cb6418a0cb8a9b1744035afe](https://github.com/tree-sitter/tree-sitter/pull/1411/commits) states that 'fdopen' is deprecate for MSVC clang compiler, now changed to __fdopen instead. 
```
src/./parser.c:1781:28: warning: 'fdopen' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fdopen. See online help for details. [-Wdeprecated-declarations]
```
The commit seemed not included in even the latest release version of tree-sitter (0.20.8) for some reason, which cause the build command `cargo build --release` on this repository to fail on Windows when compiling with clang-cl.  

So I haved forked `tree-sitter/tree-sitter` and `JoranHorig/tree-sitter-solidity`, here are the step I have done: 

- tree-sitter
  1. Fork tree-sitter/tree-sitter
  2. Create branch fix-msvc
  3. Merge all these commit from these 2 opening pq from this repository https://github.com/aminya/tree-sitter 
      - https://github.com/tree-sitter/tree-sitter/pull/1411 branch c-warning
      - https://github.com/tree-sitter/tree-sitter/pull/1835 branch clang
  4. Push to remote

- tree-sitter-solidity
  1. Forked JoranHorig/tree-sitter-solidity
  1. Update Cargo.toml, include local downloaded `tree-sitter` (`lib` folder)  as dependency.
  2. Push to remote


Now compiling in Windows will succeed and able to extract solidity source code into solidity database. 

# Dependencies
This project is based on the following projects:
- [github/codeql](https://github.com/github/codeql)
- [github/vscode-codeql-starter](https://github.com/github/vscode-codeql-starter)
- [tree-sitter/tree-sitter](https://github.com/tree-sitter/tree-sitter)
- [github/tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity)