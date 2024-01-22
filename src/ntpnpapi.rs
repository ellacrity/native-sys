use windows_sys::core::GUID;
use windows_sys::Win32::Devices::DeviceAndDriverInstallation::PNP_VETO_TYPE;
use windows_sys::Win32::Foundation::{NTSTATUS, UNICODE_STRING};

use crate::bitfield::UnionField;

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PLUGPLAY_EVENT_CATEGORY {
    HardwareProfileChangeEvent = 0,
    TargetDeviceChangeEvent = 1,
    DeviceClassChangeEvent = 2,
    CustomDeviceEvent = 3,
    DeviceInstallEvent = 4,
    DeviceArrivalEvent = 5,
    PowerEvent = 6,
    VetoEvent = 7,
    BlockedDriverEvent = 8,
    InvalidIDEvent = 9,
    MaxPlugEventCategory = 10,
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK {
    pub EventGuid: GUID,
    pub EventCategory: PLUGPLAY_EVENT_CATEGORY,
    pub Result: *mut u32,
    pub Flags: u32,
    pub TotalSize: u32,
    pub DeviceObject: *mut core::ffi::c_void,
    pub u: PLUGPLAY_EVENT_BLOCK_1,
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1 {
    pub DeviceClass: UnionField<PLUGPLAY_EVENT_BLOCK_1_1>,
    pub TargetDevice: UnionField<PLUGPLAY_EVENT_BLOCK_1_2>,
    pub InstallDevice: UnionField<PLUGPLAY_EVENT_BLOCK_1_3>,
    pub CustomNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_4>,
    pub ProfileNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_5>,
    pub PowerNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_6>,
    pub VetoNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_7>,
    pub BlockedDriverNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_8>,
    pub InvalidIDNotification: UnionField<PLUGPLAY_EVENT_BLOCK_1_9>,
    pub union_field: [u64; 3],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_1 {
    pub ClassGuid: GUID,
    pub SymbolicLinkName: [u16; 1],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_2 {
    pub DeviceIds: [u16; 1],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_3 {
    pub DeviceId: [u16; 1],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_4 {
    pub NotificationStructure: *mut core::ffi::c_void,
    pub DeviceIds: [u16; 1],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_5 {
    pub Notification: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_6 {
    pub NotificationCode: u32,
    pub NotificationData: u32,
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_7 {
    pub VetoType: PNP_VETO_TYPE,
    pub DeviceIdVetoNameBuffer: [u16; 1],
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_8 {
    pub BlockedDriverGuid: GUID,
}

#[repr(C)]
pub struct PLUGPLAY_EVENT_BLOCK_1_9 {
    pub ParentId: [u16; 1],
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PLUGPLAY_CONTROL_CLASS {
    PlugPlayControlEnumerateDevice = 0,
    PlugPlayControlRegisterNewDevice = 1,
    PlugPlayControlDeregisterDevice = 2,
    PlugPlayControlInitializeDevice = 3,
    PlugPlayControlStartDevice = 4,
    PlugPlayControlUnlockDevice = 5,
    PlugPlayControlQueryAndRemoveDevice = 6,
    PlugPlayControlUserResponse = 7,
    PlugPlayControlGenerateLegacyDevice = 8,
    PlugPlayControlGetInterfaceDeviceList = 9,
    PlugPlayControlProperty = 10,
    PlugPlayControlDeviceClassAssociation = 11,
    PlugPlayControlGetRelatedDevice = 12,
    PlugPlayControlGetInterfaceDeviceAlias = 13,
    PlugPlayControlDeviceStatus = 14,
    PlugPlayControlGetDeviceDepth = 15,
    PlugPlayControlQueryDeviceRelations = 16,
    PlugPlayControlTargetDeviceRelation = 17,
    PlugPlayControlQueryConflictList = 18,
    PlugPlayControlRetrieveDock = 19,
    PlugPlayControlResetDevice = 20,
    PlugPlayControlHaltDevice = 21,
    PlugPlayControlGetBlockedDriverList = 22,
    PlugPlayControlGetDeviceInterfaceEnabled = 23,
    MaxPlugPlayControl = 24,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtPlugPlayControl(
        PnPControlClass: PLUGPLAY_CONTROL_CLASS,
        PnPControlData: *mut core::ffi::c_void,
        PnPControlDataLength: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSerializeBoot() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtEnableLastKnownGood() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDisableLastKnownGood() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplacePartitionUnit(
        TargetInstancePath: *mut UNICODE_STRING,
        SpareInstancePath: *mut UNICODE_STRING,
        Flags: u32,
    ) -> NTSTATUS;
}
