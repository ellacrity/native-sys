use core::borrow::BorrowMut;

#[repr(C)]
#[derive(Copy, Clone, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BitfieldUnit<Storage> {
    storage: Storage,
}

impl<Storage> BitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}

impl<Storage: AsRef<[u8]> + AsMut<[u8]>> BitfieldUnit<Storage> {
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index =
            if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };

        let mask = 1 << bit_index;
        byte & mask == mask
    }

    /// Sets the target bit to the given value.
    ///
    /// # Panics
    ///
    /// Panics if the index is .
    #[inline]
    pub fn set_bit(&mut self, index: usize, value: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_mut()[byte_index].borrow_mut();
        let bit_index =
            if cfg!(target_endian = "big") { 7 - (index % 8) } else { index % 8 };

        let mask = 1 << bit_index;
        if value {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }

    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()
        );

        let mut val = 0;
        for index in 0..(bit_width as usize) {
            if self.get_bit(index + bit_offset) {
                let base_index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - index
                } else {
                    index
                };

                val |= 1 << base_index;
            }
        }

        val
    }

    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()
        );

        for index in 0..(bit_width as usize) {
            let mask = 1 << index;
            let val_bit_is_set = val & mask == mask;
            let base_index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - index
            } else {
                index
            };

            self.set_bit(base_index + bit_offset, val_bit_is_set);
        }
    }
}

#[repr(C)]
pub struct UnionField<T>(core::marker::PhantomData<T>);

impl<T> UnionField<T> {
    #[inline]
    pub const fn new() -> Self {
        Self(core::marker::PhantomData)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_ref(&self) -> &T {
        core::mem::transmute(self)
    }

    #[inline]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        core::mem::transmute(self)
    }
}

impl<T> core::default::Default for UnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> core::clone::Clone for UnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> core::marker::Copy for UnionField<T> {}

impl<T> core::hash::Hash for UnionField<T> {
    fn hash<H: core::hash::Hasher>(&self, _state: &mut H) {}
}

impl<T> core::cmp::PartialEq for UnionField<T> {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> core::cmp::Eq for UnionField<T> {}
