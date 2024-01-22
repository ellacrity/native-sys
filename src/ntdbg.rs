use windows_sys::core::GUID;
use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows_sys::Win32::Foundation::{BOOLEAN, HANDLE, NTSTATUS};
use windows_sys::Win32::System::Diagnostics::Debug::{DEBUG_EVENT, EXCEPTION_RECORD};
use windows_sys::Win32::System::Diagnostics::Etw::PENABLECALLBACK;
use windows_sys::Win32::System::WindowsProgramming::CLIENT_ID;

use crate::bitfield::UnionField;
use crate::ntdef::PREGHANDLE;

pub const DEBUG_READ_EVENT: u32 = 1;
pub const DEBUG_PROCESS_ASSIGN: u32 = 2;
pub const DEBUG_SET_INFORMATION: u32 = 4;
pub const DEBUG_QUERY_INFORMATION: u32 = 8;
pub const DEBUG_ALL_ACCESS: u32 = 2031631;
pub const DEBUG_KILL_ON_CLOSE: u32 = 1;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUserBreakPoint();
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgBreakPoint();
}

#[repr(C)]
pub struct DBGKM_EXCEPTION {
    pub ExceptionRecord: EXCEPTION_RECORD,
    pub FirstChance: u32,
}

#[repr(C)]
pub struct DBGKM_CREATE_THREAD {
    pub SubSystemKey: u32,
    pub StartAddress: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct DBGKM_CREATE_PROCESS {
    pub SubSystemKey: u32,
    pub FileHandle: HANDLE,
    pub BaseOfImage: *mut core::ffi::c_void,
    pub DebugInfoFileOffset: u32,
    pub DebugInfoSize: u32,
    pub InitialThread: DBGKM_CREATE_THREAD,
}

#[repr(C)]
pub struct DBGKM_EXIT_THREAD {
    pub ExitStatus: NTSTATUS,
}

#[repr(C)]
pub struct DBGKM_EXIT_PROCESS {
    pub ExitStatus: NTSTATUS,
}

#[repr(C)]
pub struct DBGKM_LOAD_DLL {
    pub FileHandle: HANDLE,
    pub BaseOfDll: *mut core::ffi::c_void,
    pub DebugInfoFileOffset: u32,
    pub DebugInfoSize: u32,
    pub NamePointer: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct DBGKM_UNLOAD_DLL {
    pub BaseAddress: *mut core::ffi::c_void,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum DBG_STATE {
    DbgIdle = 0,
    DbgReplyPending = 1,
    DbgCreateThreadStateChange = 2,
    DbgCreateProcessStateChange = 3,
    DbgExitThreadStateChange = 4,
    DbgExitProcessStateChange = 5,
    DbgExceptionStateChange = 6,
    DbgBreakpointStateChange = 7,
    DbgSingleStepStateChange = 8,
    DbgLoadDllStateChange = 9,
    DbgUnloadDllStateChange = 10,
}

#[repr(C)]
pub struct DBGUI_CREATE_THREAD {
    pub HandleToThread: HANDLE,
    pub NewThread: DBGKM_CREATE_THREAD,
}

#[repr(C)]
pub struct DBGUI_CREATE_PROCESS {
    pub HandleToProcess: HANDLE,
    pub HandleToThread: HANDLE,
    pub NewProcess: DBGKM_CREATE_PROCESS,
}

#[repr(C)]
pub struct DBGUI_WAIT_STATE_CHANGE {
    pub NewState: DBG_STATE,
    pub AppClientId: CLIENT_ID,
    pub StateInfo: DBGUI_WAIT_STATE_CHANGE_1,
}

#[repr(C)]
pub struct DBGUI_WAIT_STATE_CHANGE_1 {
    pub Exception: UnionField<DBGKM_EXCEPTION>,
    pub CreateThread: UnionField<DBGUI_CREATE_THREAD>,
    pub CreateProcessInfo: UnionField<DBGUI_CREATE_PROCESS>,
    pub ExitThread: UnionField<DBGKM_EXIT_THREAD>,
    pub ExitProcess: UnionField<DBGKM_EXIT_PROCESS>,
    pub LoadDll: UnionField<DBGKM_LOAD_DLL>,
    pub UnloadDll: UnionField<DBGKM_UNLOAD_DLL>,
    pub union_field: [u64; 20],
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum DEBUGOBJECTINFOCLASS {
    DebugObjectUnusedInformation = 0,
    DebugObjectKillProcessOnExitInformation = 1,
    MaxDebugObjectInfoClass = 2,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateDebugObject(
        DebugObjectHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDebugActiveProcess(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDebugContinue(
        DebugObjectHandle: HANDLE,
        ClientId: *mut CLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRemoveProcessDebug(
        ProcessHandle: HANDLE,
        DebugObjectHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetInformationDebugObject(
        DebugObjectHandle: HANDLE,
        DebugObjectInformationClass: DEBUGOBJECTINFOCLASS,
        DebugInformation: *mut core::ffi::c_void,
        DebugInformationLength: u32,
        ReturnLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtWaitForDebugEvent(
        DebugObjectHandle: HANDLE,
        Alertable: BOOLEAN,
        Timeout: *mut i64,
        WaitStateChange: *mut DBGUI_WAIT_STATE_CHANGE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConnectToDbg() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiGetThreadDebugObject() -> HANDLE;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiSetThreadDebugObject(DebugObject: HANDLE);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiWaitStateChange(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        Timeout: *mut i64,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiContinue(
        AppClientId: *mut CLIENT_ID,
        ContinueStatus: NTSTATUS,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiStopDebugging(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiDebugActiveProcess(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiRemoteBreakin(Context: *mut core::ffi::c_void);
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiIssueRemoteBreakin(Process: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConvertStateChangeStructure(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        DebugEvent: *mut DEBUG_EVENT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn DbgUiConvertStateChangeStructureEx(
        StateChange: *mut DBGUI_WAIT_STATE_CHANGE,
        DebugEvent: *mut DEBUG_EVENT,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn EtwEventRegister(
        ProviderId: *const GUID,
        EnableCallback: PENABLECALLBACK,
        CallbackContext: *mut core::ffi::c_void,
        RegHandle: PREGHANDLE,
    ) -> NTSTATUS;
}
