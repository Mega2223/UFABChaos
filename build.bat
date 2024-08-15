@echo off

DEL build /Q
DEL bin /Q
MKDIR build
MKDIR bin
SET DIR=%cd%

FOR %%f IN (./src/*.rs) DO (
	REM echo compiling %%f
	REM rustc ./src/%%f
)

FOR %%f IN (./src/*.java) DO (
	echo compiling %%f
	javac -d bin ./src/%%f
	cd DIR/bin
	jar --create --main-class VecProd --file %%f.jar *
)
