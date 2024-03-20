use windows_sys::Wdk::Foundation::OBJECT_ATTRIBUTES;
use windows_sys::Wdk::System::SystemServices::{
    KEY_INFORMATION_CLASS, KEY_VALUE_INFORMATION_CLASS,
};
use windows_sys::Win32::Foundation::{BOOLEAN, HANDLE, NTSTATUS, UNICODE_STRING};
use windows_sys::Win32::System::IO::{IO_STATUS_BLOCK, PIO_APC_ROUTINE};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const REG_INIT_BOOT_SM: u32 = 0;
pub const REG_INIT_BOOT_SETUP: u32 = 1;
pub const REG_INIT_BOOT_ACCEPTED_BASE: u32 = 2;
pub const REG_INIT_BOOT_ACCEPTED_MAX: u32 = 1001;
pub const REG_MAX_KEY_VALUE_NAME_LENGTH: u32 = 32767;
pub const REG_MAX_KEY_NAME_LENGTH: u32 = 512;
pub const REG_FLAG_VOLATILE: u32 = 1;
pub const REG_FLAG_LINK: u32 = 2;
pub const REG_KEY_DONT_VIRTUALIZE: u32 = 2;
pub const REG_KEY_DONT_SILENT_FAIL: u32 = 4;
pub const REG_KEY_RECURSE_FLAG: u32 = 8;
pub const CM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8;
pub const VR_DEVICE_NAME: &[u8; 19] = b"\\Device\\VRegDriver\0";
pub const VR_FLAG_INHERIT_TRUST_CLASS: u32 = 1;
pub const VR_FLAG_WRITE_THROUGH_HIVE: u32 = 2;
pub const VR_FLAG_LOCAL_MACHINE_TRUST_CLASS: u32 = 4;
pub const VR_KEY_COMROOT: u32 = 0;
pub const VR_KEY_MACHINE_SOFTWARE: u32 = 1;
pub const VR_KEY_CONTROL_SET: u32 = 2;
pub const IOCTL_VR_INITIALIZE_JOB_FOR_VREG: u32 = 2228228;
pub const IOCTL_VR_LOAD_DIFFERENCING_HIVE: u32 = 2228232;
pub const IOCTL_VR_CREATE_NAMESPACE_NODE: u32 = 2228236;
pub const IOCTL_VR_MODIFY_FLAGS: u32 = 2228240;
pub const IOCTL_VR_CREATE_MULTIPLE_NAMESPACE_NODES: u32 = 2228244;
pub const IOCTL_VR_UNLOAD_DYNAMICALLY_LOADED_HIVES: u32 = 2228248;
pub const IOCTL_VR_GET_VIRTUAL_ROOT_KEY: u32 = 2228252;
pub const IOCTL_VR_LOAD_DIFFERENCING_HIVE_FOR_HOST: u32 = 2228256;
pub const IOCTL_VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST: u32 = 2228260;

#[repr(C)]
pub struct KEY_FLAGS_INFORMATION {
    pub Wow64Flags: u32,
    pub KeyFlags: u32,
    pub ControlFlags: u32,
}

#[repr(C)]
pub struct KEY_HANDLE_TAGS_INFORMATION {
    pub HandleTags: u32,
}

