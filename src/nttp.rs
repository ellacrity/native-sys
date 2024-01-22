use windows_sys::Win32::Foundation::{HANDLE, NTSTATUS};
use windows_sys::Win32::System::Threading::{
    CRITICAL_SECTION, PTP_CALLBACK_INSTANCE, PTP_CLEANUP_GROUP, PTP_IO, PTP_POOL,
    PTP_SIMPLE_CALLBACK, PTP_TIMER, PTP_TIMER_CALLBACK, PTP_WAIT, PTP_WAIT_CALLBACK,
    PTP_WORK, PTP_WORK_CALLBACK, TP_CALLBACK_ENVIRON_V3, TP_POOL_STACK_INFORMATION,
};
use windows_sys::Win32::System::IO::IO_STATUS_BLOCK;

pub type PTP_ALPC_CALLBACK = core::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut core::ffi::c_void,
        Alpc: *mut core::ffi::c_void,
    ),
>;

pub type PTP_ALPC_CALLBACK_EX = core::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut core::ffi::c_void,
        Alpc: *mut core::ffi::c_void,
        ApcContext: *mut core::ffi::c_void,
    ),
>;

pub type PTP_IO_CALLBACK = core::option::Option<
    unsafe extern "system" fn(
        Instance: PTP_CALLBACK_INSTANCE,
        Context: *mut core::ffi::c_void,
        ApcContext: *mut core::ffi::c_void,
        IoSB: *mut IO_STATUS_BLOCK,
        Io: PTP_IO,
    ),
>;

#[repr(i32)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub enum TP_TRACE_TYPE {
    TpTraceThreadPriority = 1,
    TpTraceThreadAffinity = 2,
    MaxTpTraceType = 3,
}

#[link(name = "ntdll.dll", kind = "raw-dylib", modifiers = "+verbatim")]
extern "system" {
    pub fn TpAllocPool(
        PoolReturn: *mut PTP_POOL,
        Reserved: *mut core::ffi::c_void,
    ) -> NTSTATUS;
    pub fn TpReleasePool(Pool: PTP_POOL);
    pub fn TpSetPoolMaxThreads(Pool: PTP_POOL, MaxThreads: u32);
    pub fn TpSetPoolMinThreads(Pool: PTP_POOL, MinThreads: u32) -> NTSTATUS;
    pub fn TpQueryPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    pub fn TpSetPoolStackInformation(
        Pool: PTP_POOL,
        PoolStackInformation: *mut TP_POOL_STACK_INFORMATION,
    ) -> NTSTATUS;
    pub fn TpSetPoolThreadBasePriority(Pool: PTP_POOL, BasePriority: u32) -> NTSTATUS;
    pub fn TpAllocCleanupGroup(CleanupGroupReturn: *mut PTP_CLEANUP_GROUP) -> NTSTATUS;
    pub fn TpReleaseCleanupGroup(CleanupGroup: PTP_CLEANUP_GROUP);
    pub fn TpReleaseCleanupGroupMembers(
        CleanupGroup: PTP_CLEANUP_GROUP,
        CancelPendingCallbacks: u32,
        CleanupParameter: *mut core::ffi::c_void,
    );
    pub fn TpCallbackSetEventOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Event: HANDLE,
    );
    pub fn TpCallbackReleaseSemaphoreOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Semaphore: HANDLE,
        ReleaseCount: u32,
    );
    pub fn TpCallbackReleaseMutexOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        Mutex: HANDLE,
    );
    pub fn TpCallbackLeaveCriticalSectionOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        CriticalSection: *mut CRITICAL_SECTION,
    );
    pub fn TpCallbackUnloadDllOnCompletion(
        Instance: PTP_CALLBACK_INSTANCE,
        DllHandle: *mut core::ffi::c_void,
    );
    pub fn TpCallbackMayRunLong(Instance: PTP_CALLBACK_INSTANCE) -> NTSTATUS;
    pub fn TpDisassociateCallback(Instance: PTP_CALLBACK_INSTANCE);
    pub fn TpSimpleTryPost(
        Callback: PTP_SIMPLE_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpAllocWork(
        WorkReturn: *mut PTP_WORK,
        Callback: PTP_WORK_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpReleaseWork(Work: PTP_WORK);
    pub fn TpPostWork(Work: PTP_WORK);
    pub fn TpWaitForWork(Work: PTP_WORK, CancelPendingCallbacks: u32);
    pub fn TpAllocTimer(
        Timer: *mut PTP_TIMER,
        Callback: PTP_TIMER_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpReleaseTimer(Timer: PTP_TIMER);
    pub fn TpSetTimer(
        Timer: PTP_TIMER,
        DueTime: *mut i64,
        Period: u32,
        WindowLength: u32,
    );
    pub fn TpSetTimerEx(
        Timer: PTP_TIMER,
        DueTime: *mut i64,
        Period: u32,
        WindowLength: u32,
    ) -> NTSTATUS;
    pub fn TpIsTimerSet(Timer: PTP_TIMER) -> u32;
    pub fn TpWaitForTimer(Timer: PTP_TIMER, CancelPendingCallbacks: u32);
    pub fn TpAllocWait(
        WaitReturn: *mut PTP_WAIT,
        Callback: PTP_WAIT_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpReleaseWait(Wait: PTP_WAIT);
    pub fn TpSetWait(Wait: PTP_WAIT, Handle: HANDLE, Timeout: *mut i64);
    pub fn TpSetWaitEx(
        Wait: PTP_WAIT,
        Handle: HANDLE,
        Timeout: *mut i64,
        Reserved: *mut core::ffi::c_void,
    ) -> NTSTATUS;
    pub fn TpWaitForWait(Wait: PTP_WAIT, CancelPendingCallbacks: u32);
    pub fn TpAllocIoCompletion(
        IoReturn: *mut PTP_IO,
        File: HANDLE,
        Callback: PTP_IO_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpReleaseIoCompletion(Io: PTP_IO);
    pub fn TpStartAsyncIoOperation(Io: PTP_IO);
    pub fn TpCancelAsyncIoOperation(Io: PTP_IO);
    pub fn TpWaitForIoCompletion(Io: PTP_IO, CancelPendingCallbacks: u32);
    pub fn TpAllocAlpcCompletion(
        AlpcReturn: *mut *mut core::ffi::c_void,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpAllocAlpcCompletionEx(
        AlpcReturn: *mut *mut core::ffi::c_void,
        AlpcPort: HANDLE,
        Callback: PTP_ALPC_CALLBACK_EX,
        Context: *mut core::ffi::c_void,
        CallbackEnviron: *mut TP_CALLBACK_ENVIRON_V3,
    ) -> NTSTATUS;
    pub fn TpReleaseAlpcCompletion(Alpc: *mut core::ffi::c_void);
    pub fn TpWaitForAlpcCompletion(Alpc: *mut core::ffi::c_void);
    pub fn TpCaptureCaller(Type: TP_TRACE_TYPE);
    pub fn TpCheckTerminateWorker(Thread: HANDLE);
}
