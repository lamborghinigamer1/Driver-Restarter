#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char const *argv[])
{
    if (argc <= 1 || strlen(argv[1]) == 0)
    {
        printf("No device instance path was given.\n");
        printf("Please read the manual for more information\n");
        system("start https://github.com/lamborghinigamer1/Driver-Restarter/blob/main/README.md");
        printf("Press any key to exit...");
        getchar();
        return 1;
    }
    else
    {
        char CommandDisable[100] = "pnputil /disable-device \"";
        char CommandEnable[100] = "pnputil /enable-device \"";

        for (int i = 1; i < argc; i++)
        {
            strcat(CommandDisable, argv[i]);
            strcat(CommandDisable, "\"");

            strcat(CommandEnable, argv[i]);
            strcat(CommandEnable, "\"");

            system(CommandDisable);
            system(CommandEnable);

            strcpy(CommandDisable, "pnputil /disable-device \"");
            strcpy(CommandEnable, "pnputil /enable-device \"");
        }
        return 0;
    }
}