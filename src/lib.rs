pub use winapi::um::winuser::MB_OK;

use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;

use core::ffi::*;



//#[allow(non_camel_case_types)]
//type DWORD_PTR  = usize;

include!("../functions/GetLastError.rs");
include!("../functions/MessageBoxA.rs");
include!("../functions/MessageBoxExA.rs");
include!("../functions/MessageBoxIndirectA.rs");
include!("../functions/SetLastError.rs");
include!("../structures/MSGBOXPARAMSA.rs");
