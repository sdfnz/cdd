@echo off
CALL cdd.exe %*
CALL setDir.bat
del setDir.bat
cd %CD_PATH%
