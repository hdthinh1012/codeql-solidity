@echo off

type NUL && "%CODEQL_DIST%\codeql.exe" database index-files ^
    --prune=**/*.testproj ^
    --include-extension=.sol ^
    --size-limit=5m ^
    --language=solidity ^
    "%CODEQL_EXTRACTOR_SOLIDITY_WIP_DATABASE%"

exit /b %ERRORLEVEL%
