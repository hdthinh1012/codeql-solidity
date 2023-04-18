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
[tree-sitter](https://github.com/hdthinh1012/tree-sitter): cloned  
[tree-sitter-solidity](https://github.com/hdthinh1012/tree-sitter-solidity): cloned  

## Forked 
Github codeql libraries  
Github tree-sitter

## Repositories used in this current project
[hdthinh1012/solidity-codeql-library-projects](https://github.com/hdthinh1012/solidity-codeql-library-project) `main` branch: using Ruby extractor source code, this repository, use to develop Solidity core libraries.
[hdthinh1012/vscode-codeql-starter](https://github.com/hdthinh1012/vscode-codeql-starter) `dev` branch  
[hdthinh1012/codeql](https://github.com/hdthinh1012/codeql) `dev-solidity` branch

When contribute, your branch name will be `dev-<github username>` and pull request is merge into the branch state above.

# Build Solidity Extractor
Clone [hdthinh1012/solidity-codeql-library-projects](https://github.com/hdthinh1012/solidity-codeql-library-project), checkout to your branch named `dev-username`, follow command using `cargo` to build for WSL2 or Windows below.

## WSL2 
[Install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
cargo clean
cargo build --release
```

### Generating the database schema and QL library

The generated `ql/lib/solidity.dbscheme` and `ql/lib/codeql/solidity/ast/internal/TreeSitter.qll` files are included in the repository, but they can be re-generated as follows:

```bash
# Run the generator
cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
# Then auto-format the QL library
codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
```
### Building a CodeQL database for a Solidity program

Run `scripts/create-extractor-pack.sh` (Linux/Mac) and the pack will be created in the `extractor-pack` directory.


## Windows
 So clone this repo, [Install Rust](https://www.rust-lang.org/tools/install), then run:

```bash
cargo clean
cargo build --release
```
### Generating the database schema and QL library

The generated `ql/lib/solidity.dbscheme` and `ql/lib/codeql/solidity/ast/internal/TreeSitter.qll` files are included in the repository, but they can be re-generated as follows:

```bash
# Run the generator
cargo run --release -p solidity-generator -- --dbscheme ql/lib/solidity.dbscheme --library ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
# Then auto-format the QL library
codeql query format -i ql/lib/codeql/solidity/ast/internal/TreeSitter.qll
```

### Building a CodeQL database for a Solidity program

Run `scripts\create-extractor-pack.ps1` (Windows PowerShell) and the pack will be created in the `extractor-pack` directory.

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

# Running queries with VSCode CodeQL Starter

Clone the forked [hdthinh1012/vscode-codeql-starter](https://github.com/hdthinh1012/vscode-codeql-starter), checkout to `dev` branch (your branch must be named `dev-<username>`).

## Setup new 'QL' directory containing solidity libraries into vscode-codeql-starter

Running queries in [vscode-codeql-starter](https://github.com/hdthinh1012/vscode-codeql-starter) that import module 'solidity' will be errored as cannot resolve module 'solidity'. Steps to fix the issue:
- The `ql` (or `codeql-repo`) folder that opened in the vscode-codeql-starter workspace (`codeql-repo` cloned from `hdthinh1012/codeql` libraries repository) must include submodule `solidity` (which is a submodule cloned from `hdthinh1012/solidity-codeql-libraries-project`)
- Must use [codeql-packs](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-packs) combined with [codeql-workspace.yml](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-workspaces#the-codeql-workspaceyml-file) to declared to the CodeQL workspaces about qlpack.cmd locations.

### Using repository [hdthinh1012/codeql](https://github.com/hdthinh1012/codeql) as `ql` module in vscode-codeql-starter instead of github/codeql

Third, clone with `recurse-submodule` option the [hdthinh1012/codeql](https://github.com/hdthinh1012/codeql) and renamed to `codeql-repo` with the command:
```bash
git clone --recurse-submodule https://github.com/hdthinh1012/codeql codeql-repo
```
Checkout to `dev-solidity` which is main branch of this repo.

Then remove the default submodule `ql` folder out of vscode-codeql-starter workspace (just right-click in the GUI and click 'Remove folder from Workspace') and add the folder cloned from [hdthinh1012/codeql](https://github.com/hdthinh1012/codeql) (switch to branch `dev-solidity`) as the libraries directory, named it as codeql-repo. The `codeql-workspace.yml` found in `ql` directory has declared all qlpack.cmd for VSCode extension to import pack locally:
```
provide:
  - "*/ql/src/qlpack.yml"
  - "*/ql/lib/qlpack.yml"
  - "*/ql/test/qlpack.yml"
  - "*/ql/examples/qlpack.yml"
  - "*/ql/consistency-queries/qlpack.yml"
  - "shared/*/qlpack.yml"
   ...
  - "ql/extractor-pack/codeql-extractor.yml"
   ...
```

The qlpack.yml that declared `hdthinh1012/solidity-all` packs is in `<codeql-repo>/solidity/ql/lib/`
```
name: hdthinh1012/solidity-all
version: 0.0.1
dbscheme: solidity.dbscheme
library: true
dependencies:
  codeql/regex: ${workspace} # This mean the codeql-workspace.yml found in current VSCode workspace will handle this for me
  codeql/ssa: ${workspace}
  codeql/tutorial: ${workspace}
  codeql/util: ${workspace}

```

### Adding codeql-custom-queries-solidity
In the hdthinh1012/vscode-codeql-starter, when checking out to dev branch, a `codeql-custom-queries-solidity` directory will appear, please add this into current open VSCode workspace.

There is a simple file `example.ql` that contains simple query that just query the base AstNode from TreeSitter.qll.

```
import codeql.solidity.ast.internal.TreeSitter

from Solidity::AstNode node
select node, "This is low-level Solidity::AstNode."
```


We will try to add more and more related part with solidity:

- Nothing related to solidity: pass
- [Just `import solidity` to get the libraries and trigger checks of `solidity.dbscheme`](https://github.com/users/hdthinh1012/projects/1?pane=issue&itemId=25746878): failed, stuck at some upgrade/downgrade feature related to CLI/dbscheme version discrepancies between the queried database vs CodeQL Cli (the solidity extractor-pack that we copy-pasted early on)
- [Queries tree-sitter wrapper](https://github.com/users/hdthinh1012/projects/1?pane=issue&itemId=25746892): to do
- [Queries tree-sitter token](https://github.com/users/hdthinh1012/projects/1?pane=issue&itemId=25746899): to do
- [Queries tree-sitter AST](https://github.com/users/hdthinh1012/projects/1?pane=issue&itemId=25746906): to do 



Now you can include the solidity database and right-click, choose 'CodeQL: Run Query on Selected Database' option on the example.ql file.




# Contribution note
Currently, in `hdthinh1012/codeql` & `hdthinh1012/vscode-codeql-starter` & `hdthinh1012/solidity-codeql-library-projects`, just checkout branch with name `dev-<your github username>` is enough. If the project happens to grow into something useful, there will be a new guide.

# Tracking current project state
Records about current state of the `ql/lib` folder, where most of my work is actually about
- TreeSitter.qll, solidity.dbscheme: generate from solidity-generator
- codeql/files/FileSystem.qll: copy from Ruby
- codeql/Locations.qll: copy from Ruby

# Update flow for each 
For each commit in `hdthinh1012/solidity-codeql-library-projects`, you must do all the following steps to update the `solidity` extractor in `codeql-home/codeql/codeql`, the `codeql-repo`, the `codeql-repo` that opened in `vscode-codeql-starter` workspace.
- Commit code in this repository [solidity-codeql-library-projects](https://github.com/hdthinh1012/solidity-codeql-library-project)
- Run command `cargo clean && cargo build --release`
- Run script `scripts/create-extractor-pack.sh` (Linux) or `scripts/create-extractor-pack.ps1` (Windows) to create `extractor-pack` directory
- Push new commit in `hdthinh1012/solidity-codeql-library-projects` to remote, pull request to `main` branch.
- Copy the new `extractor-pack` directory into `<user-home>/codeql-home/codeql/codeql` (where the CodeQL CLI installed), rename to `solidity`, replace the old `solidity` extractor folder if exists.
- Change directory into `codeql-repo` folder (where the QL libraries installed), checkout the `dev-solidity` branch, (discard everychange you have in TreeSitter.qll), fetch the new commit in `solidity` directory with command `git submodule update --remote`
- Open vscode-codeql-starter workspace in VSCode, add the `codeql-repo` folder into workspace as library, remove `ql` directory out of workspace (not delete, since touching submodule folder breaks everything), test the example queries with solidity module.









# Current Issues

## (Issue 1) Fixing AstNode.getLocation() override
Some class declared in TreeSitter.qll extends from AstNode, @solidity_<equivalent_node_name>:
```ql
  /** A class representing `parameter` nodes. */
  class Parameter extends @solidity_parameter, AstNode {
    /** Gets the name of the primary QL class for this element. */
    final override string getAPrimaryQlClass() { result = "Parameter" }

    /** Gets the location of the node */
    final AstNode getLocation() { solidity_parameter_location(this, result) } <- Here
    
    /** Gets the node corresponding to the field `name`. */
    final Identifier getName() { solidity_parameter_name(this, result) }

    /** Gets the node corresponding to the field `type`. */
    final TypeName getType() { solidity_parameter_def(this, result) }

    /** Gets a field or child node of this node. */
    final override AstNode getAFieldOrChild() {
      solidity_parameter_location(this, result) or
      solidity_parameter_name(this, result) or
      solidity_parameter_def(this, result)
    }
  }

```

The original of AstNode
```ql
  class AstNode extends @solidity_ast_node {
    /** Gets a string representation of this element. */
    string toString() { result = this.getAPrimaryQlClass() }

    /** Gets the location of this element. */
    final L::Location getLocation() { solidity_ast_node_info(this, _, _, result) } <- Here

    /** Gets the parent of this element. */
    final AstNode getParent() { solidity_ast_node_info(this, result, _, _) }

    /** Gets the index of this node among the children of its parent. */
    final int getParentIndex() { solidity_ast_node_info(this, _, result, _) }

    /** Gets a field or child node of this node. */
    AstNode getAFieldOrChild() { none() }

    /** Gets the name of the primary QL class for this element. */
    string getAPrimaryQlClass() { result = "???" }

    /** Gets a comma-separated list of the names of the primary CodeQL classes to which this element belongs. */
    string getPrimaryQlClasses() { result = concat(this.getAPrimaryQlClass(), ",") }
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
The commit seemed not included in even the latest release version of tree-sitter (0.20.8) for some reason, which cause the build command `cargo build --release` on this repository to fail on Windows.  

So I haved cloned and republished crate `tree-sitter-unofficial` and `tree-sitter-solidity-unofficial`, here are the step I have done: 

- tree-sitter-unofficial
  1. Cloned tree-sitter (all branch)
  2. Merge all these commit from these 2 opening pq from this repository https://github.com/aminya/tree-sitter 
      - https://github.com/tree-sitter/tree-sitter/pull/1411 branch c-warning
      - https://github.com/tree-sitter/tree-sitter/pull/1835 branch clang
  3. Publish crate tree-sitter-unofficial

- tree-sitter-solidity-unofficial
  1. Update Cargo.toml, include the 'tree-sitter-unofficial' as dependency.
  2. Publish crate tree-sitter-solidity-unofficial


Now compiling in Windows will succeed and able to extract solidity source code into solidity database. 