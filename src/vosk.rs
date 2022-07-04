use crate::vosk_binding;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn create_model() -> Result<*mut vosk_binding::VoskModel, Box<dyn std::error::Error>> {
    // let vosk = Vosk::
    unsafe {
        let vosk = vosk_binding::Vosk::new("lib/vosk/libvosk.so")?;
        let model_path = CString::new("model").unwrap();
        let model = vosk.vosk_model_new(model_path.as_ptr());
        Ok(model)
    }
}

fn load_lib() -> Result<libloading::Library, Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./lib/vosk/libvosk.so")?;
        Ok(lib)
    }
}
