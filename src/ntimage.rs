use crate::bitfield::{BitfieldUnit, UnionField};

pub const IMAGE_FILE_MACHINE_CHPE_X86: u32 = 14948;
pub const IMAGE_DVRT_ARM64X_FIXUP_TYPE_ZEROFILL: u32 = 0;
pub const IMAGE_DVRT_ARM64X_FIXUP_TYPE_VALUE: u32 = 1;
pub const IMAGE_DVRT_ARM64X_FIXUP_TYPE_DELTA: u32 = 2;
pub const IMAGE_DVRT_ARM64X_FIXUP_SIZE_2BYTES: u32 = 1;
pub const IMAGE_DVRT_ARM64X_FIXUP_SIZE_4BYTES: u32 = 2;
pub const IMAGE_DVRT_ARM64X_FIXUP_SIZE_8BYTES: u32 = 3;
pub const IMAGE_DYNAMIC_RELOCATION_ARM64X: u32 = 6;
pub const IMAGE_DYNAMIC_RELOCATION_MM_SHARED_USER_DATA_VA: u32 = 2147352576;
pub const IMAGE_DEBUG_POGO_SIGNATURE_LTCG: &[u8; 4] = b"LTCG";
pub const IMAGE_DEBUG_POGO_SIGNATURE_PGU: &[u8; 4] = b"PGU\0";
pub const IMAGE_DYNAMIC_RELOCATION_KI_USER_SHARED_DATA64: u64 = 18446734727860715520;

#[repr(C)]
pub struct IMAGE_DEBUG_POGO_ENTRY {
    pub Rva: u32,
    pub Size: u32,
    pub Name: [i8; 1],
}

#[repr(C)]
pub struct IMAGE_DEBUG_POGO_SIGNATURE {
    pub Signature: u32,
}

#[repr(C, align(2))]
pub struct IMAGE_RELOCATION_RECORD {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl IMAGE_RELOCATION_RECORD {
    #[inline]
    pub fn Offset(&self) -> u16 {
        self._bitfield_1.get(0usize, 12u8) as u16
    }

    #[inline]
    pub fn set_Offset(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 12u8, val as u64)
    }

    #[inline]
    pub fn Type(&self) -> u16 {
        self._bitfield_1.get(12usize, 4u8) as u16
    }

