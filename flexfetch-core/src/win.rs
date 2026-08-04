//! Phase 8.9 — Windows FFI helpers for the Tier-2 collectors.
//!
//! Only compiled on Windows targets (`#[cfg(target_os = "windows")]` in
//! lib.rs). Thin, safe-ish wrappers over `windows-sys` so the module
//! collectors don't each repeat the same unsafe registry/UTF-16 boilerplate.

use windows_sys::Win32::System::Registry::{
    RegCloseKey, RegGetValueW, RegOpenKeyExW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, RRF_RT_REG_DWORD,
    RRF_RT_REG_SZ,
};

/// Encode a string as a null-terminated UTF-16 buffer (Windows API input).
pub fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

/// Open an HKLM subkey (returns a null HKEY when absent).
fn open_hklm(subkey: &str) -> HKEY {
    let subkey_w = wide(subkey);
    let mut hkey: HKEY = std::ptr::null_mut();
    let open = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            subkey_w.as_ptr(),
            0,
            KEY_READ,
            &mut hkey,
        )
    };
    if open != 0 {
        return std::ptr::null_mut();
    }
    hkey
}

/// Read a REG_SZ value under `HKLM\<subkey>\<value>`.
pub fn read_registry_string(subkey: &str, value: &str) -> Option<String> {
    let hkey = open_hklm(subkey);
    if hkey.is_null() {
        return None;
    }
    let value_w = wide(value);
    let mut buf = [0u16; 512];
    let mut len = (buf.len() * 2) as u32;
    let result = unsafe {
        RegGetValueW(
            hkey,
            std::ptr::null(),
            value_w.as_ptr(),
            RRF_RT_REG_SZ,
            std::ptr::null_mut(),
            buf.as_mut_ptr() as *mut _,
            &mut len,
        )
    };
    unsafe {
        RegCloseKey(hkey);
    }
    if result != 0 {
        return None;
    }
    let bytes = (len as usize).min(buf.len() * 2);
    let s = String::from_utf16_lossy(&buf[..bytes / 2]);
    Some(s.trim_end_matches('\0').to_string())
}

/// Read a REG_DWORD value under `HKLM\<subkey>\<value>`.
pub fn read_registry_dword(subkey: &str, value: &str) -> Option<u32> {
    let hkey = open_hklm(subkey);
    if hkey.is_null() {
        return None;
    }
    let value_w = wide(value);
    let mut out: u32 = 0;
    let mut len = std::mem::size_of::<u32>() as u32;
    let result = unsafe {
        RegGetValueW(
            hkey,
            std::ptr::null(),
            value_w.as_ptr(),
            RRF_RT_REG_DWORD,
            std::ptr::null_mut(),
            &mut out as *mut u32 as *mut _,
            &mut len,
        )
    };
    unsafe {
        RegCloseKey(hkey);
    }
    if result != 0 {
        return None;
    }
    Some(out)
}
