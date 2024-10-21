unsafe extern "system" {
    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-setlasterror)\]
    /// Set `dwErrCode` for a later caller to query by <code>[GetLastError]\(\)</code>.
    ///
    ///
    ///
    /// ## Safe
    ///
    /// It's just a thread local storage write of a `u32`.  Should be perfectly harmless.
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
    safe fn SetLastError(dwErrCode: ::winresult::ErrorHResultOrCode);
}
