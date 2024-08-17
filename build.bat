@echo off

DEL build /Q
MKDIR build

SET DIR=%cd%

FOR %%f IN (./src/*.rs) DO (
	cd ./build
	echo compiling %%f
	rustc "%DIR%\src\%%f"
	cd ..
)

FOR %%f IN (./build/*.pdb) DO ( DEL build\%%f /Q )

FOR %%f IN (./src/*.java) DO (
	echo compiling %%f
	javac -d build ./src/%%f
	cd ./build
	jar --create --main-class VecProd --file %%f.jar *
	cd ..
)

FOR %%f IN (./build/*.class) DO ( DEL build\%%f /Q )

