//! Validators add validation capabilities by wrapping and extending basic validators.

use crate::{validation::SharedContext, Fallible};
#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;
use core::{any::TypeId, fmt};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use hashbrown::HashMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

/// Errors that can occur when checking shared memory.
#[derive(Debug)]
pub enum SharedError {
    /// Multiple pointers exist to the same location with different types
    TypeMismatch {
        /// A previous type that the location was checked as
        previous: TypeId,
        /// The current type that the location is checked as
        current: TypeId,
    },
}

impl fmt::Display for SharedError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SharedError::TypeMismatch { previous, current } => write!(
                f,
                "the same memory region has been claimed as two different types ({:?} and {:?})",
                previous, current
            ),
        }
    }
}

#[cfg(feature = "std")]
const _: () = {
    use std::error::Error;

    impl Error for SharedError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            match self {
                SharedError::TypeMismatch { .. } => None,
            }
        }
    }
};

/// A validator that can verify shared memory.
pub struct SharedValidator {
    shared: HashMap<*const u8, TypeId>,
}

impl SharedValidator {
    /// Wraps the given context and adds shared memory validation.
    #[inline]
    pub fn new() -> Self {
        Self {
            shared: HashMap::new(),
        }
    }
}

impl Fallible for SharedValidator {
    type Error = SharedError;
}

impl SharedContext for SharedValidator {
    fn register_shared_ptr(
        &mut self,
        ptr: *const u8,
        type_id: TypeId,
    ) -> Result<bool, Self::Error> {
        if let Some(previous_type_id) = self.shared.get(&ptr) {
            if previous_type_id != &type_id {
                Err(SharedError::TypeMismatch {
                    previous: *previous_type_id,
                    current: type_id,
                })
            } else {
                Ok(false)
            }
        } else {
            self.shared.insert(ptr, type_id);
            Ok(true)
        }
    }
}
