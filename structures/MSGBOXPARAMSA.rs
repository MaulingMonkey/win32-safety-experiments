#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
#[allow(non_snake_case)]
#[repr(C)] pub struct MSGBOXPARAMSA<'a> {
    /// XXX: safety hazard if cbSize > size_of::<Self>()
    pub cbSize:             UINT,
    pub hwndOwner:          Option<::hwnd0::NonNullHWND>,
    pub hInstance:          HINSTANCE,
    pub lpszText:           Option<abistr::CStrNonNull<'a, abistr::encoding::windows::System>>,
    pub lpszCaption:        Option<abistr::CStrNonNull<'a, abistr::encoding::windows::System>>,
    pub dwStyle:            DWORD,
    pub lpszIcon:           LPCSTR,
    pub dwContextHelpId:    DWORD_PTR,
    /// XXX: MSGBOXCALLBACK is `unsafe` and will be called with utterly unclear preconditions... possibly a truncated `HELPINFO`?
    pub lpfnMsgBoxCallback: MSGBOXCALLBACK,
    pub dwLanguageId:       DWORD,
}

unsafe impl<'a> bytemuck::Zeroable for MSGBOXPARAMSA<'a> {}
