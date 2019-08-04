use std::fmt;

#[repr(transparent)]
pub struct Symbol(*mut puredata_sys::t_symbol);

impl Symbol {}

impl std::convert::From<std::ffi::CString> for Symbol {
    fn from(s: std::ffi::CString) -> Self {
        unsafe { Self(puredata_sys::gensym(s.as_ptr())) }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe {
            let c = std::ffi::CStr::from_ptr((*self.0).s_name);
            if let Ok(c) = c.to_str() {
                write!(f, "{}", c)
            } else {
                Err(std::fmt::Error {})
            }
        }
    }
}

impl std::cmp::PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        let s = self.0 as *const _;
        let o = other.0 as *const _;
        s == o
    }
}

impl std::cmp::Eq for Symbol {}

impl Copy for Symbol {}
impl Clone for Symbol {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
