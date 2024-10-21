unsafe extern "system" {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa)\]
    /// `MessageBoxA` &mdash; Display a message dialog box using the system locale's narrow codepage.
    ///
    ///
    ///
    /// ## ⚠️ Prefer `MessageBoxW` ⚠️
    ///
    /// Windows 10.0.19045.5011 assumes `lpText` and `lpCaption` use the system locale, and simply converts these strings to UTF-16 via <code>MBToWCSEx(CP_ACP, ...)</code> before invoking `MessageBox...W`.
    /// This assumption is likely mistaken, and will result in [Mojibake](https://en.wikipedia.org/wiki/Mojibake) if you feed it ASCII-based strings on a Japanese Shift-JIS system, replacing e.g. `|` with `¥` (`0x5C`), and `~` with `‾` (`0x7E`).
    /// Microsoft [recommends](https://learn.microsoft.com/en-us/windows/win32/intl/unicode) using the codepage-agnostic [UTF-16]ish `wchar_t`-based `*W` APIs that work directly with Windows&nbsp;NT's native internal encoding.
    ///
    ///
    ///
    /// ## Safe
    ///
    /// Parameters are well validated.
    /// The only possible source of undefined behavior would be failing to `\0`-terminate your strings, which this binding avoids via <code>[abistr]::[NonNull]</code>.
    ///
    ///
    ///
    /// ## Parameters
    ///
    /// -   `hWnd` &mdash;
    ///     Optional.
    ///     If a valid window is specified, the message box will become modal &mdash; it will stay atop the specified window, stealing focus and blocking (non-programatic) access to said window until the message box is dismissed.
    ///     If an invalid window is specified, the function will return `0` and <code>[GetLastError]\(\)</code> will return `ERROR_INVALID_WINDOW_HANDLE`.
    ///     [`PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_strict_handle_check_policy) does not affect this behavior.
    ///
    /// -   `lpText` &mdash;
    ///     Body text of the message box.
    ///     Defaults to being blank string if [`None`].
    ///     May contain newlines (`b'\n`').
    ///
    /// -   `lpCaption` &mdash;
    ///     Optional.
    ///     Caption / title bar of the message box.
    ///     Defaults to `Error` if null.
    ///
    /// -   `uType` &mdash;
    ///     The message box buttons and icons to show.
    ///     If an invalid type is specified, the function will return `0` and <code>[GetLastError]\(\)</code> will return `ERROR_INVALID_MSGBOX_STYLE`.
    ///
    ///
    ///
    /// ## Returns
    ///
    /// `0` on error, with <code>[GetLastError]\(\)</code>:
    /// -   `ERROR_INVALID_WINDOW_HANDLE` (bad `hWnd`)
    /// -   `ERROR_INVALID_MSGBOX_STYLE` (bad `uType`)
    /// -   `???`
    ///
    /// Nonzero on success:
    ///
    /// | Value | Constant      | Condition |
    /// | -----:| --------------| ----------|
    /// | 1     | `IDOK`        | **OK** was selected
    /// | 2     | `IDCANCEL`    | **Cancel** was selected
    /// | 3     | `IDABORT`     | **Abort** was selected
    /// | 4     | `IDRETRY`     | **Retry** was selected
    /// | 5     | `IDIGNORE`    | **Ignore** was selected
    /// | 6     | `IDYES`       | **Yes** was selected
    /// | 7     | `IDNO`        | **No** was selected
    /// |
    /// |       |               | Windows NT 4.0 / `#if(WINVER >= 0x0400)`
    /// | 8     | `IDCLOSE`     | (callback only?)
    /// | 9     | `IDHELP`      | (callback only?)
    /// |
    /// |       |               | Windows 2000 / `#if(WINVER >= 0x0500)`
    /// | 10    | `IDTRYAGAIN`  | **Try Again** was selected
    /// | 11    | `IDCONTINUE`  | **Continue** was selected
    /// |
    /// |       |               | Windows XP / `#if(WINVER >= 0x0501)`
    /// | 32000 | `IDTIMEOUT`   | If the undocumented `MessageBoxTimeout` variant of this function times out?
    ///
    ///
    /// The exact behavior of closing a message box via `ESC` (keyboard) or clicking `X` (to close the dialog via mouse) varies by `uType`:
    ///
    /// | `uType`       | `Enter` with default focus    | `ESC` / `X` Behavior  |
    /// | --------------| ------------------------------| ----------------------|
    /// | `MB_OK`       | `IDOK` (1)                    | `IDOK` (1)            |
    /// | `MB_OKCANCEL` | `IDOK` (1)                    | `IDCANCEL` (2)        |
    /// | ...           | ...                           | ...                   |
    ///
    ///
    ///
    /// ## Other Error Conditions
    ///
    /// Disclaimer:  This list is likely incomplete.
    ///
    /// If the process does not have access to the current
    /// [Window Station](https://learn.microsoft.com/en-us/windows/win32/winstation/window-stations) or
    /// [Desktop](https://learn.microsoft.com/en-us/windows/win32/winstation/desktops),
    /// the process may crash when loading `user32.dll`.
    ///
    /// If GDI ran out of handles... who knows.
    ///
    /// If the process runs out of memory... who knows.
    ///
    ///
    ///
    /// ## Requirements
    ///
    /// | Requirement   | Value |
    /// | --------------| ------|
    /// | Windows       | `3.1`? `95`?
    /// | Header        | `<windows.h>` (→ `<winuser.h>`)
    /// | DLL           | `user32.dll`
    ///
    ///
    ///
    /// <!-- References -->
    ///
    /// [UTF-16]:               https://en.wikipedia.org/wiki/UTF-16
    /// [Mojibake]:             https://en.wikipedia.org/wiki/Mojibake
    ///
    safe fn MessageBoxA(
        hWnd:       Option<::hwnd0::NonNullHWND>,
        lpText:     Option<::abistr::CStrNonNull<::abistr::encoding::windows::System>>,
        lpCaption:  Option<::abistr::CStrNonNull<::abistr::encoding::windows::System>>,
        uType:      UINT,
    ) -> c_int;
}
