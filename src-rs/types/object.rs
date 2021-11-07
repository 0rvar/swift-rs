use std::ops::Deref;

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug)]
#[repr(C)]
pub struct SRObject<T>(*const SRObjectImpl<T>);

#[derive(Debug)]
#[repr(C)]
struct SRObjectImpl<T> {
    _nsobject_offset: u8,
    data: T,
}

impl<T> Deref for SRObject<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.0).data }
    }
}

impl<T> Serialize for SRObject<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.deref().serialize(serializer)
    }
}