#[repr(C, align(4))]
pub struct KEY_SET_LAYER_INFORMATION {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl KEY_SET_LAYER_INFORMATION {
    #[inline]
    pub fn IsTombstone(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsTombstone(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsSupersedeLocal(&self) -> u32 {
        self._bitfield_1.get(1usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsSupersedeLocal(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 1u8, val as u64)
    }

    #[inline]
    pub fn IsSupersedeTree(&self) -> u32 {
        self._bitfield_1.get(2usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IsSupersedeTree(&mut self, val: u32) {
        self._bitfield_1.set(2usize, 1u8, val as u64)
    }

    #[inline]
    pub fn ClassIsInherited(&self) -> u32 {
        self._bitfield_1.get(3usize, 1u8) as u32
    }

    #[inline]
    pub fn set_ClassIsInherited(&mut self, val: u32) {
        self._bitfield_1.set(3usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved(&self) -> u32 {
        self._bitfield_1.get(4usize, 28u8) as u32
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u32) {
        self._bitfield_1.set(4usize, 28u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        IsTombstone: u32,
        IsSupersedeLocal: u32,
        IsSupersedeTree: u32,
        ClassIsInherited: u32,
        Reserved: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();
        bitfield_unit.set(0, 1, IsTombstone as u64);
        bitfield_unit.set(1, 1, IsSupersedeLocal as u64);
        bitfield_unit.set(2, 1, IsSupersedeTree as u64);
        bitfield_unit.set(3, 1, ClassIsInherited as u64);
        bitfield_unit.set(4, 28, Reserved as u64);
        bitfield_unit
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum CM_EXTENDED_PARAMETER_TYPE {
    CmExtendedParameterInvalidType = 0,
    CmExtendedParameterTrustClassKey = 1,
    CmExtendedParameterEvent = 2,
    CmExtendedParameterFileAccessToken = 3,
    CmExtendedParameterMax = 4,
}

#[repr(C)]
pub struct CM_EXTENDED_PARAMETER {
    pub Anonymous1: CM_EXTENDED_PARAMETER_1,
    pub Anonymous2: CM_EXTENDED_PARAMETER_2,
}

#[repr(C, align(8))]
pub struct CM_EXTENDED_PARAMETER_1 {
    _bitfield_align_1: [u64; 0],
    _bitfield_1: BitfieldUnit<[u8; 8]>,
}

impl CM_EXTENDED_PARAMETER_1 {
    #[inline]
    pub fn Type(&self) -> u64 {
        self._bitfield_1.get(0usize, 8u8)
    }

    #[inline]
    pub fn set_Type(&mut self, val: u64) {
        self._bitfield_1.set(0usize, 8u8, val)
    }

    #[inline]
    pub fn Reserved(&self) -> u64 {
        self._bitfield_1.get(8usize, 56u8)
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: u64) {
        self._bitfield_1.set(8usize, 56u8, val)
    }

    #[inline]
    pub fn new_bitfield_1(Type: u64, Reserved: u64) -> BitfieldUnit<[u8; 8]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 8]> = Default::default();

        bitfield_unit.set(0usize, 8u8, Type);

        bitfield_unit.set(8usize, 56u8, Reserved);

        bitfield_unit
    }
}

#[repr(C)]
pub struct CM_EXTENDED_PARAMETER_2 {
    pub ULong64: UnionField<u64>,
    pub Pointer: UnionField<*mut core::ffi::c_void>,
    pub Size: UnionField<usize>,
    pub Handle: UnionField<HANDLE>,
    pub ULong: UnionField<u32>,
    pub AccessMask: UnionField<u32>,
    pub union_field: u64,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum REG_ACTION {
    KeyAdded = 0,
    KeyRemoved = 1,
    KeyModified = 2,
}

#[repr(C)]
pub struct REG_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: REG_ACTION,
    pub KeyLength: u32,
    pub Key: [u16; 1],
}

#[repr(C)]
pub struct KEY_PID_ARRAY {
    pub ProcessId: HANDLE,
    pub KeyName: UNICODE_STRING,
}

#[repr(C)]
pub struct KEY_OPEN_SUBKEYS_INFORMATION {
    pub Count: u32,
    pub KeyArray: [KEY_PID_ARRAY; 1],
}

#[repr(C)]
pub struct VR_INITIALIZE_JOB_FOR_VREG {
    pub Job: HANDLE,
}

#[repr(C)]
pub struct VR_LOAD_DIFFERENCING_HIVE {
    pub Job: HANDLE,
    pub NextLayerIsHost: u32,
    pub Flags: u32,
    pub LoadFlags: u32,
    pub KeyPathLength: u16,
    pub HivePathLength: u16,
    pub NextLayerKeyPathLength: u16,
    pub FileAccessToken: HANDLE,
    pub Strings: [u16; 1],
}

#[repr(C)]
pub struct VR_CREATE_NAMESPACE_NODE {
    pub Job: HANDLE,
    pub ContainerPathLength: u16,
    pub HostPathLength: u16,
    pub Flags: u32,
    pub AccessMask: u32,
    pub Strings: [u16; 1],
}

#[repr(C)]
pub struct VR_MODIFY_FLAGS {
    pub Job: HANDLE,
    pub AddFlags: u32,
    pub RemoveFlags: u32,
}

#[repr(C)]
pub struct NAMESPACE_NODE_DATA {
    pub AccessMask: u32,
    pub ContainerPathLength: u16,
    pub HostPathLength: u16,
    pub Flags: u32,
    pub Strings: [u16; 1],
}

#[repr(C)]
pub struct VR_CREATE_MULTIPLE_NAMESPACE_NODES {
    pub Job: HANDLE,
    pub NumNewKeys: u32,
    pub Keys: [NAMESPACE_NODE_DATA; 1],
}

#[repr(C)]
pub struct VR_UNLOAD_DYNAMICALLY_LOADED_HIVES {
    pub Job: HANDLE,
}

#[repr(C)]
pub struct VR_GET_VIRTUAL_ROOT {
    pub Job: HANDLE,
    pub Index: u32,
}

#[repr(C)]
pub struct VR_GET_VIRTUAL_ROOT_RESULT {
    pub Key: HANDLE,
}

#[repr(C)]
pub struct VR_LOAD_DIFFERENCING_HIVE_FOR_HOST {
    pub LoadFlags: u32,
    pub Flags: u32,
    pub KeyPathLength: u16,
    pub HivePathLength: u16,
    pub NextLayerKeyPathLength: u16,
    pub FileAccessToken: HANDLE,
    pub Strings: [u16; 1],
}

#[repr(C)]
pub struct VR_UNLOAD_DIFFERENCING_HIVE_FOR_HOST {
    pub Reserved: u32,
    pub TargetKeyPathLength: u16,
    pub TargetKeyPath: [u16; 1],
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TitleIndex: u32,
        Class: *mut UNICODE_STRING,
        CreateOptions: u32,
        TransactionHandle: HANDLE,
        Disposition: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKey(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyTransacted(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtOpenKeyTransactedEx(
        KeyHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjectAttributes: *mut OBJECT_ATTRIBUTES,
        OpenOptions: u32,
        TransactionHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeleteKey(KeyHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtDeleteValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryKey(
        KeyHandle: HANDLE,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut core::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut core::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetValueKey(
        KeyHandle: HANDLE,
        ValueName: *mut UNICODE_STRING,
        TitleIndex: u32,
        Type: u32,
        Data: *mut core::ffi::c_void,
        DataSize: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtEnumerateKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyInformationClass: KEY_INFORMATION_CLASS,
        KeyInformation: *mut core::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtEnumerateValueKey(
        KeyHandle: HANDLE,
        Index: u32,
        KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
        KeyValueInformation: *mut core::ffi::c_void,
        Length: u32,
        ResultLength: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFlushKey(KeyHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompactKeys(Count: u32, KeyArray: *mut HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCompressKey(KeyHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey2(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKeyEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        TrustClassKey: HANDLE,
        Event: HANDLE,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut core::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLoadKey3(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        SourceFile: *mut OBJECT_ATTRIBUTES,
        Flags: u32,
        ExtendedParameters: *mut CM_EXTENDED_PARAMETER,
        ExtendedParameterCount: u32,
        DesiredAccess: u32,
        RootHandle: *mut HANDLE,
        Reserved: *mut core::ffi::c_void,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtReplaceKey(
        NewFile: *mut OBJECT_ATTRIBUTES,
        TargetHandle: HANDLE,
        OldFile: *mut OBJECT_ATTRIBUTES,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveKey(KeyHandle: HANDLE, FileHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveKeyEx(KeyHandle: HANDLE, FileHandle: HANDLE, Format: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSaveMergedKeys(
        HighPrecedenceKeyHandle: HANDLE,
        LowPrecedenceKeyHandle: HANDLE,
        FileHandle: HANDLE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtRestoreKey(KeyHandle: HANDLE, FileHandle: HANDLE, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKey(TargetKey: *mut OBJECT_ATTRIBUTES) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKey2(TargetKey: *mut OBJECT_ATTRIBUTES, Flags: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtUnloadKeyEx(TargetKey: *mut OBJECT_ATTRIBUTES, Event: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtNotifyChangeKey(
        KeyHandle: HANDLE,
        Event: HANDLE,
        ApcRoutine: PIO_APC_ROUTINE,
        ApcContext: *mut core::ffi::c_void,
        IoStatusBlock: *mut IO_STATUS_BLOCK,
        CompletionFilter: u32,
        WatchTree: BOOLEAN,
        Buffer: *mut core::ffi::c_void,
        BufferSize: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryOpenSubKeys(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        HandleCount: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtQueryOpenSubKeysEx(
        TargetKey: *mut OBJECT_ATTRIBUTES,
        BufferLength: u32,
        Buffer: *mut core::ffi::c_void,
        RequiredSize: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtInitializeRegistry(BootCondition: u16) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockRegistryKey(KeyHandle: HANDLE) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtLockProductActivationKeys(
        pPrivateVer: *mut u32,
        pSafeMode: *mut u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtFreezeRegistry(TimeOutInSeconds: u32) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtThawRegistry() -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCreateRegistryTransaction(
        RegistryTransactionHandle: *mut HANDLE,
        DesiredAccess: u32,
        ObjAttributes: *mut OBJECT_ATTRIBUTES,
        CreateOptions: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtCommitRegistryTransaction(
        RegistryTransactionHandle: HANDLE,
        Flags: u32,
    ) -> NTSTATUS;
}
