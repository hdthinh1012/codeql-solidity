@echo off

type NUL && "%CODEQL_EXTRACTOR_SOLIDITY_ROOT%\tools\%CODEQL_PLATFORM%\autobuilder.exe"

exit /b %ERRORLEVEL%
