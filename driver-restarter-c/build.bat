if not exist "bin" (
    mkdir bin
)

if exist bin\driver-restarter-c.exe (
    del bin\driver-restarter-c.exe
)

gcc main.c -o bin\driver-restarter-c.exe