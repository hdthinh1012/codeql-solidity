# Developer information

This document contains information about common development tasks.

## Note
Due to some Linking issues of native C code when compiling extractor (Rust sources) in Windows, the projects is only confirmed to compile successfully in WSL2.  

Who installed codeql CLI in Windows environment must open 2 VSCode instances, one in WSL2 to be able to run `codeql database create` command using symbolic-linked solidity extractor, one in Windows to run Solidity QL queries (higher performances). 

# (Linux/WSL2) Building the extractor tools from source

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

# (Windows) Adding core libraries QLL files

Using vscode-codeql-starter, running queries that import module 'solidity' will be errored as cannot resolve module 'solidity' and the CodeQL extension recommend to use qlpack.cmd both to publish packs contain core libraries for solidity (like `hdthinh1012/solidity-all`) and import to the packs contain the current user queries.

## (Windows) Trying to make the solidity-queries/solidity-database/solidity-libraries-module working with the vscode-codeql-starter workspace

Using [codeql-packs](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-packs) combined with [codeql-workspace.yml](https://docs.github.com/en/code-security/codeql-cli/codeql-cli-reference/about-codeql-workspaces#the-codeql-workspaceyml-file) to achieve the ultimate goals of writing queries anywhere importing QL libraries from everywhere.

### (Windows) Create symbolic link from codeql cli directory to ./extractor-path

### (Windows) Creating some qlpack.yml and codeql-workspaces.yml to detect queries from codeql-custom-queries-solidity and ql/solidity (symbolic linked to Solidity_CodeQL_Library_Projects)

### (Windows) Copy ql/shared folder containing 4 folders of language-diagnostic QL core packs: codeql/ssa, codeql/regex, codeql/util, codeql/tutorial to ql/solidity for packs "hdthinh1012/solidity-all"  

### (Windows) Adding repository 'hdthinh1012/solidity-codeql-libraries-project' as submodule of 'hdthinh1012/codeql'

# Current Issues
## AstNode.getLocation() override
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
Currently, the solution is temporarily comment all the derived method. 

Now running the queries is going for a longer period, but now gets to the annoying issue that force me to try to compile the extractor in Windows, which somehow caught with LNK error while the Linux version ran perfertly !?

## Skip database downgrade because cannot find the solidity extractor

The log skip some error
```
[2023-04-12 17:51:00] Plumbing command codeql resolve languages completed:
                      {
                        "extractors" : {
                          "cpp" : [
                            {
                              "extractor_root" : "C:\\Users\\hdthinh1012\\codeql-home\\codeql\\codeql\\cpp"
                            }
                          ],
                          "csharp" : [
                            {
                              "extractor_root" : "C:\\Users\\hdthinh1012\\codeql-home\\codeql\\codeql\\csharp",
                            }
                          ],
                          ... java, javascript, go, etc.
                          "xml" : [
                            {
                              "extractor_root" : "C:\\Users\\hdthinh1012\\codeql-home\\codeql\\codeql\\xml"
                            }
                          ]
                        }
                      }
[2023-04-12 17:51:00] [PROGRESS] execute query-server2> Skip database downgrade because I cannot find the solidity extractor
```
So the next step is to compiler Rust source in Windows (cannot avoid it now :)) and copy `extractor-pack` folder to location of CodeQL Extractor CLI folder (usually is `<your-home>/codeql-home/codeql/codeql`)

# Issues related to compiling extractor in Windows
All three option for Rust crate 'cc' compilers all get errors:
- AR: lib, CC: clang (clang-cl), CXX: clang++ (cl). This get wierd error of not reference fdopen
- AR: ar, CC: gcc, CXX: g++. These options currently stuck at changing argument to fit the ar command
- AR: lib, CC: cl, CXX: cl
The commit [6e0c49305e003d69cb6418a0cb8a9b1744035afe](https://github.com/tree-sitter/tree-sitter/pull/1411/commits) states that 'fdopen' is deprecate for MSVC clang compiler, now changed to __fdopen instead. 
```
src/./parser.c:1781:28: warning: 'fdopen' is deprecated: The POSIX name for this item is deprecated. Instead, use the ISO C and C++ conformant name: _fdopen. See online help for details. [-Wdeprecated-declarations]
```
The commit seemed not included in even the latest release version of tree-sitter (0.20.8) for some reason. Must cloned the latest version, use special git command to add the include the fix commit (git cherry-pick) and release myself some cargo 'tree-sitter-unofficial' for example.

## Cloned, publish custom tree-sitter unofficial packages to fix compile errors when using MSVC Clang

### tree-sitter-unofficial
1. Cloned tree-sitter (all branch)
2. Merge all these commit from these 2 opening pq from this repository https://github.com/aminya/tree-sitter 
  - https://github.com/tree-sitter/tree-sitter/pull/1411 branch c-warning
  - https://github.com/tree-sitter/tree-sitter/pull/1835 branch clang
3. Publish crate tree-sitter-unofficial

### tree-sitter-solidity-unofficial
1. Update Cargo.toml, include the 'tree-sitter-unofficial' as dependency.

### Results
Now compile in Windows succeed

## Test the newly create extractor in Windows

## Running qltests

Run

```bash
codeql test run <test-path> --search-path <extractor-pack-path>
```
