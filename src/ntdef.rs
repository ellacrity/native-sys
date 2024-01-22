use windows_sys::Win32::Foundation::BOOLEAN;

pub const NT_FACILITY_MASK: u32 = 4095;
pub const NT_FACILITY_SHIFT: u32 = 16;
pub const OBJ_PROTECT_CLOSE: u32 = 1;
pub const INT_ERROR: i32 = -1;
pub const DESKTOP_ALL_ACCESS: u32 = 983551;
pub const DESKTOP_GENERIC_READ: u32 = 131137;
pub const DESKTOP_GENERIC_WRITE: u32 = 131262;
pub const DESKTOP_GENERIC_EXECUTE: u32 = 131328;
pub const WINSTA_GENERIC_READ: u32 = 131843;
pub const WINSTA_GENERIC_WRITE: u32 = 131100;
pub const WINSTA_GENERIC_EXECUTE: u32 = 131168;
pub const WMIGUID_GENERIC_READ: u32 = 131085;
pub const WMIGUID_GENERIC_WRITE: u32 = 131170;
pub const WMIGUID_GENERIC_EXECUTE: u32 = 134800;

#[repr(C)]
#[repr(align(16))]
pub struct QUAD_PTR {
    pub DoNotUseThisField1: usize,
    pub DoNotUseThisField2: usize,
}

#[repr(transparent)]
#[derive(Copy, Hash, PartialEq, Eq)]
pub struct PREGHANDLE(pub u64);

impl PREGHANDLE {
    pub const fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}

impl core::clone::Clone for PREGHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}

#[repr(transparent)]
#[derive(Copy, Hash, PartialEq, Eq)]
pub struct TRACEHANDLE(pub u64);

impl TRACEHANDLE {
    pub const fn is_invalid(&self) -> bool {
        self.0 == 0
    }
}

impl core::clone::Clone for TRACEHANDLE {
    fn clone(&self) -> Self {
        *self
    }
}

pub type PENCLAVE_ROUTINE =
    Option<unsafe extern "system" fn(lpThreadParameter: *mut core::ffi::c_void) -> u32>;

pub type WAITORTIMERCALLBACKFUNC =
    Option<unsafe extern "system" fn(_: *mut core::ffi::c_void, _: BOOLEAN)>;