    #[inline]
    pub fn set_Type(&mut self, val: u16) {
        self._bitfield_1.set(12usize, 4u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(Offset: u16, Type: u16) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 12u8, Offset as u64);

        bitfield_unit.set(12usize, 4u8, Type as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct IMAGE_CHPE_METADATA_X86 {
    pub Version: u32,
    pub CHPECodeAddressRangeOffset: u32,
    pub CHPECodeAddressRangeCount: u32,
    pub WowA64ExceptionHandlerFunctionPointer: u32,
    pub WowA64DispatchCallFunctionPointer: u32,
    pub WowA64DispatchIndirectCallFunctionPointer: u32,
    pub WowA64DispatchIndirectCallCfgFunctionPointer: u32,
    pub WowA64DispatchRetFunctionPointer: u32,
    pub WowA64DispatchRetLeafFunctionPointer: u32,
    pub WowA64DispatchJumpFunctionPointer: u32,
    pub CompilerIATPointer: u32,
    pub WowA64RdtscFunctionPointer: u32,
}

#[repr(C)]
pub struct IMAGE_CHPE_RANGE_ENTRY {
    pub Anonymous1: IMAGE_CHPE_RANGE_ENTRY_1,
    pub Length: u32,
}

#[repr(C)]
pub struct IMAGE_CHPE_RANGE_ENTRY_1 {
    pub StartOffset: UnionField<u32>,
    pub Anonymous1: UnionField<IMAGE_CHPE_RANGE_ENTRY_1_1>,
    pub union_field: u32,
}

#[repr(C, align(4))]
pub struct IMAGE_CHPE_RANGE_ENTRY_1_1 {
    _bitfield_align_1: [u32; 0],
    _bitfield_1: BitfieldUnit<[u8; 4]>,
}

impl IMAGE_CHPE_RANGE_ENTRY_1_1 {
    #[inline]
    pub fn NativeCode(&self) -> u32 {
        self._bitfield_1.get(0usize, 1u8) as u32
    }

    #[inline]
    pub fn set_NativeCode(&mut self, val: u32) {
        self._bitfield_1.set(0usize, 1u8, val as u64)
    }

    #[inline]
    pub fn AddressBits(&self) -> u32 {
        self._bitfield_1.get(1usize, 31u8) as u32
    }

    #[inline]
    pub fn set_AddressBits(&mut self, val: u32) {
        self._bitfield_1.set(1usize, 31u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(NativeCode: u32, AddressBits: u32) -> BitfieldUnit<[u8; 4]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 4]> = Default::default();

        bitfield_unit.set(0usize, 1u8, NativeCode as u64);

        bitfield_unit.set(1usize, 31u8, AddressBits as u64);

        bitfield_unit
    }
}

#[repr(C)]
pub struct IMAGE_ARM64EC_METADATA {
    pub Version: u32,
    pub CodeMap: u32,
    pub CodeMapCount: u32,
    pub CodeRangesToEntryPoints: u32,
    pub RedirectionMetadata: u32,
    pub tbd__os_arm64x_dispatch_call_no_redirect: u32,
    pub tbd__os_arm64x_dispatch_ret: u32,
    pub tbd__os_arm64x_dispatch_call: u32,
    pub tbd__os_arm64x_dispatch_icall: u32,
    pub tbd__os_arm64x_dispatch_icall_cfg: u32,
    pub AlternateEntryPoint: u32,
    pub AuxiliaryIAT: u32,
    pub CodeRangesToEntryPointsCount: u32,
    pub RedirectionMetadataCount: u32,
    pub GetX64InformationFunctionPointer: u32,
    pub SetX64InformationFunctionPointer: u32,
    pub ExtraRFETable: u32,
    pub ExtraRFETableSize: u32,
    pub __os_arm64x_dispatch_fptr: u32,
    pub AuxiliaryIATCopy: u32,
}

#[repr(C)]
pub struct IMAGE_ARM64EC_REDIRECTION_ENTRY {
    pub Source: u32,
    pub Destination: u32,
}

#[repr(C)]
pub struct IMAGE_ARM64EC_CODE_RANGE_ENTRY_POINT {
    pub StartRva: u32,
    pub EndRva: u32,
    pub EntryPoint: u32,
}

#[repr(C, align(2))]
pub struct IMAGE_DVRT_ARM64X_FIXUP_RECORD {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl IMAGE_DVRT_ARM64X_FIXUP_RECORD {
    #[inline]
    pub fn Offset(&self) -> u16 {
        self._bitfield_1.get(0usize, 12u8) as u16
    }

    #[inline]
    pub fn set_Offset(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 12u8, val as u64)
    }

    #[inline]
    pub fn Type(&self) -> u16 {
        self._bitfield_1.get(12usize, 2u8) as u16
    }

    #[inline]
    pub fn set_Type(&mut self, val: u16) {
        self._bitfield_1.set(12usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Size(&self) -> u16 {
        self._bitfield_1.get(14usize, 2u8) as u16
    }

    #[inline]
    pub fn set_Size(&mut self, val: u16) {
        self._bitfield_1.set(14usize, 2u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(Offset: u16, Type: u16, Size: u16) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 12u8, Offset as u64);

        bitfield_unit.set(12usize, 2u8, Type as u64);

        bitfield_unit.set(14usize, 2u8, Size as u64);

        bitfield_unit
    }
}

#[repr(C, align(2))]
pub struct IMAGE_DVRT_ARM64X_DELTA_FIXUP_RECORD {
    _bitfield_align_1: [u16; 0],
    _bitfield_1: BitfieldUnit<[u8; 2]>,
}

impl IMAGE_DVRT_ARM64X_DELTA_FIXUP_RECORD {
    #[inline]
    pub fn Offset(&self) -> u16 {
        self._bitfield_1.get(0usize, 12u8) as u16
    }

    #[inline]
    pub fn set_Offset(&mut self, val: u16) {
        self._bitfield_1.set(0usize, 12u8, val as u64)
    }

    #[inline]
    pub fn Type(&self) -> u16 {
        self._bitfield_1.get(12usize, 2u8) as u16
    }

    #[inline]
    pub fn set_Type(&mut self, val: u16) {
        self._bitfield_1.set(12usize, 2u8, val as u64)
    }

    #[inline]
    pub fn Sign(&self) -> u16 {
        self._bitfield_1.get(14usize, 1u8) as u16
    }

    #[inline]
    pub fn set_Sign(&mut self, val: u16) {
        self._bitfield_1.set(14usize, 1u8, val as u64)
    }

    #[inline]
    pub fn Scale(&self) -> u16 {
        self._bitfield_1.get(15usize, 1u8) as u16
    }

    #[inline]
    pub fn set_Scale(&mut self, val: u16) {
        self._bitfield_1.set(15usize, 1u8, val as u64)
    }

    #[inline]
    pub fn new_bitfield_1(
        Offset: u16,
        Type: u16,
        Sign: u16,
        Scale: u16,
    ) -> BitfieldUnit<[u8; 2]> {
        let mut bitfield_unit: BitfieldUnit<[u8; 2]> = Default::default();

        bitfield_unit.set(0usize, 12u8, Offset as u64);

        bitfield_unit.set(12usize, 2u8, Type as u64);

        bitfield_unit.set(14usize, 1u8, Sign as u64);

        bitfield_unit.set(15usize, 1u8, Scale as u64);

        bitfield_unit
    }
}
