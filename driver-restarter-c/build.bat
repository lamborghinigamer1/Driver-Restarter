if not exist "bin" (
    mkdir bin
)

if exist bin\device-restarter-c.exe (
    del bin\device-restarter-c.exe
)

gcc main.c -o bin\device-restarter-c.exe