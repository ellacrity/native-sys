#![allow(clippy::useless_transmute)]

use windows_sys::Win32::Foundation::{BOOLEAN, HANDLE, NTSTATUS};
use windows_sys::Win32::System::Kernel::PROCESSOR_NUMBER;
use windows_sys::Win32::System::Power::{
    DEVICE_POWER_STATE, EXECUTION_STATE, POWER_ACTION, POWER_MONITOR_REQUEST_REASON,
    SYSTEM_POWER_STATE,
};

use crate::bitfield::{BitfieldUnit, UnionField};

pub const POWER_REQUEST_CONTEXT_NOT_SPECIFIED: u32 = 2147483648;
pub const PROCESSOR_STATE_TYPE_PERFORMANCE: u32 = 1;
pub const PROCESSOR_STATE_TYPE_THROTTLE: u32 = 2;
pub const IDLE_STATE_FLAGS_C1_HLT: u32 = 1;
pub const IDLE_STATE_FLAGS_C1_IO_HLT: u32 = 2;
pub const IDLE_STATE_FLAGS_IO: u32 = 4;
pub const IDLE_STATE_FLAGS_MWAIT: u32 = 8;
pub const POWER_REQUEST_SUPPORTED_TYPES_V1: u32 = 3;
pub const POWER_REQUEST_SUPPORTED_TYPES_V2: u32 = 9;
pub const POWER_REQUEST_SUPPORTED_TYPES_V3: u32 = 5;
pub const POWER_REQUEST_SUPPORTED_TYPES_V4: u32 = 6;

#[repr(C)]
pub struct SYSTEM_HIBERFILE_INFORMATION {
    pub NumberOfMcbPairs: u32,
    pub Mcb: [i64; 1],
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum POWER_REQUEST_TYPE_INTERNAL {
    PowerRequestDisplayRequiredInternal = 0,
    PowerRequestSystemRequiredInternal = 1,
    PowerRequestAwayModeRequiredInternal = 2,
    PowerRequestExecutionRequiredInternal = 3,
    PowerRequestPerfBoostRequiredInternal = 4,
    PowerRequestActiveLockScreenInternal = 5,
    PowerRequestInternalInvalid = 6,
    PowerRequestInternalUnknown = 7,
    PowerRequestFullScreenVideoRequired = 8,
}

#[repr(C)]
pub struct POWER_REQUEST_ACTION {
    pub PowerRequestHandle: HANDLE,
    pub RequestType: POWER_REQUEST_TYPE_INTERNAL,
    pub SetAction: BOOLEAN,
    pub ProcessHandle: HANDLE,
}

#[repr(C)]
pub struct POWER_STATE {
    pub SystemState: UnionField<SYSTEM_POWER_STATE>,
    pub DeviceState: UnionField<DEVICE_POWER_STATE>,
    pub union_field: u32,
}

#[repr(C)]
#[repr(align(4))]
pub struct SYSTEM_POWER_STATE_CONTEXT_1_1 {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl SYSTEM_POWER_STATE_CONTEXT_1_1 {
    #[inline]
    pub fn Reserved1(&self) -> u32 {
        self._bitfield_1.get(0usize, 8u8) as u32
    }

    #[inline]
    pub fn set_Reserved1(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 8u8, val as u64)
    }

    #[inline]
    pub fn TargetSystemState(&self) -> u32 {
        self._bitfield_1.get(8usize, 4u8) as u32
    }

    #[inline]
    pub fn set_TargetSystemState(&mut self, val: u32) {
        self._bitfield_1.set(8usize, 4u8, val as u64)
    }

    #[inline]
    pub fn EffectiveSystemState(&self) -> u32 {
        self._bitfield_1.get(12usize, 4u8) as u32
    }

    #[inline]
    pub fn set_EffectiveSystemState(&mut self, val: u32) {
        self._bitfield_1.set(12usize, 4u8, val as u64)
    }

    #[inline]
    pub fn CurrentSystemState(&self) -> u32 {
        self._bitfield_1.get(16usize, 4u8) as u32
    }

