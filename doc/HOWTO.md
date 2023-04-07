# Developer information

This document contains information about common development tasks.

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

## Building a CodeQL database for a Ruby program

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

## Running qltests

Run

```bash
codeql test run <test-path> --search-path <extractor-pack-path>
```

## Current Error
Error occurs when running query while selecting the newly created solidity database.

### Reproduce step
Using VSCode extension, select the newly create solidity database and run a dummy query on it

Clone [VSCode CodeQL Starter](https://github.com/github/vscode-codeql-starter) project
Create symbolic link to Solidity Extractor/Library directory

```
cd /mnt/c/Users/hdthinh1012/codeql-home/vscode-codeql-starter/ql
ln -s /mnt/c/Users/hdthinh1012/LVTN_CodeQL_Extractor/Solidity_CodeQL_Library_Projects solidity
```

Create an directory named `codeql-custom-queries-solidity` add a simple ql `example.ql` that not import solidity related QL library yet:
```
int getANumber() {
    result = 0
    or
    result <= 100 and result = getANumber() + 1
}

select getANumber()
```
Right click `example.ql` in editor, click 'CodeQL: Run Query on Selected Database'. Getting error with logs in CodeQL Query Server:
```
Error running query: Internal error. (codeQL.runQueryContextEditor)
Error: Error running query: Internal error.
    at handleResponse (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4399:40)
    at processMessageQueue (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4222:13)
    at Immediate.<anonymous> (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4208:11)
    at processImmediate (node:internal/timers:466:21)
-----------------------------------------------------------------------------------------------------------------------------------------------------------
| Log being saved to c:\Users\hdthinh1012\AppData\Roaming\Code\User\globalStorage\github.vscode-codeql\queries\example.ql-rBaPhXqV0jrW404f_rYYm\query.log |
-----------------------------------------------------------------------------------------------------------------------------------------------------------
[2023-04-07 23:38:12] [SPAMMY] execute query-server2> runQuery called with c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql
[2023-04-07 23:38:12] Calling plumbing command: codeql resolve library-path --additional-packs=c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-cpp;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-csharp;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-go;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-java;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-javascript;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-python;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-ruby;c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\ql --query=c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql --format=json
[2023-04-07 23:38:12] [DETAILS] resolve library-path> Found no pack; trying after symlink resolution with C:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql.
[2023-04-07 23:38:12] [DETAILS] resolve library-path> Resolving query at normalized path C:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql.
[2023-04-07 23:38:12] [DETAILS] resolve library-path> Found enclosing pack 'codeql-custom-queries-solidity' at C:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity.
[2023-04-07 23:38:12] [DETAILS] resolve library-path> Adding compilation cache at C:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\.cache.
[2023-04-07 23:38:12] [DETAILS] resolve library-path> Found no dbscheme through dependencies.
[2023-04-07 23:38:12] [DETAILS] resolve library-path> QL pack dependencies for C:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity found in cache.
[2023-04-07 23:38:12] Plumbing command codeql resolve library-path completed:
                      {
                        "libraryPath" : [
                          "C:\\Users\\hdthinh1012\\codeql-home\\vscode-codeql-starter\\codeql-custom-queries-solidity"
                        ],
                        "compilationCache" : [
                          "C:\\Users\\hdthinh1012\\codeql-home\\vscode-codeql-starter\\codeql-custom-queries-solidity\\.cache"
                        ],
                        "relativeName" : "codeql-custom-queries-solidity\\example.ql",
                        "possibleAdvice" : "There should probably be a qlpack.yml file declaring dependencies in C:\\Users\\hdthinh1012\\codeql-home\\vscode-codeql-starter\\codeql-custom-queries-solidity or an enclosing directory.",
                        "qlPackName" : "codeql-custom-queries-solidity"
                      }
[2023-04-07 23:38:12] [PROGRESS] execute query-server2> Compiling query plan for c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql.
[2023-04-07 23:38:12] [DETAILS] execute query-server2> Resolving imports for c:\Users\hdthinh1012\codeql-home\vscode-codeql-starter\codeql-custom-queries-solidity\example.ql.
[2023-04-07 23:38:12] [PROGRESS] execute query-server2> Error running query
[2023-04-07 23:38:12] [ERROR] Error running query
                              com.semmle.util.exception.ResourceError: Failed to read path C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
                              (eventual cause: NoSuchFileException "C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-so...")
                              	at com.semmle.util.io.WholeIO.strictread(WholeIO.java:341)
                              	at com.semmle.frontend.imports.QLFileSetResolver.addDbscheme(QLFileSetResolver.java:164)
                              	at com.semmle.frontend.imports.QLFileSetResolver.resolveFileSet(QLFileSetResolver.java:108)
                              	at com.semmle.frontend.imports.QLFileSetResolver.resolveFileSet(QLFileSetResolver.java:71)
                              	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.compileInner(CompilationCommon.java:312)
                              	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.compile(CompilationCommon.java:263)
                              	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.access$000(CompilationCommon.java:236)
                              	at com.semmle.cli2.ql.CompilationCommon.compile(CompilationCommon.java:233)
                              	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.compileAndPrepareQuery(QueryServer2Command.java:408)
                              	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$runQuery$26(QueryServer2Command.java:465)
                              	at com.semmle.util.concurrent.FutureUtils.supplyCompose(FutureUtils.java:217)
                              	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$runQuery$28(QueryServer2Command.java:457)
                              	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$withProgressAndCancellation$15(QueryServer2Command.java:346)
                              	at com.semmle.util.concurrent.FutureUtils.supplyCompose(FutureUtils.java:217)
                              	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$withProgressAndCancellation$16(QueryServer2Command.java:346)
                              	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(Unknown Source)
                              	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(Unknown Source)
                              	at java.base/java.lang.Thread.run(Unknown Source)
                              Caused by: java.nio.file.NoSuchFileException: C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
                              	at java.base/sun.nio.fs.WindowsException.translateToIOException(Unknown Source)
                              	at java.base/sun.nio.fs.WindowsException.rethrowAsIOException(Unknown Source)
                              	at java.base/sun.nio.fs.WindowsException.rethrowAsIOException(Unknown Source)
                              	at java.base/sun.nio.fs.WindowsFileSystemProvider.newByteChannel(Unknown Source)
                              	at java.base/java.nio.file.Files.newByteChannel(Unknown Source)
                              	at java.base/java.nio.file.Files.newByteChannel(Unknown Source)
                              	at java.base/java.nio.file.spi.FileSystemProvider.newInputStream(Unknown Source)
                              	at java.base/java.nio.file.Files.newInputStream(Unknown Source)
                              	at com.semmle.util.io.WholeIO.read(WholeIO.java:288)
                              	at com.semmle.util.io.WholeIO.strictread(WholeIO.java:339)
                              	... 17 common frames omitted
Apr 07, 2023 11:38:12 PM org.eclipse.lsp4j.jsonrpc.RemoteEndpoint fallbackResponseError
SEVERE: Internal error: Failed to read path C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
(eventual cause: NoSuchFileException "C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-so...")
java.util.concurrent.CompletionException: Failed to read path C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
(eventual cause: NoSuchFileException "C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-so...")
	at java.base/java.util.concurrent.CompletableFuture.encodeThrowable(Unknown Source)
	at java.base/java.util.concurrent.CompletableFuture.completeThrowable(Unknown Source)
	at java.base/java.util.concurrent.CompletableFuture.uniExceptionally(Unknown Source)
	at java.base/java.util.concurrent.CompletableFuture.uniExceptionallyStage(Unknown Source)
	at java.base/java.util.concurrent.CompletableFuture.exceptionally(Unknown Source)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$runQuery$28(QueryServer2Command.java:503)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$withProgressAndCancellation$15(QueryServer2Command.java:346)
	at com.semmle.util.concurrent.FutureUtils.supplyCompose(FutureUtils.java:217)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$withProgressAndCancellation$16(QueryServer2Command.java:346)
	at java.base/java.util.concurrent.ThreadPoolExecutor.runWorker(Unknown Source)
	at java.base/java.util.concurrent.ThreadPoolExecutor$Worker.run(Unknown Source)
	at java.base/java.lang.Thread.run(Unknown Source)
Caused by: Failed to read path C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
(eventual cause: NoSuchFileException "C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-so...")
	at com.semmle.util.io.WholeIO.strictread(WholeIO.java:341)
	at com.semmle.frontend.imports.QLFileSetResolver.addDbscheme(QLFileSetResolver.java:164)
	at com.semmle.frontend.imports.QLFileSetResolver.resolveFileSet(QLFileSetResolver.java:108)
	at com.semmle.frontend.imports.QLFileSetResolver.resolveFileSet(QLFileSetResolver.java:71)
	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.compileInner(CompilationCommon.java:312)
	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.compile(CompilationCommon.java:263)
	at com.semmle.cli2.ql.CompilationCommon$OneCompilation.access$000(CompilationCommon.java:236)
	at com.semmle.cli2.ql.CompilationCommon.compile(CompilationCommon.java:233)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.compileAndPrepareQuery(QueryServer2Command.java:408)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$runQuery$26(QueryServer2Command.java:465)
	at com.semmle.util.concurrent.FutureUtils.supplyCompose(FutureUtils.java:217)
	at com.semmle.cli2.execute.queryServer2.QueryServer2Command$Impl.lambda$runQuery$28(QueryServer2Command.java:457)
	... 6 more
Caused by: java.nio.file.NoSuchFileException: C:\Users\hdthinh1012\LVTN_CodeQL_Database\solidity\database\dummy-solidity\db-solidity\solidity.dbscheme.stats
	at java.base/sun.nio.fs.WindowsException.translateToIOException(Unknown Source)
	at java.base/sun.nio.fs.WindowsException.rethrowAsIOException(Unknown Source)
	at java.base/sun.nio.fs.WindowsException.rethrowAsIOException(Unknown Source)
	at java.base/sun.nio.fs.WindowsFileSystemProvider.newByteChannel(Unknown Source)
	at java.base/java.nio.file.Files.newByteChannel(Unknown Source)
	at java.base/java.nio.file.Files.newByteChannel(Unknown Source)
	at java.base/java.nio.file.spi.FileSystemProvider.newInputStream(Unknown Source)
	at java.base/java.nio.file.Files.newInputStream(Unknown Source)
	at com.semmle.util.io.WholeIO.read(WholeIO.java:288)
	at com.semmle.util.io.WholeIO.strictread(WholeIO.java:339)
	... 17 more

Error running query: Internal error. (codeQL.runQueryContextEditor)
Error: Error running query: Internal error.
    at handleResponse (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4399:40)
    at processMessageQueue (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4222:13)
    at Immediate.<anonymous> (c:\Users\hdthinh1012\.vscode\extensions\github.vscode-codeql-1.8.1\out\extension.js:4208:11)
    at processImmediate (node:internal/timers:466:21)
```

It seems that `solidity.dbscheme.stats` is lacking from the directory `db-solidity`
