use abistr::CStrNonNull;

use bytemuck::Zeroable;

use firehazard::policy::*;

use hwnd0::NonNullHWND;

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::winerror::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;

use winresult::ErrorCode;

use core::ffi::*;



//#[allow(non_camel_case_types)]
//type DWORD_PTR  = usize;

include!("../functions/GetLastError.rs");
include!("../functions/MessageBoxA.rs");
include!("../functions/MessageBoxExA.rs");
include!("../functions/MessageBoxIndirectA.rs");
include!("../functions/SetLastError.rs");
include!("../structures/MSGBOXPARAMSA.rs");



fn main() {
    let caption         = unsafe { CStrNonNull::from_units_with_nul_unchecked(b"caption\0") };
    let text            = unsafe { CStrNonNull::from_units_with_nul_unchecked(b"text\0") };
    let invalid_hwnd    = NonNullHWND::try_from(0x12345678).unwrap();

    /// Set a breakpoint on `MBToWCSEx` to inspect encoding parameters.
    /// Assuming x64:
    ///
    /// | Reg   | Parameter         | Notes |
    /// | ------| ------------------| ------|
    /// | RCX   | CodePage          | 0 = CP_ACP, 1 = CP_OEMCP, 3 = CP_THREAD_CP
    /// | RDX   | lpMultiByteStr    |
    /// | R8    | cbMultiByte       |
    /// | R9    | lpUnicodeStr      |
    /// | ...   | ...
    ///
    /// Per:
    /// *   <https://web.archive.org/web/20140617053343/https://www.winehq.org/pipermail/wine-devel/2003-August/019177.html>
    /// *   <https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention>
    ///
    struct _Encoding;



    // These calls are all expected to succeed.

    // on Windows 10.0.19045.5011, these all seem to call `MBToWCSEx(CP_ACP, caption | text, ...)`
    assert_ne!(0, MessageBoxA(None, Some(text), Some(caption), MB_OK));
    assert_ne!(0, MessageBoxExA(None, Some(text), Some(caption), MB_OK, None));
    assert_ne!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, lpszText: Some(text), lpszCaption: Some(caption), dwStyle: MB_OK, .. Zeroable::zeroed() })});

    // null caption should default to "Error".  Source: https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa
    assert_ne!(0, MessageBoxA(None, Some(text), None, MB_OK));
    assert_ne!(0, MessageBoxExA(None, Some(text), None, MB_OK, None));
    assert_ne!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, lpszText: Some(text), lpszCaption: None, dwStyle: MB_OK, .. Zeroable::zeroed() })});

    // null text defaults to a blank body in practice / testing.
    assert_ne!(0, MessageBoxA(None, None, None, MB_OK));
    assert_ne!(0, MessageBoxExA(None, None, None, MB_OK, None));
    assert_ne!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, lpszText: None, lpszCaption: None, dwStyle: MB_OK, .. Zeroable::zeroed() })});

    // Subsequent calls are testing failure modes.



    // and an invalid kind?

    const MB_INVALID : u32 = !0;
    const ERROR_PLACEHOLDER : ErrorCode = ErrorCode::from_constant(!0);
    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxA(None, None, None, MB_INVALID));
    assert_eq!(ERROR_INVALID_MSGBOX_STYLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxExA(None, None, None, MB_INVALID, None));
    assert_eq!(ERROR_INVALID_MSGBOX_STYLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, lpszText: None, lpszCaption: None, dwStyle: MB_INVALID, .. Zeroable::zeroed() })});
    assert_eq!(ERROR_INVALID_MSGBOX_STYLE, GetLastError());



    // and invalid handles?

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxA(Some(invalid_hwnd), None, None, MB_OK));
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxExA(Some(invalid_hwnd), None, None, MB_OK, None));
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, hwndOwner: Some(invalid_hwnd), lpszText: None, lpszCaption: None, dwStyle: MB_OK, .. Zeroable::zeroed() })});
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());



    // and invalid handles after enabling strict handle checks?

    set_process_mitigation_policy({
        let mut policy = PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY::default();
        policy.set_HandleExceptionsPermanentlyEnabled(1);
        policy.set_RaiseExceptionOnInvalidHandleReference(1);
        policy
    }).expect("failed to set PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY");
    eprintln!("PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY set");

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxA(Some(invalid_hwnd), None, None, MB_OK));
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, MessageBoxExA(Some(invalid_hwnd), None, None, MB_OK, None));
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());

    SetLastError(ERROR_PLACEHOLDER.into());
    assert_eq!(0, unsafe { MessageBoxIndirectA(&MSGBOXPARAMSA { cbSize: size_of::<MSGBOXPARAMSA>() as _, hwndOwner: Some(invalid_hwnd), lpszText: None, lpszCaption: None, dwStyle: MB_OK, .. Zeroable::zeroed() })});
    assert_eq!(ERROR_INVALID_WINDOW_HANDLE, GetLastError());



    // did we finish despite `PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`?

    eprintln!("reached end of test except for one final message box");
    assert_ne!(0, MessageBoxA(None, None, None, MB_OK));
}
