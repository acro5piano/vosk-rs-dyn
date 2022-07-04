use crate::vosk_binding;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn create_recognizer() -> Result<*mut vosk_binding::VoskRecognizer, Box<dyn std::error::Error>>
{
    // let vosk = Vosk::
    unsafe {
        let vosk = vosk_binding::Vosk::new("lib/vosk/libvosk.so")?;
        let model_path = CString::new("model").unwrap();
        let model = vosk.vosk_model_new(model_path.as_ptr());
        let recognizer = vosk.vosk_recognizer_new(model, 16000.0);
        Ok(recognizer)
    }
}
