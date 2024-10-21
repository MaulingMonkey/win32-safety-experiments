unsafe extern "system" {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror)\]
    /// Read the last `dwErrCode` set by <code>[SetLastError]{,[Ex](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setlasterrorex)}\(dwErrorCode\)</code> on this thread.
    ///
    /// Many - but not all - Windows APIs use this to provide additional details about what kind of error was encountered after returning zero, non-zero, or null.
    /// In some cases, success and failure are indistinguishable without calling <code>[SetLastError]\(0\)</code> before the potentially failing code.
    ///
    ///
    ///
    /// ## Safe
    ///
    /// It's just a thread local storage read of a `u32`.  Should be perfectly harmless.
    ///
    ///
    ///
    /// ## Requirements
    ///
    /// | Requirement   | Value |
    /// | --------------| ------|
    /// | Windows       | `3.1`? `95`?
    /// | Header        | `<windows.h>` (→ `<errhandlingapi.h>`)
    /// | DLL           | `kernel32.dll`
    ///
    safe fn GetLastError() -> ::winresult::ErrorHResultOrCode;
}
