using System;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Threading;

class Program
{
    [StructLayout(LayoutKind.Sequential)]
    public struct SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX
    {
        public IntPtr Object;
        public IntPtr UniqueProcessId;
        public IntPtr HandleValue;
        public uint GrantedAccess;
        public ushort CreatorBackTraceIndex;
        public ushort ObjectTypeIndex;
        public uint HandleAttributes;
        public uint Reserved;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct UNICODE_STRING
    {
        public ushort Length;
        public ushort MaximumLength;
        public IntPtr Buffer;
    }

    [DllImport("ntdll.dll")]
    public static extern int NtQuerySystemInformation(int SystemInformationClass, IntPtr SystemInformation, int SystemInformationLength, out int ReturnLength);

    [DllImport("ntdll.dll")]
    public static extern int NtQueryObject(IntPtr Handle, int ObjectInformationClass, IntPtr ObjectInformation, int ObjectInformationLength, out int ReturnLength);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern IntPtr OpenProcess(uint processAccess, bool inheritHandle, int processId);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool DuplicateHandle(IntPtr hSourceProcessHandle, IntPtr hSourceHandle, IntPtr hTargetProcessHandle, out IntPtr lpTargetHandle, uint dwDesiredAccess, [MarshalAs(UnmanagedType.Bool)] bool bInheritHandle, uint dwOptions);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool CloseHandle(IntPtr hObject);

    [DllImport("kernel32.dll", SetLastError = true)]
    public static extern uint WaitForSingleObject(IntPtr hHandle, uint dwMilliseconds);

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    static extern bool SetForegroundWindow(IntPtr hWnd);

    [DllImport("user32.dll", SetLastError = true)]
    static extern IntPtr FindWindow(string lpClassName, string lpWindowName);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern IntPtr VirtualAllocEx(IntPtr hProcess, IntPtr lpAddress, uint dwSize, uint flAllocationType, uint flProtect);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool WriteProcessMemory(IntPtr hProcess, IntPtr lpBaseAddress, byte[] lpBuffer, uint nSize, out UIntPtr lpNumberOfBytesWritten);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern uint ResumeThread(IntPtr hThread);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern bool GetExitCodeThread(IntPtr hThread, out uint lpExitCode);

    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    static extern int MessageBox(IntPtr hWnd, string text, string caption, uint type);

    [DllImport("kernel32.dll", SetLastError = true)]
    static extern IntPtr CreateRemoteThread(IntPtr hProcess, IntPtr lpThreadAttributes, uint dwStackSize, IntPtr lpStartAddress, IntPtr lpParameter, uint dwCreationFlags, IntPtr lpThreadId);

    [DllImport("kernel32.dll", CharSet = CharSet.Ansi, ExactSpelling = true, SetLastError = true)]
    static extern IntPtr GetProcAddress(IntPtr hModule, string procName);

    [DllImport("kernel32.dll", CharSet = CharSet.Auto)]
    public static extern IntPtr GetModuleHandle(string lpModuleName);

    [DllImport("user32.dll", SetLastError = true)]
    static extern bool PostMessage(IntPtr hWnd, uint Msg, IntPtr wParam, IntPtr lParam);

    [DllImport("user32.dll")]
    static extern uint GetWindowThreadProcessId(IntPtr hWnd, IntPtr ProcessId);

    [DllImport("kernel32.dll")]
    static extern uint GetCurrentThreadId();

    [DllImport("user32.dll")]
    static extern bool AttachThreadInput(uint idAttach, uint idAttachTo, bool fAttach);

    [DllImport("user32.dll")]
    static extern IntPtr GetFocus();

    [DllImport("user32.dll")]
    [return: MarshalAs(UnmanagedType.Bool)]
    public static extern bool EnumChildWindows(IntPtr hwndParent, EnumWindowsProc lpEnumFunc, IntPtr lParam);
    public delegate bool EnumWindowsProc(IntPtr hWnd, IntPtr lParam);
    
