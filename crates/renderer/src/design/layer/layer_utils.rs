use slotmap::{DefaultKey, Key, KeyData};

/// Convert u64 to DefaultKey.
/// Used for taking key from frontend and converting to backend format,
/// since SlotMap's DefaultKey isn't serializable.
pub fn convert_u64_to_default_key(raw_key: u64) -> DefaultKey {
    let key_data = KeyData::from_ffi(raw_key);
    let item_key: DefaultKey = key_data.into();
    item_key
}

/// Convert DefaultKey to u64.
/// Used for taking key from backend and converting to frontend format.
pub fn convert_default_key_to_u64(raw_key: DefaultKey) -> u64 {
    raw_key.data().as_ffi()
}
