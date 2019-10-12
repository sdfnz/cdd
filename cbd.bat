@echo off
CALL cdd.exe %*
IF EXIST "setDir.bat" (
	CALL setDir.bat
	del setDir.bat
) ELSE (
	SET CD_PATH="."
)
cd %CD_PATH%