    #[inline]
    pub fn set_CurrentSystemState(&mut self, val: u32) {
        self._bitfield_1.set(16usize, 4u8, val as u64)
    }

    #[inline]
    pub fn IgnoreHibernationPath(&self) -> u32 {
        self._bitfield_1.get(20usize, 1u8) as u32
    }

    #[inline]
    pub fn set_IgnoreHibernationPath(&mut self, val: u32) {
        self._bitfield_1.set(20usize, 1u8, val as u64)
    }

    #[inline]
    pub fn PseudoTransition(&self) -> u32 {
        self._bitfield_1.get(21usize, 1u8) as u32
    }

    #[inline]
    pub fn set_PseudoTransition(&mut self, val: u32) {
        self._bitfield_1.set(21usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Reserved2(&self) -> u32 {
        self._bitfield_1.get(22usize, 10u8) as u32
    }

    #[inline]
    pub fn set_Reserved2(&mut self, val: u32) {
        self._bitfield_1.set(22usize, 10u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Reserved1: u32,
        TargetSystemState: u32,
        EffectiveSystemState: u32,
        CurrentSystemState: u32,
        IgnoreHibernationPath: u32,
        PseudoTransition: u32,
        Reserved2: u32,
    ) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 8u8, Reserved1 as u64);

        bitfield_unit.set(8usize, 4u8, TargetSystemState as u64);

        bitfield_unit.set(12usize, 4u8, EffectiveSystemState as u64);

        bitfield_unit.set(16usize, 4u8, CurrentSystemState as u64);

        bitfield_unit.set(20usize, 1u8, IgnoreHibernationPath as u64);

        bitfield_unit.set(21usize, 1u8, PseudoTransition as u64);

        bitfield_unit.set(22usize, 10u8, Reserved2 as u64);

        bitfield_unit
    }
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum REQUESTER_TYPE {
    KernelRequester = 0,
    UserProcessRequester = 1,
    UserSharedServiceRequester = 2,
}
pub struct COUNTED_REASON_CONTEXT_RELATIVE {
    pub Flags: u32,
    pub Anonymous1: COUNTED_REASON_CONTEXT_RELATIVE_1,
}

#[repr(C)]
pub struct COUNTED_REASON_CONTEXT_RELATIVE_1 {
    pub Anonymous1: UnionField<COUNTED_REASON_CONTEXT_RELATIVE_1_1>,
    pub SimpleStringOffset: UnionField<usize>,
    pub union_field: [u64; 3],
}

#[repr(C)]
pub struct COUNTED_REASON_CONTEXT_RELATIVE_1_1 {
    pub ResourceFileNameOffset: usize,
    pub ResourceReasonId: u16,
    pub StringCount: u32,
    pub SubstitutionStringsOffset: usize,
}

#[repr(C)]
pub struct DIAGNOSTIC_BUFFER {
    pub Size: usize,
    pub CallerType: REQUESTER_TYPE,
    pub Anonymous1: DIAGNOSTIC_BUFFER_1,
    pub ReasonOffset: usize,
}

#[repr(C)]
pub struct DIAGNOSTIC_BUFFER_1 {
    pub Anonymous1: UnionField<DIAGNOSTIC_BUFFER_1_1>,
    pub Anonymous2: UnionField<DIAGNOSTIC_BUFFER_1_2>,
    pub union_field: [u64; 2],
}

#[repr(C)]
pub struct DIAGNOSTIC_BUFFER_1_1 {
    pub ProcessImageNameOffset: usize,
    pub ProcessId: u32,
    pub ServiceTag: u32,
}

#[repr(C)]
pub struct DIAGNOSTIC_BUFFER_1_2 {
    pub DeviceDescriptionOffset: usize,
    pub DevicePathOffset: usize,
}

#[repr(C)]
pub struct WAKE_TIMER_INFO {
    pub OffsetToNext: usize,
    pub DueTime: u64,
    pub Period: u32,
    pub ReasonContext: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct PROCESSOR_PERF_CAP_HV {
    pub Version: u32,
    pub InitialApicId: u32,
    pub Ppc: u32,
    pub Tpc: u32,
    pub ThermalCap: u32,
}

#[repr(C)]
pub struct PROCESSOR_IDLE_TIMES {
    pub StartTime: u64,
    pub EndTime: u64,
    pub Reserved: [u32; 4],
}

pub type PROCESSOR_IDLE_HANDLER = core::option::Option<
    unsafe extern "system" fn(
        Context: usize,
        IdleTimes: *mut PROCESSOR_IDLE_TIMES,
    ) -> NTSTATUS,
>;

pub type PPROCESSOR_IDLE_HANDLER = PROCESSOR_IDLE_HANDLER;

#[repr(C)]
pub struct PROCESSOR_IDLE_STATE {
    pub StateType: u8,
    pub StateFlags: u32,
    pub HardwareLatency: u32,
    pub Power: u32,
    pub Context: usize,
    pub Handler: PPROCESSOR_IDLE_HANDLER,
}

#[repr(C)]
pub struct PROCESSOR_IDLE_STATES {
    pub Size: u32,
    pub Revision: u32,
    pub Count: u32,
    pub Type: u32,
    pub TargetProcessors: usize,
    pub State: [PROCESSOR_IDLE_STATE; 1],
}

#[repr(C)]
pub struct PROCESSOR_LOAD {
    pub ProcessorNumber: PROCESSOR_NUMBER,
    pub BusyPercentage: u8,
    pub FrequencyPercentage: u8,
    pub Padding: u16,
}

#[repr(C)]
pub struct PROCESSOR_CAP {
    pub Version: u32,
    pub ProcessorNumber: PROCESSOR_NUMBER,
    pub PlatformCap: u32,
    pub ThermalCap: u32,
    pub LimitReasons: u32,
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_INFO {
    pub Count: u32,
    pub Offsets: [u32; 1],
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_HISTORY {
    pub Count: u32,
    pub Offsets: [u32; 1],
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PO_WAKE_SOURCE_TYPE {
    DeviceWakeSourceType = 0,
    FixedWakeSourceType = 1,
    TimerWakeSourceType = 2,
    TimerPresumedWakeSourceType = 3,
    InternalWakeSourceType = 4,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PO_INTERNAL_WAKE_SOURCE_TYPE {
    InternalWakeSourceDozeToHibernate = 0,
    InternalWakeSourcePredictedUserPresence = 1,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum PO_FIXED_WAKE_SOURCE_TYPE {
    FixedWakeSourcePowerButton = 0,
    FixedWakeSourceSleepButton = 1,
    FixedWakeSourceRtc = 2,
    FixedWakeSourceDozeToHibernate = 3,
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_HEADER {
    pub Type: PO_WAKE_SOURCE_TYPE,
    pub Size: u32,
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_DEVICE {
    pub Header: PO_WAKE_SOURCE_HEADER,
    pub InstancePath: [u16; 1],
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_FIXED {
    pub Header: PO_WAKE_SOURCE_HEADER,
    pub FixedWakeSourceType: PO_FIXED_WAKE_SOURCE_TYPE,
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_INTERNAL {
    pub Header: PO_WAKE_SOURCE_HEADER,
    pub InternalWakeSourceType: PO_INTERNAL_WAKE_SOURCE_TYPE,
}

#[repr(C)]
pub struct PO_WAKE_SOURCE_TIMER {
    pub Header: PO_WAKE_SOURCE_HEADER,
    pub Reason: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct POWER_REQUEST {
    pub Anonymous1: POWER_REQUEST_1,
}

#[repr(C)]
pub struct POWER_REQUEST_1 {
    pub V1: UnionField<POWER_REQUEST_1_1>,
    pub V2: UnionField<POWER_REQUEST_1_2>,
    pub V3: UnionField<POWER_REQUEST_1_3>,
    pub V4: UnionField<POWER_REQUEST_1_4>,
    pub union_field: [u64; 10],
}

#[repr(C)]
pub struct POWER_REQUEST_1_1 {
    pub SupportedRequestMask: u32,
    pub PowerRequestCount: [u32; 3],
    pub DiagnosticBuffer: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct POWER_REQUEST_1_2 {
    pub SupportedRequestMask: u32,
    pub PowerRequestCount: [u32; 9],
    pub DiagnosticBuffer: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct POWER_REQUEST_1_3 {
    pub SupportedRequestMask: u32,
    pub PowerRequestCount: [u32; 5],
    pub DiagnosticBuffer: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct POWER_REQUEST_1_4 {
    pub SupportedRequestMask: u32,
    pub PowerRequestCount: [u32; 6],
    pub DiagnosticBuffer: DIAGNOSTIC_BUFFER,
}

#[repr(C)]
pub struct POWER_REQUEST_LIST {
    pub Count: usize,
    pub PowerRequestOffsets: [usize; 1],
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum POWER_STATE_HANDLER_TYPE {
    PowerStateSleeping1 = 0,
    PowerStateSleeping2 = 1,
    PowerStateSleeping3 = 2,
    PowerStateSleeping4 = 3,
    PowerStateShutdownOff = 4,
    PowerStateShutdownReset = 5,
    PowerStateSleeping4Firmware = 6,
    PowerStateMaximum = 7,
}

pub type PENTER_STATE_SYSTEM_HANDLER = core::option::Option<
    unsafe extern "system" fn(SystemContext: *mut core::ffi::c_void) -> NTSTATUS,
>;

pub type PENTER_STATE_HANDLER = core::option::Option<
    unsafe extern "system" fn(
        Context: *mut core::ffi::c_void,
        SystemHandler: PENTER_STATE_SYSTEM_HANDLER,
        SystemContext: *mut core::ffi::c_void,
        NumberProcessors: i32,
        Number: *mut i32,
    ) -> NTSTATUS,
>;

#[repr(C)]
pub struct POWER_STATE_HANDLER {
    pub Type: POWER_STATE_HANDLER_TYPE,
    pub RtcWake: BOOLEAN,
    pub Spare: [u8; 3],
    pub Handler: PENTER_STATE_HANDLER,
    pub Context: *mut core::ffi::c_void,
}

pub type PENTER_STATE_NOTIFY_HANDLER = core::option::Option<
    unsafe extern "system" fn(
        State: POWER_STATE_HANDLER_TYPE,
        Context: *mut core::ffi::c_void,
        Entering: BOOLEAN,
    ) -> NTSTATUS,
>;

#[repr(C)]
pub struct POWER_STATE_NOTIFY_HANDLER {
    pub Handler: PENTER_STATE_NOTIFY_HANDLER,
    pub Context: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct POWER_REQUEST_ACTION_INTERNAL {
    pub PowerRequestPointer: *mut core::ffi::c_void,
    pub RequestType: POWER_REQUEST_TYPE_INTERNAL,
    pub SetAction: BOOLEAN,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum POWER_INFORMATION_LEVEL_INTERNAL {
    PowerInternalAcpiInterfaceRegister = 0,
    PowerInternalS0LowPowerIdleInfo = 1,
    PowerInternalReapplyBrightnessSettings = 2,
    PowerInternalUserAbsencePrediction = 3,
    PowerInternalUserAbsencePredictionCapability = 4,
    PowerInternalPoProcessorLatencyHint = 5,
    PowerInternalStandbyNetworkRequest = 6,
    PowerInternalDirtyTransitionInformation = 7,
    PowerInternalSetBackgroundTaskState = 8,
    PowerInternalTtmOpenTerminal = 9,
    PowerInternalTtmCreateTerminal = 10,
    PowerInternalTtmEvacuateDevices = 11,
    PowerInternalTtmCreateTerminalEventQueue = 12,
    PowerInternalTtmGetTerminalEvent = 13,
    PowerInternalTtmSetDefaultDeviceAssignment = 14,
    PowerInternalTtmAssignDevice = 15,
    PowerInternalTtmSetDisplayState = 16,
    PowerInternalTtmSetDisplayTimeouts = 17,
    PowerInternalBootSessionStandbyActivationInformation = 18,
    PowerInternalSessionPowerState = 19,
    PowerInternalSessionTerminalInput = 20,
    PowerInternalSetWatchdog = 21,
    PowerInternalPhysicalPowerButtonPressInfoAtBoot = 22,
    PowerInternalExternalMonitorConnected = 23,
    PowerInternalHighPrecisionBrightnessSettings = 24,
    PowerInternalWinrtScreenToggle = 25,
    PowerInternalPpmQosDisable = 26,
    PowerInternalTransitionCheckpoint = 27,
    PowerInternalInputControllerState = 28,
    PowerInternalFirmwareResetReason = 29,
    PowerInternalPpmSchedulerQosSupport = 30,
    PowerInternalBootStatGet = 31,
    PowerInternalBootStatSet = 32,
    PowerInternalCallHasNotReturnedWatchdog = 33,
    PowerInternalBootStatCheckIntegrity = 34,
    PowerInternalBootStatRestoreDefaults = 35,
    PowerInternalHostEsStateUpdate = 36,
    PowerInternalGetPowerActionState = 37,
    PowerInternalBootStatUnlock = 38,
    PowerInternalWakeOnVoiceState = 39,
    PowerInternalDeepSleepBlock = 40,
    PowerInternalIsPoFxDevice = 41,
    PowerInternalPowerTransitionExtensionAtBoot = 42,
    PowerInternalProcessorBrandedFrequency = 43,
    PowerInternalTimeBrokerExpirationReason = 44,
    PowerInternalNotifyUserShutdownStatus = 45,
    PowerInternalPowerRequestTerminalCoreWindow = 46,
    PowerInternalProcessorIdleVeto = 47,
    PowerInternalPlatformIdleVeto = 48,
    PowerInternalIsLongPowerButtonBugcheckEnabled = 49,
    PowerInternalAutoChkCausedReboot = 50,
    PowerInternalSetWakeAlarmOverride = 51,
    PowerInternalDirectedFxAddTestDevice = 53,
    PowerInternalDirectedFxRemoveTestDevice = 54,
    PowerInternalDirectedFxSetMode = 56,
    PowerInternalRegisterPowerPlane = 57,
    PowerInternalSetDirectedDripsFlags = 58,
    PowerInternalClearDirectedDripsFlags = 59,
    PowerInternalRetrieveHiberFileResumeContext = 60,
    PowerInternalReadHiberFilePage = 61,
    PowerInternalLastBootSucceeded = 62,
    PowerInternalQuerySleepStudyHelperRoutineBlock = 63,
    PowerInternalDirectedDripsQueryCapabilities = 64,
    PowerInternalClearConstraints = 65,
    PowerInternalSoftParkVelocityEnabled = 66,
    PowerInternalQueryIntelPepCapabilities = 67,
    PowerInternalGetSystemIdleLoopEnablement = 68,
    PowerInternalGetVmPerfControlSupport = 69,
    PowerInternalGetVmPerfControlConfig = 70,
    PowerInternalSleepDetailedDiagUpdate = 71,
    PowerInternalProcessorClassFrequencyBandsStats = 72,
    PowerInternalHostGlobalUserPresenceStateUpdate = 73,
    PowerInternalCpuNodeIdleIntervalStats = 74,
    PowerInternalClassIdleIntervalStats = 75,
    PowerInternalCpuNodeConcurrencyStats = 76,
    PowerInternalClassConcurrencyStats = 77,
    PowerInternalQueryProcMeasurementCapabilities = 78,
    PowerInternalQueryProcMeasurementValues = 79,
    PowerInternalPrepareForSystemInitiatedReboot = 80,
    PowerInternalGetAdaptiveSessionState = 81,
    PowerInternalSetConsoleLockedState = 82,
    PowerInternalOverrideSystemInitiatedRebootState = 83,
    PowerInternalFanImpactStats = 84,
    PowerInternalFanRpmBuckets = 85,
    PowerInternalPowerBootAppDiagInfo = 86,
    PowerInternalUnregisterShutdownNotification = 87,
    PowerInternalManageTransitionStateRecord = 88,
    PowerInternalGetAcpiTimeAndAlarmCapabilities = 89,
    PowerInternalSuspendResumeRequest = 90,
    PowerInformationInternalMaximum = 91,
}

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum POWER_S0_DISCONNECTED_REASON {
    PoS0DisconnectedReasonNone = 0,
    PoS0DisconnectedReasonNonCompliantNic = 1,
    PoS0DisconnectedReasonSettingPolicy = 2,
    PoS0DisconnectedReasonEnforceDsPolicy = 3,
    PoS0DisconnectedReasonCsChecksFailed = 4,
    PoS0DisconnectedReasonSmartStandby = 5,
    PoS0DisconnectedReasonMaximum = 6,
}

#[repr(C)]
pub struct POWER_S0_LOW_POWER_IDLE_INFO {
    pub DisconnectedReason: POWER_S0_DISCONNECTED_REASON,
    pub CsDeviceCompliance: POWER_S0_LOW_POWER_IDLE_INFO_1,
    pub Policy: POWER_S0_LOW_POWER_IDLE_INFO_2,
}

#[repr(C)]
pub struct POWER_S0_LOW_POWER_IDLE_INFO_1 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 1]>>,
    pub AsUCHAR: UnionField<u8>,
    pub union_field: u8,
}

impl POWER_S0_LOW_POWER_IDLE_INFO_1 {
    #[inline]
    pub fn Storage(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(0usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_Storage(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn WiFi(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(1usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_WiFi(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(1usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Mbn(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(2usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_Mbn(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(2usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Ethernet(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(3usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_Ethernet(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(3usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(4usize, 4u8) as u8)
        }
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(4usize, 4u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        Storage: BOOLEAN,
        WiFi: BOOLEAN,
        Mbn: BOOLEAN,
        Ethernet: BOOLEAN,
        Reserved: BOOLEAN,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            let Storage: u8 = unsafe { core::mem::transmute(Storage) };

            Storage as u64
        });

        bitfield_unit.set(1usize, 1u8, {
            let WiFi: u8 = unsafe { core::mem::transmute(WiFi) };

            WiFi as u64
        });

        bitfield_unit.set(2usize, 1u8, {
            let Mbn: u8 = unsafe { core::mem::transmute(Mbn) };

            Mbn as u64
        });

        bitfield_unit.set(3usize, 1u8, {
            let Ethernet: u8 = unsafe { core::mem::transmute(Ethernet) };

            Ethernet as u64
        });

        bitfield_unit.set(4usize, 4u8, {
            let Reserved: u8 = unsafe { core::mem::transmute(Reserved) };

            Reserved as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
pub struct POWER_S0_LOW_POWER_IDLE_INFO_2 {
    _bitfield_align_1: [u8; 0],
    _bitfield_1: UnionField<BitfieldUnit<[u8; 1]>>,
    pub AsUCHAR: UnionField<u8>,
    pub union_field: u8,
}

impl POWER_S0_LOW_POWER_IDLE_INFO_2 {
    #[inline]
    pub fn DisconnectInStandby(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(0usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_DisconnectInStandby(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(0usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn EnforceDs(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(1usize, 1u8) as u8)
        }
    }

    #[inline]
    pub fn set_EnforceDs(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(1usize, 1u8, val as u64)
        }
    }

    #[inline]
    pub fn Reserved(&self) -> BOOLEAN {
        unsafe {
            core::mem::transmute(self._bitfield_1.as_ref().get(2usize, 6u8) as u8)
        }
    }

    #[inline]
    pub fn set_Reserved(&mut self, val: BOOLEAN) {
        unsafe {
            let val: u8 = core::mem::transmute(val);

            self._bitfield_1.as_mut().set(2usize, 6u8, val as u64)
        }
    }

    #[inline]
    pub fn new_bitfield_1(
        DisconnectInStandby: BOOLEAN,
        EnforceDs: BOOLEAN,
        Reserved: BOOLEAN,
    ) -> BitfieldUnit<[u8; 1]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 1]> = Default::default();

        bitfield_unit.set(0usize, 1u8, {
            let DisconnectInStandby: u8 =
                unsafe { core::mem::transmute(DisconnectInStandby) };

            DisconnectInStandby as u64
        });

        bitfield_unit.set(1usize, 1u8, {
            let EnforceDs: u8 = unsafe { core::mem::transmute(EnforceDs) };

            EnforceDs as u64
        });

        bitfield_unit.set(2usize, 6u8, {
            let Reserved: u8 = unsafe { core::mem::transmute(Reserved) };

            Reserved as u64
        });

        bitfield_unit
    }
}

#[repr(C)]
pub struct POWER_INFORMATION_INTERNAL_HEADER {
    pub InternalType: POWER_INFORMATION_LEVEL_INTERNAL,
    pub Version: u32,
}

#[repr(C)]
pub struct POWER_USER_ABSENCE_PREDICTION {
    pub Header: POWER_INFORMATION_INTERNAL_HEADER,
    pub ReturnTime: i64,
}

#[repr(C)]
pub struct POWER_USER_ABSENCE_PREDICTION_CAPABILITY {
    pub AbsencePredictionCapability: BOOLEAN,
}

#[repr(C)]
pub struct POWER_PROCESSOR_LATENCY_HINT {
    pub PowerInformationInternalHeader: POWER_INFORMATION_INTERNAL_HEADER,
    pub Type: u32,
}

#[repr(C)]
pub struct POWER_STANDBY_NETWORK_REQUEST {
    pub PowerInformationInternalHeader: POWER_INFORMATION_INTERNAL_HEADER,
    pub Active: BOOLEAN,
}

#[repr(C)]
pub struct POWER_SET_BACKGROUND_TASK_STATE {
    pub PowerInformationInternalHeader: POWER_INFORMATION_INTERNAL_HEADER,
    pub Engaged: BOOLEAN,
}

#[repr(C)]
pub struct POWER_BOOT_SESSION_STANDBY_ACTIVATION_INFO {
    pub StandbyTotalTime: u32,
    pub DripsTotalTime: u32,
    pub ActivatorClientTotalActiveTime: u32,
    pub PerActivatorClientTotalActiveTime: [u32; 98],
}

#[repr(C)]
pub struct POWER_SESSION_POWER_STATE {
    pub Header: POWER_INFORMATION_INTERNAL_HEADER,
    pub SessionId: u32,
    pub On: BOOLEAN,
    pub IsConsole: BOOLEAN,
    pub RequestReason: POWER_MONITOR_REQUEST_REASON,
}

#[repr(C)]
pub struct POWER_INTERNAL_PROCESSOR_QOS_SUPPORT {
    pub QosSupportedAndConfigured: BOOLEAN,
    pub SchedulerDirectedPerfStatesSupported: BOOLEAN,
    pub QosGroupPolicyDisable: BOOLEAN,
}

#[repr(C)]
pub struct POWER_INTERNAL_HOST_ENERGY_SAVER_STATE {
    pub Header: POWER_INFORMATION_INTERNAL_HEADER,
    pub EsEnabledOnHost: BOOLEAN,
}

#[repr(C)]
pub struct POWER_INTERNAL_PROCESSOR_BRANDED_FREQENCY_INPUT {
    pub InternalType: POWER_INFORMATION_LEVEL_INTERNAL,
    pub ProcessorNumber: PROCESSOR_NUMBER,
}

#[repr(C)]
pub struct POWER_INTERNAL_PROCESSOR_BRANDED_FREQENCY_OUTPUT {
    pub Version: u32,
    pub NominalFrequency: u32,
}

#[repr(C)]
pub struct POWER_INTERNAL_BOOTAPP_DIAGNOSTIC {
    pub BootAppErrorDiagCode: u32,
    pub BootAppFailureStatus: u32,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetThreadExecutionState(
        NewFlags: EXECUTION_STATE,
        PreviousFlags: *mut EXECUTION_STATE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtInitiatePowerAction(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
        Asynchronous: BOOLEAN,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtSetSystemPowerState(
        SystemAction: POWER_ACTION,
        LightestSystemState: SYSTEM_POWER_STATE,
        Flags: u32,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtGetDevicePowerState(
        Device: HANDLE,
        State: *mut DEVICE_POWER_STATE,
    ) -> NTSTATUS;
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn NtIsSystemResumeAutomatic() -> BOOLEAN;
}
