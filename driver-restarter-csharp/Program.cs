using System.Diagnostics;

class DriverRestarter
{
    public void Disable(string deviceid)
    {
        string disable = $"/C pnputil /disable-device \"{deviceid}\"";
        ProcessStartInfo Disableaction = new ProcessStartInfo("CMD.exe", disable);
        Process.Start(Disableaction);
    }

    public void Enable(string deviceid)
    {
        string enable = $"/C pnputil /enable-device \"{deviceid}\"";
        ProcessStartInfo Enableaction = new ProcessStartInfo("CMD.exe", enable);
        Process.Start(Enableaction);
    }

    static void Main(string[] args)
    {
        if (args.Length == 0)
        {
            Console.WriteLine("\nNo device instance path was given.");
            Console.WriteLine("Please read the manual for more information");
            Console.WriteLine("Press any key to exit...");
            string redirect = $"/C start https://github.com/lamborghinigamer1/Driver-Restarter/blob/main/README.md";
            ProcessStartInfo manual = new ProcessStartInfo("CMD.exe", redirect);
            Process.Start(manual);
            Console.ReadLine();
        }
        else
        {
            DriverRestarter restart = new DriverRestarter();
            foreach (string deviceid in args)
            {
                restart.Disable(deviceid);
                restart.Enable(deviceid);
            }
        }
    }
}