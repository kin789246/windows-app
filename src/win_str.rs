use std::borrow::Cow;
use windows::core::*;
use windows::Win32::Globalization::*;

pub fn hstr_to_pcwstr(h: &HSTRING) -> PCWSTR {
    PCWSTR::from_raw(h.as_ptr())
}

pub fn str_to_pcwstr(s: &str) -> PCWSTR {
    PCWSTR::from_raw(HSTRING::from(s).as_ptr())
}

pub fn str_to_hstring(s: &str) -> HSTRING {
    HSTRING::from(s)
}

/// Wrapper for MultiByteToWideChar.
///
/// See https://msdn.microsoft.com/en-us/library/windows/desktop/dd319072(v=vs.85).aspx
/// for more details.
/// refer to https://github.com/bozaro/local-encoding-rs
pub fn multi_byte_to_wide_char(
    codepage: u32,
    flags: MULTI_BYTE_TO_WIDE_CHAR_FLAGS,
    multi_byte_str: &[u8]
) -> std::result::Result<Cow<'static, str>, Cow<'static, str>> {
    // Empty string
    if multi_byte_str.len() == 0 {
        return Ok(Cow::Owned(String::new()));
    }
    unsafe {
        // Get length of UTF-16 string
        let len = MultiByteToWideChar(
            codepage,
            flags,
            multi_byte_str,
            None,
        );
        if len > 0 {
            // Convert to UTF-16
            let mut wstr: Vec<u16> = Vec::with_capacity(len as usize);
            wstr.set_len(len as usize);
            let len = MultiByteToWideChar(
                codepage,
                flags,
                multi_byte_str,
                Some(wstr.as_mut_slice()),
            );
            if len > 0 {
                return Ok(
                    Cow::Owned(
                        String::from_utf16_lossy(&wstr[0..(len as usize)])
                ));
            }
        }
        Err(Cow::Owned(String::from("convert failed")))
    }
}