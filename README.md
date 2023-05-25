# What is Driver Restarter?

Driver Restarter is just a simple program **FOR WINDOWS** that will disable and enable your driver of choice.

## Why?

Well I had issues with my Windows 10 installation that I manually had to restart my driver of my front usb panel through device manager.

# How to run

## Method 1 Startup:

1. Open device manager

2. Find the driver you want restart and right click and press properties

3. navigate to details and find device instance path

4. Right click and press copy

5. Open task scheduler

6. Select run whether user logged in or not and run with highest privileges

7. Go to triggers and make a new trigger and select Begin task at startup and OK

8. Go to actions and then press new and press browse and select device-restarter.exe

9. Add in arguments the device instance path and press OK

10. Test if your driver starts automatically

## Method 2 Shortcut: 

1. Open device manager

2. Find the driver you want restart and right click and press properties

3. navigate to details and find device instance path

4. Right click and press copy

5. Right click the exe and press Create Shortcut

6. Right click the shortcut and click Properties

7. At the end of the target add a space and "" and paste your device instance path between the "". Example: YourPath\device-restarter.exe "DeviceInstanceId"

## Method 3 Command prompt:

1. Open device manager

2. Find the driver you want restart and right click and press properties

3. navigate to details and find device instance path

4. Right click and press copy

5. Run the exe in Command prompt as admin and put the device instance path between ""

## Something worth knowing:

You can put multiple DeviceInstances id's in: ```device-restarter-c.exe "DeviceInstanceId1" "DeviceInstanceId2"```

# Troubleshooting

## Admin privileges

The exe must be run as admin so right click and run as admin.

You can also go to properties and compatibility and set it to run as administrator

## Antivirus

Because the exe restarts your drivers it might be flagged as a false positive. So make sure you allow the program to run via your antivirus software.

# Compiling it yourself

## C

Make sure you have gcc installed and set the environment variables and just double click ```build.bat```

## Rust

Make sure you have [Cargo](https://www.rust-lang.org/tools/install) installed and just do ```cargo build``` and the exe should appear in the target\debug folder.

## C#

Make sure you have [Dotnet sdk](https://dotnet.microsoft.com/en-us/download) installed and just do ```dotnet build``` and the exe should appear in the bin\debug\netYourDotnetVersion folder.
