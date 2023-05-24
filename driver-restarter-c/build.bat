if not exist "bin" (
    mkdir bin
)

if exist bin\device-restarter.exe (
    del bin\device-restarter.exe
)

gcc main.c -o bin\device-restarter.exe