    [DllImport("user32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    public static extern int GetClassName(IntPtr hWnd, System.Text.StringBuilder lpClassName, int nMaxCount);

    const uint WM_KEYDOWN = 0x0100;
    const uint WM_KEYUP = 0x0101;
    const uint WM_CHAR = 0x0102;
    const int VK_TAB = 0x09;

    const uint MEM_COMMIT = 0x00001000;
    const uint MEM_RESERVE = 0x00002000;
    const uint PAGE_READWRITE = 0x04;

    const int SystemExtendedHandleInformation = 0x40;
    const int ObjectNameInformation = 1;
    const int ObjectTypeInformation = 2;
    const uint DUPLICATE_CLOSE_SOURCE = 0x00000001;
    const uint DUPLICATE_SAME_ACCESS = 0x00000002;
    const uint PROCESS_DUP_HANDLE = 0x0040;
    const uint PROCESS_QUERY_LIMITED_INFORMATION = 0x1000;

    [StructLayout(LayoutKind.Sequential)]
    public struct STARTUPINFO
    {
        public int cb;
        public string lpReserved;
        public string lpDesktop;
        public string lpTitle;
        public int dwX, dwY, dwXSize, dwYSize;
        public int dwXCountChars, dwYCountChars;
        public int dwFillAttribute;
        public int dwFlags;
        public short wShowWindow;
        public short cbReserved2;
        public IntPtr lpReserved2;
        public IntPtr hStdInput;
        public IntPtr hStdOutput;
        public IntPtr hStdError;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct PROCESS_INFORMATION
    {
        public IntPtr hProcess;
        public IntPtr hThread;
        public int dwProcessId;
        public int dwThreadId;
    }

    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Auto)]
    static extern bool CreateProcess(
        string lpApplicationName,
        string lpCommandLine,
        IntPtr lpProcessAttributes,
        IntPtr lpThreadAttributes,
        bool bInheritHandles,
        uint dwCreationFlags,
        IntPtr lpEnvironment,
        string lpCurrentDirectory,
        ref STARTUPINFO lpStartupInfo,
        out PROCESS_INFORMATION lpProcessInformation);

    const uint CREATE_SUSPENDED = 0x00000004;
    const uint HIGH_PRIORITY_CLASS = 0x00000080;

    static void Main(string[] args)
    {
        if (args.Length < 3) {
            Console.WriteLine("Usage: launcher_bypass.exe [exePath] <ip> <port> <flag> [user] [pass] [spoofer_active]");
            return;
        }

        string exePath = @"D:\Sobee\Istanbul Kiyamet Vakti\istanbul.exe";
        int argOffset = 0;

        // If first argument looks like a path
        if (args[0].Contains("\\") || args[0].Contains("/")) {
            exePath = args[0];
            argOffset = 1;
        }

        string gameDir = exePath;
        if (exePath.ToLower().EndsWith(".exe")) {
            gameDir = Path.GetDirectoryName(exePath);
        } else {
            exePath = Path.Combine(gameDir, "istanbul.exe");
        }
        
        // Clean up old temporary instances left behind from previous runs
        try
        {
            foreach (var file in Directory.GetFiles(gameDir, "istanbul-*.exe"))
            {
                // Don't delete official launcher copies if they exist
                if (!file.Contains("eminonu") && !file.Contains("galata"))
                {
                    try { File.Delete(file); } catch { /* Ignore if still running */ }
                }
            }
        }
        catch { }

        string targetIp = args[argOffset];
        string targetPort = args[argOffset + 1];
        string targetFlag = args[argOffset + 2];
        
        string cmdArgs = string.Format("{0} {1} {2}", targetIp, targetPort, targetFlag);
        string macroUser = args.Length >= argOffset + 4 ? args[argOffset + 3] : "";
        string macroPass = args.Length >= argOffset + 5 ? args[argOffset + 4] : "";
        string spooferActive = args.Length >= argOffset + 6 ? args[argOffset + 5] : "true";

        // We do NOT pass credentials directly to istanbul.exe here, 
        // because it breaks IP parsing and causes it to connect to an empty IP.
        // Instead, the spoofer DLL will read them from the JSON file written below.

        string fixedName = targetPort == "27206" ? "istanbul-galata.exe" : "istanbul-eminonu.exe";
        if (!string.IsNullOrEmpty(macroUser)) {
            string safeUser = string.Join("_", macroUser.Split(Path.GetInvalidFileNameChars()));
            fixedName = "istanbul-" + safeUser + ".exe";
        }

        string targetExePath = Path.Combine(gameDir, fixedName);
        
        try {
            FileInfo sourceInfo = new FileInfo(exePath);
            FileInfo targetInfo = new FileInfo(targetExePath);
            if (!targetInfo.Exists || targetInfo.Length != sourceInfo.Length || targetInfo.LastWriteTimeUtc != sourceInfo.LastWriteTimeUtc) {
                File.Copy(exePath, targetExePath, true);
            }
        } catch {
            // Ignore copy errors
        }
        
        // Macro file will be written after process creation to append PID

        Console.WriteLine(string.Format("Starting {0} with args: {1}", targetExePath, cmdArgs));

        bool mutexCreated;
        bool eventCreated;
        using (Mutex launcherMutex = new Mutex(true, "IstanbulLauncher_SingleInstance", out mutexCreated))
        using (EventWaitHandle launcherEvent = new EventWaitHandle(false, EventResetMode.AutoReset, "svcessw", out eventCreated))
        {
            // Launch the game SUSPENDED so we can inject DLL before any code runs
            STARTUPINFO si = new STARTUPINFO();
            si.cb = Marshal.SizeOf(si);
            PROCESS_INFORMATION pi;

            string fullCommandLine = string.Format("\"{0}\" {1}", targetExePath, cmdArgs);
            bool created = CreateProcess(
                targetExePath,
                fullCommandLine,
                IntPtr.Zero,
                IntPtr.Zero,
                false,
                CREATE_SUSPENDED | HIGH_PRIORITY_CLASS,
                IntPtr.Zero,
                gameDir,
                ref si,
                out pi);

            if (!created)
            {
                Console.WriteLine("CreateProcess failed: " + Marshal.GetLastWin32Error());
                return;
            }

            Console.WriteLine(string.Format("Process created SUSPENDED with PID={0}", pi.dwProcessId));

            if (targetFlag != "0") {
                string macroFilePath = Path.Combine(Path.GetTempPath(), string.Format("ikv_macro_{0}.txt", pi.dwProcessId));
                if (!string.IsNullOrEmpty(macroUser)) {
                    try {
                        File.WriteAllText(macroFilePath, macroUser + "\n" + macroPass + "\n" + spooferActive);
                    } catch { }
                }
            }

            // --- DLL INJECTION BEFORE RESUME ---
            try {
                string dllPath = Path.Combine(AppDomain.CurrentDomain.BaseDirectory, "ikv_spoofer.dll");
                if (File.Exists(dllPath)) {
                    Console.WriteLine("Injecting HWID Spoofer DLL before resume...");
                    IntPtr hProcess = pi.hProcess;
                    IntPtr loadLibraryAddr = GetProcAddress(GetModuleHandle("kernel32.dll"), "LoadLibraryA");
                    IntPtr allocMemAddress = VirtualAllocEx(hProcess, IntPtr.Zero, (uint)((dllPath.Length + 1) * Marshal.SystemDefaultCharSize), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
                    
                    UIntPtr bytesWritten;
                    WriteProcessMemory(hProcess, allocMemAddress, System.Text.Encoding.Default.GetBytes(dllPath + "\0"), (uint)((dllPath.Length + 1) * Marshal.SystemDefaultCharSize), out bytesWritten);
                    
                    IntPtr hRemoteThread = CreateRemoteThread(hProcess, IntPtr.Zero, 0, loadLibraryAddr, allocMemAddress, 0, IntPtr.Zero);
                    bool injectionSuccess = false;
                    // Wait for LoadLibrary to complete
                    if (hRemoteThread != IntPtr.Zero)
                    {
                        WaitForSingleObject(hRemoteThread, 5000);
                        uint exitCode = 0;
                        if (GetExitCodeThread(hRemoteThread, out exitCode) && exitCode != 0) {
                            injectionSuccess = true;
                        }
                        CloseHandle(hRemoteThread);
                    }
                    if (!injectionSuccess) {
                        MessageBox(IntPtr.Zero, "Anti-Ban Sistemi Başlatılamadı!\n\nikv_spoofer.dll oyuna enjekte edilemedi.", "Kritik Hata", 0x10);
                    }
                    Console.WriteLine("DLL Injection complete. Resuming process...");
                } else {
                    MessageBox(IntPtr.Zero, "Anti-Ban Sistemi Başlatılamadı!\n\nikv_spoofer.dll bulunamadı.", "Kritik Hata", 0x10);
                    Console.WriteLine("DLL not found, resuming anyway: " + dllPath);
                }
            } catch (Exception ex) {
                Console.WriteLine("Injection error: " + ex.Message);
            }
            // --- DLL INJECTION END ---

            // Now resume the game — DLL hooks are already in place
            ResumeThread(pi.hThread);
            CloseHandle(pi.hThread);

            Process p;
            try { p = Process.GetProcessById(pi.dwProcessId); }
            catch { Console.WriteLine("Process already exited."); return; }

            if (!p.HasExited)
            {

                Console.WriteLine("Process started. Waiting for mutex svcmux...");
                IntPtr stolenHandle = IntPtr.Zero;
                bool closedInTarget = false;
                int error = 0;
                bool result = false;

                // Loop to wait for svcmux to be created by the game (up to 5 seconds)
                for (int i = 0; i < 100; i++)
                {
                    if (p.HasExited) break;
                    result = TryGetNamedMutexHandle(p.Id, "svcmux", out stolenHandle, out closedInTarget, out error);
                    if (result)
                    {
                        Console.WriteLine("Mutex stolen successfully!");
                        break;
                    }
                    Thread.Sleep(50);
                }
                
                // Macro file is now written before process creation to prevent race conditions

                // Keep process alive for a bit to hold the handles before exiting
                Thread.Sleep(2000);
            }
            else
            {
                Console.WriteLine("Process exited immediately.");
            }
        }
    }

    public static bool TryGetNamedMutexHandle(int targetPid, string mutexName, out IntPtr handleValue, out bool closedInTarget, out int closeWin32Error)
    {
        handleValue = IntPtr.Zero;
        closedInTarget = false;
        closeWin32Error = 0;

        IntPtr hProcess = OpenProcess(PROCESS_DUP_HANDLE | PROCESS_QUERY_LIMITED_INFORMATION, false, targetPid);
        if (hProcess == IntPtr.Zero) return false;

        try
        {
            int length = 0x10000;
            IntPtr ptr = Marshal.AllocHGlobal(length);
            int returnLength;

            while (NtQuerySystemInformation(SystemExtendedHandleInformation, ptr, length, out returnLength) != 0)
            {
                length *= 2;
                Marshal.FreeHGlobal(ptr);
                ptr = Marshal.AllocHGlobal(length);
            }

            long handleCount = Marshal.ReadIntPtr(ptr).ToInt64();
            Console.WriteLine(string.Format("Total handles in system: {0}", handleCount));
            int offset = IntPtr.Size * 2; // For SYSTEM_HANDLE_INFORMATION_EX, the handles array starts after 16 bytes on x64
            int structSize = Marshal.SizeOf(typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));

            int targetHandleCount = 0;

            for (long i = 0; i < handleCount; i++)
            {
                IntPtr handleEntryPtr = new IntPtr(ptr.ToInt64() + offset + (i * structSize));
                SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX handleInfo = (SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX)Marshal.PtrToStructure(handleEntryPtr, typeof(SYSTEM_HANDLE_TABLE_ENTRY_INFO_EX));

                if (handleInfo.UniqueProcessId.ToInt32() == targetPid)
                {
                    targetHandleCount++;
                    IntPtr hDup;
                    if (DuplicateHandle(hProcess, handleInfo.HandleValue, Process.GetCurrentProcess().Handle, out hDup, 0, false, DUPLICATE_SAME_ACCESS))
                    {
                        try
                        {
                            string typeName, objectName;
                            if (TryGetObjectTypeAndName(hDup, out typeName, out objectName))
                            {
                                if (typeName == "Mutant" || typeName == "Mutex")
                                {
                                    Console.WriteLine(string.Format("Found Mutex: {0}", objectName));
                                    if (!string.IsNullOrWhiteSpace(objectName) && (objectName.EndsWith("\\" + mutexName) || objectName == mutexName))
                                    {
                                        IntPtr hFinalDup;
                                        if (DuplicateHandle(hProcess, handleInfo.HandleValue, Process.GetCurrentProcess().Handle, out hFinalDup, 0, false, DUPLICATE_CLOSE_SOURCE | DUPLICATE_SAME_ACCESS))
                                        {
                                            closedInTarget = true;
                                            handleValue = hFinalDup; 
                                            return true;
                                        }
                                        else
                                        {
                                            closeWin32Error = Marshal.GetLastWin32Error();
                                            closedInTarget = true; // Best effort assumption
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                        finally
                        {
                            CloseHandle(hDup);
                        }
                    }
                }
            }

            Console.WriteLine(string.Format("Target process has {0} handles.", targetHandleCount));
            Marshal.FreeHGlobal(ptr);
        }
        finally
        {
            CloseHandle(hProcess);
        }

        return false;
    }

    public static bool TryGetObjectTypeAndName(IntPtr handle, out string typeName, out string objectName)
    {
        typeName = "";
        objectName = "";

        if (!TryQueryUnicodeString(handle, ObjectTypeInformation, out typeName)) return false;
        TryQueryUnicodeString(handle, ObjectNameInformation, out objectName);
        return true;
    }

    public static bool TryQueryUnicodeString(IntPtr handle, int infoClass, out string value)
    {
        value = "";
        int length = 0;
        NtQueryObject(handle, infoClass, IntPtr.Zero, 0, out length);
        if (length <= 0) return false;

        IntPtr ptr = Marshal.AllocHGlobal(length);
        try
        {
            int returnLength;
            if (NtQueryObject(handle, infoClass, ptr, length, out returnLength) != 0) return false;

            // NtQueryObject with ObjectTypeInformation (2) returns PUBLIC_OBJECT_TYPE_INFORMATION, 
            // where the UNICODE_STRING is at offset 0.
            // NtQueryObject with ObjectNameInformation (1) returns OBJECT_NAME_INFORMATION,
            // where the UNICODE_STRING is at offset 0.
            UNICODE_STRING unicodeString = (UNICODE_STRING)Marshal.PtrToStructure(ptr, typeof(UNICODE_STRING));
            if (unicodeString.Buffer != IntPtr.Zero && unicodeString.Length > 0)
            {
                value = Marshal.PtrToStringUni(unicodeString.Buffer, unicodeString.Length / 2);
                return true;
            }
        }
        finally
        {
            Marshal.FreeHGlobal(ptr);
        }
        return false;
    }
}
