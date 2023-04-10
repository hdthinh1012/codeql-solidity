@echo off

type NUL && "%CODEQL_DIST%\codeql.exe" database index-files ^
    --prune=**/*.testproj ^
    --include-extension=.sol ^
    --include=**/Gemfile ^
    --size-limit=5m ^
    --language=ruby ^
    "%CODEQL_EXTRACTOR_SOLIDITY_WIP_DATABASE%"

exit /b %ERRORLEVEL%
