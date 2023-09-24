use std::ffi::CString;

use crate::pointer::Pointer;

pub trait GfxBind: Pointer {
    fn uniform_location(&self, name: &dyn ToString, program: u32) -> Result<i32, std::ffi::NulError> {
        unsafe {
            let name = CString::new(name.to_string())?;
            Ok(gl::GetUniformLocation(
                program,
                name.as_ptr() as *const _,
            ))
        }
    }
    fn bind(&self, name: &dyn ToString, program: u32) -> Result<(), std::ffi::NulError>;
}