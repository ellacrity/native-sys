use windows_sys::Win32::Foundation::{BOOLEAN, NTSTATUS};
use windows_sys::Win32::System::Diagnostics::Debug::{CONTEXT, EXCEPTION_RECORD};

pub const KCONTINUE_FLAG_TEST_ALERT: u32 = 1;
pub const KCONTINUE_FLAG_DELIVER_APC: u32 = 2;

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlDispatchException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
    ) -> BOOLEAN;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn RtlRaiseStatus(Status: NTSTATUS) -> !;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtContinue(ContextRecord: *mut CONTEXT, TestAlert: BOOLEAN) -> NTSTATUS;
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum KCONTINUE_TYPE {
    KCONTINUE_UNWIND = 0,
    KCONTINUE_RESUME = 1,
    KCONTINUE_LONGJUMP = 2,
    KCONTINUE_SET = 3,
    KCONTINUE_LAST = 4,
}

#[repr(C)]
pub struct KCONTINUE_ARGUMENT {
    pub ContinueType: KCONTINUE_TYPE,
    pub ContinueFlags: u32,
    pub Reserved: [u64; 2],
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtContinueEx(
        ContextRecord: *mut CONTEXT,
        ContinueArgument: *mut core::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRaiseException(
        ExceptionRecord: *mut EXCEPTION_RECORD,
        ContextRecord: *mut CONTEXT,
        FirstChance: BOOLEAN,
    ) -> NTSTATUS;
}
