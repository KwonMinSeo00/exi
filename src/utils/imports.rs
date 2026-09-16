
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportRule {
    pub dll: &'static str,
    function: &'static str,
}

pub const IMPORTS: &[ImportRule] = &[
    ImportRule {
        dll: "kernel32.dll",
        function: "OpenProcess",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "VirtualAlloc",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "VirtualProtect",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "ReadProcessMemory",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "WriteProcessMemory",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "CreateRemoteThread",  
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "QueueUserAPC"  
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "GetThreadContext",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "SetThreadContext",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "SuspendThead",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "ResumeThead",
    },

    ImportRule {
        dll: "kernel32.dll",
        function: "SetWindowsHook",
    },

    // Continue...
];

