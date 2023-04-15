# Solidity analysis support for CodeQL

This directory contains the extractor, CodeQL libraries, and queries that power Ruby
support in [LGTM](https://lgtm.com) and the other CodeQL products that [GitHub](https://github.com)
makes available to its customers worldwide.

It contains two major components:
  1. static analysis libraries and queries written in
     [CodeQL](https://codeql.github.com/docs/) that can be used to analyze such
     a database to find coding mistakes or security vulnerabilities.
  2. an extractor, written in Rust (by the Github CodeQL Teams, not me), that parses Solidity source code and converts it
     into a database that can be queried using CodeQL.

# Documentation step for this project
See [document](doc/HOWTO.md) to follow the step of trying to achieve the ability of querying QL that import solidity modules (not success yet).

# Projects dependencies
[JoranHonig's tree-sitter-solidity](https://github.com/JoranHonig/tree-sitter-solidity)
