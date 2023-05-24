#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char const *argv[])
{
    if (argc < 2 || strlen(argv[1]) == 0)
    {
        printf("No device instance path was given.\n");
        printf("1. Open Device Manager\n2. Find the driver you want to restart, right-click, and select Properties\n3. Navigate to the Details tab and find the Device Instance Path\n4. Right-click and select Copy\n5. Run the executable in the command prompt and put the device instance path between double quotes.\n");
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