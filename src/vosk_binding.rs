/* automatically generated by rust-bindgen 0.60.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskModel {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskSpkModel {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskRecognizer {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskBatchModel {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VoskBatchRecognizer {
    _unused: [u8; 0],
}
extern crate libloading;
pub struct Vosk {
    __library: ::libloading::Library,
    pub vosk_model_new:
        unsafe extern "C" fn(model_path: *const ::std::os::raw::c_char) -> *mut VoskModel,
    pub vosk_model_free: unsafe extern "C" fn(model: *mut VoskModel),
    pub vosk_model_find_word: unsafe extern "C" fn(
        model: *mut VoskModel,
        word: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
    pub vosk_spk_model_new:
        unsafe extern "C" fn(model_path: *const ::std::os::raw::c_char) -> *mut VoskSpkModel,
    pub vosk_spk_model_free: unsafe extern "C" fn(model: *mut VoskSpkModel),
    pub vosk_recognizer_new:
        unsafe extern "C" fn(model: *mut VoskModel, sample_rate: f32) -> *mut VoskRecognizer,
    pub vosk_recognizer_new_spk: unsafe extern "C" fn(
        model: *mut VoskModel,
        sample_rate: f32,
        spk_model: *mut VoskSpkModel,
    ) -> *mut VoskRecognizer,
    pub vosk_recognizer_new_grm: unsafe extern "C" fn(
        model: *mut VoskModel,
        sample_rate: f32,
        grammar: *const ::std::os::raw::c_char,
    ) -> *mut VoskRecognizer,
    pub vosk_recognizer_set_spk_model:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer, spk_model: *mut VoskSpkModel),
    pub vosk_recognizer_set_max_alternatives: unsafe extern "C" fn(
        recognizer: *mut VoskRecognizer,
        max_alternatives: ::std::os::raw::c_int,
    ),
    pub vosk_recognizer_set_words:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer, words: ::std::os::raw::c_int),
    pub vosk_recognizer_set_partial_words:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer, partial_words: ::std::os::raw::c_int),
    pub vosk_recognizer_set_nlsml:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer, nlsml: ::std::os::raw::c_int),
    pub vosk_recognizer_accept_waveform: unsafe extern "C" fn(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub vosk_recognizer_accept_waveform_s: unsafe extern "C" fn(
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_short,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub vosk_recognizer_accept_waveform_f: unsafe extern "C" fn(
        recognizer: *mut VoskRecognizer,
        data: *const f32,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
    pub vosk_recognizer_result:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer) -> *const ::std::os::raw::c_char,
    pub vosk_recognizer_partial_result:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer) -> *const ::std::os::raw::c_char,
    pub vosk_recognizer_final_result:
        unsafe extern "C" fn(recognizer: *mut VoskRecognizer) -> *const ::std::os::raw::c_char,
    pub vosk_recognizer_reset: unsafe extern "C" fn(recognizer: *mut VoskRecognizer),
    pub vosk_recognizer_free: unsafe extern "C" fn(recognizer: *mut VoskRecognizer),
    pub vosk_set_log_level: unsafe extern "C" fn(log_level: ::std::os::raw::c_int),
    pub vosk_gpu_init: unsafe extern "C" fn(),
    pub vosk_gpu_thread_init: unsafe extern "C" fn(),
    pub vosk_batch_model_new: unsafe extern "C" fn() -> *mut VoskBatchModel,
    pub vosk_batch_model_free: unsafe extern "C" fn(model: *mut VoskBatchModel),
    pub vosk_batch_model_wait: unsafe extern "C" fn(model: *mut VoskBatchModel),
    pub vosk_batch_recognizer_new: unsafe extern "C" fn(
        model: *mut VoskBatchModel,
        sample_rate: f32,
    ) -> *mut VoskBatchRecognizer,
    pub vosk_batch_recognizer_free: unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer),
    pub vosk_batch_recognizer_accept_waveform: unsafe extern "C" fn(
        recognizer: *mut VoskBatchRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ),
    pub vosk_batch_recognizer_set_nlsml:
        unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer, nlsml: ::std::os::raw::c_int),
    pub vosk_batch_recognizer_finish_stream:
        unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer),
    pub vosk_batch_recognizer_front_result:
        unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer) -> *const ::std::os::raw::c_char,
    pub vosk_batch_recognizer_pop: unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer),
    pub vosk_batch_recognizer_get_pending_chunks:
        unsafe extern "C" fn(recognizer: *mut VoskBatchRecognizer) -> ::std::os::raw::c_int,
}
impl Vosk {
    pub unsafe fn new<P>(path: P) -> Result<Self, ::libloading::Error>
    where
        P: AsRef<::std::ffi::OsStr>,
    {
        let library = ::libloading::Library::new(path)?;
        Self::from_library(library)
    }
    pub unsafe fn from_library<L>(library: L) -> Result<Self, ::libloading::Error>
    where
        L: Into<::libloading::Library>,
    {
        let __library = library.into();
        let vosk_model_new = __library.get(b"vosk_model_new\0").map(|sym| *sym)?;
        let vosk_model_free = __library.get(b"vosk_model_free\0").map(|sym| *sym)?;
        let vosk_model_find_word = __library.get(b"vosk_model_find_word\0").map(|sym| *sym)?;
        let vosk_spk_model_new = __library.get(b"vosk_spk_model_new\0").map(|sym| *sym)?;
        let vosk_spk_model_free = __library.get(b"vosk_spk_model_free\0").map(|sym| *sym)?;
        let vosk_recognizer_new = __library.get(b"vosk_recognizer_new\0").map(|sym| *sym)?;
        let vosk_recognizer_new_spk = __library
            .get(b"vosk_recognizer_new_spk\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_new_grm = __library
            .get(b"vosk_recognizer_new_grm\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_set_spk_model = __library
            .get(b"vosk_recognizer_set_spk_model\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_set_max_alternatives = __library
            .get(b"vosk_recognizer_set_max_alternatives\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_set_words = __library
            .get(b"vosk_recognizer_set_words\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_set_partial_words = __library
            .get(b"vosk_recognizer_set_partial_words\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_set_nlsml = __library
            .get(b"vosk_recognizer_set_nlsml\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_accept_waveform = __library
            .get(b"vosk_recognizer_accept_waveform\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_accept_waveform_s = __library
            .get(b"vosk_recognizer_accept_waveform_s\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_accept_waveform_f = __library
            .get(b"vosk_recognizer_accept_waveform_f\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_result = __library.get(b"vosk_recognizer_result\0").map(|sym| *sym)?;
        let vosk_recognizer_partial_result = __library
            .get(b"vosk_recognizer_partial_result\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_final_result = __library
            .get(b"vosk_recognizer_final_result\0")
            .map(|sym| *sym)?;
        let vosk_recognizer_reset = __library.get(b"vosk_recognizer_reset\0").map(|sym| *sym)?;
        let vosk_recognizer_free = __library.get(b"vosk_recognizer_free\0").map(|sym| *sym)?;
        let vosk_set_log_level = __library.get(b"vosk_set_log_level\0").map(|sym| *sym)?;
        let vosk_gpu_init = __library.get(b"vosk_gpu_init\0").map(|sym| *sym)?;
        let vosk_gpu_thread_init = __library.get(b"vosk_gpu_thread_init\0").map(|sym| *sym)?;
        let vosk_batch_model_new = __library.get(b"vosk_batch_model_new\0").map(|sym| *sym)?;
        let vosk_batch_model_free = __library.get(b"vosk_batch_model_free\0").map(|sym| *sym)?;
        let vosk_batch_model_wait = __library.get(b"vosk_batch_model_wait\0").map(|sym| *sym)?;
        let vosk_batch_recognizer_new = __library
            .get(b"vosk_batch_recognizer_new\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_free = __library
            .get(b"vosk_batch_recognizer_free\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_accept_waveform = __library
            .get(b"vosk_batch_recognizer_accept_waveform\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_set_nlsml = __library
            .get(b"vosk_batch_recognizer_set_nlsml\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_finish_stream = __library
            .get(b"vosk_batch_recognizer_finish_stream\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_front_result = __library
            .get(b"vosk_batch_recognizer_front_result\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_pop = __library
            .get(b"vosk_batch_recognizer_pop\0")
            .map(|sym| *sym)?;
        let vosk_batch_recognizer_get_pending_chunks = __library
            .get(b"vosk_batch_recognizer_get_pending_chunks\0")
            .map(|sym| *sym)?;
        Ok(Vosk {
            __library,
            vosk_model_new,
            vosk_model_free,
            vosk_model_find_word,
            vosk_spk_model_new,
            vosk_spk_model_free,
            vosk_recognizer_new,
            vosk_recognizer_new_spk,
            vosk_recognizer_new_grm,
            vosk_recognizer_set_spk_model,
            vosk_recognizer_set_max_alternatives,
            vosk_recognizer_set_words,
            vosk_recognizer_set_partial_words,
            vosk_recognizer_set_nlsml,
            vosk_recognizer_accept_waveform,
            vosk_recognizer_accept_waveform_s,
            vosk_recognizer_accept_waveform_f,
            vosk_recognizer_result,
            vosk_recognizer_partial_result,
            vosk_recognizer_final_result,
            vosk_recognizer_reset,
            vosk_recognizer_free,
            vosk_set_log_level,
            vosk_gpu_init,
            vosk_gpu_thread_init,
            vosk_batch_model_new,
            vosk_batch_model_free,
            vosk_batch_model_wait,
            vosk_batch_recognizer_new,
            vosk_batch_recognizer_free,
            vosk_batch_recognizer_accept_waveform,
            vosk_batch_recognizer_set_nlsml,
            vosk_batch_recognizer_finish_stream,
            vosk_batch_recognizer_front_result,
            vosk_batch_recognizer_pop,
            vosk_batch_recognizer_get_pending_chunks,
        })
    }
    #[doc = " Loads model data from the file and returns the model object"]
    #[doc = ""]
    #[doc = " @param model_path: the path of the model on the filesystem"]
    #[doc = " @returns model object or NULL if problem occured"]
    pub unsafe fn vosk_model_new(
        &self,
        model_path: *const ::std::os::raw::c_char,
    ) -> *mut VoskModel {
        (self.vosk_model_new)(model_path)
    }
    #[doc = " Releases the model memory"]
    #[doc = ""]
    #[doc = "  The model object is reference-counted so if some recognizer"]
    #[doc = "  depends on this model, model might still stay alive. When"]
    #[doc = "  last recognizer is released, model will be released too."]
    pub unsafe fn vosk_model_free(&self, model: *mut VoskModel) -> () {
        (self.vosk_model_free)(model)
    }
    #[doc = " Check if a word can be recognized by the model"]
    #[doc = " @param word: the word"]
    #[doc = " @returns the word symbol if @param word exists inside the model"]
    #[doc = " or -1 otherwise."]
    #[doc = " Reminding that word symbol 0 is for <epsilon>"]
    pub unsafe fn vosk_model_find_word(
        &self,
        model: *mut VoskModel,
        word: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int {
        (self.vosk_model_find_word)(model, word)
    }
    #[doc = " Loads speaker model data from the file and returns the model object"]
    #[doc = ""]
    #[doc = " @param model_path: the path of the model on the filesystem"]
    #[doc = " @returns model object or NULL if problem occured"]
    pub unsafe fn vosk_spk_model_new(
        &self,
        model_path: *const ::std::os::raw::c_char,
    ) -> *mut VoskSpkModel {
        (self.vosk_spk_model_new)(model_path)
    }
    #[doc = " Releases the model memory"]
    #[doc = ""]
    #[doc = "  The model object is reference-counted so if some recognizer"]
    #[doc = "  depends on this model, model might still stay alive. When"]
    #[doc = "  last recognizer is released, model will be released too."]
    pub unsafe fn vosk_spk_model_free(&self, model: *mut VoskSpkModel) -> () {
        (self.vosk_spk_model_free)(model)
    }
    #[doc = " Creates the recognizer object"]
    #[doc = ""]
    #[doc = "  The recognizers process the speech and return text using shared model data"]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub unsafe fn vosk_recognizer_new(
        &self,
        model: *mut VoskModel,
        sample_rate: f32,
    ) -> *mut VoskRecognizer {
        (self.vosk_recognizer_new)(model, sample_rate)
    }
    #[doc = " Creates the recognizer object with speaker recognition"]
    #[doc = ""]
    #[doc = "  With the speaker recognition mode the recognizer not just recognize"]
    #[doc = "  text but also return speaker vectors one can use for speaker identification"]
    #[doc = ""]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @param spk_model speaker model for speaker identification"]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub unsafe fn vosk_recognizer_new_spk(
        &self,
        model: *mut VoskModel,
        sample_rate: f32,
        spk_model: *mut VoskSpkModel,
    ) -> *mut VoskRecognizer {
        (self.vosk_recognizer_new_spk)(model, sample_rate, spk_model)
    }
    #[doc = " Creates the recognizer object with the phrase list"]
    #[doc = ""]
    #[doc = "  Sometimes when you want to improve recognition accuracy and when you don't need"]
    #[doc = "  to recognize large vocabulary you can specify a list of phrases to recognize. This"]
    #[doc = "  will improve recognizer speed and accuracy but might return [unk] if user said"]
    #[doc = "  something different."]
    #[doc = ""]
    #[doc = "  Only recognizers with lookahead models support this type of quick configuration."]
    #[doc = "  Precompiled HCLG graph models are not supported."]
    #[doc = ""]
    #[doc = "  @param model       VoskModel containing static data for recognizer. Model can be"]
    #[doc = "                     shared across recognizers, even running in different threads."]
    #[doc = "  @param sample_rate The sample rate of the audio you going to feed into the recognizer."]
    #[doc = "                     Make sure this rate matches the audio content, it is a common"]
    #[doc = "                     issue causing accuracy problems."]
    #[doc = "  @param grammar The string with the list of phrases to recognize as JSON array of strings,"]
    #[doc = "                 for example \"[\"one two three four five\", \"[unk]\"]\"."]
    #[doc = ""]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub unsafe fn vosk_recognizer_new_grm(
        &self,
        model: *mut VoskModel,
        sample_rate: f32,
        grammar: *const ::std::os::raw::c_char,
    ) -> *mut VoskRecognizer {
        (self.vosk_recognizer_new_grm)(model, sample_rate, grammar)
    }
    #[doc = " Adds speaker model to already initialized recognizer"]
    #[doc = ""]
    #[doc = " Can add speaker recognition model to already created recognizer. Helps to initialize"]
    #[doc = " speaker recognition for grammar-based recognizer."]
    #[doc = ""]
    #[doc = " @param spk_model Speaker recognition model"]
    pub unsafe fn vosk_recognizer_set_spk_model(
        &self,
        recognizer: *mut VoskRecognizer,
        spk_model: *mut VoskSpkModel,
    ) -> () {
        (self.vosk_recognizer_set_spk_model)(recognizer, spk_model)
    }
    #[doc = " Configures recognizer to output n-best results"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "   {"]
    #[doc = "      \"alternatives\": ["]
    #[doc = "          { \"text\": \"one two three four five\", \"confidence\": 0.97 },"]
    #[doc = "          { \"text\": \"one two three for five\", \"confidence\": 0.03 },"]
    #[doc = "      ]"]
    #[doc = "   }"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " @param max_alternatives - maximum alternatives to return from recognition results"]
    pub unsafe fn vosk_recognizer_set_max_alternatives(
        &self,
        recognizer: *mut VoskRecognizer,
        max_alternatives: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_recognizer_set_max_alternatives)(recognizer, max_alternatives)
    }
    #[doc = " Enables words with times in the output"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "   \"result\" : [{"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.110000,"]
    #[doc = "       \"start\" : 0.870000,"]
    #[doc = "       \"word\" : \"what\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.530000,"]
    #[doc = "       \"start\" : 1.110000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 1.950000,"]
    #[doc = "       \"start\" : 1.530000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 2.340000,"]
    #[doc = "       \"start\" : 1.950000,"]
    #[doc = "       \"word\" : \"zero\""]
    #[doc = "     }, {"]
    #[doc = "       \"conf\" : 1.000000,"]
    #[doc = "       \"end\" : 2.610000,"]
    #[doc = "       \"start\" : 2.340000,"]
    #[doc = "       \"word\" : \"one\""]
    #[doc = "     }],"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " @param words - boolean value"]
    pub unsafe fn vosk_recognizer_set_words(
        &self,
        recognizer: *mut VoskRecognizer,
        words: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_recognizer_set_words)(recognizer, words)
    }
    #[doc = " Like above return words and confidences in partial results"]
    #[doc = ""]
    #[doc = " @param partial_words - boolean value"]
    pub unsafe fn vosk_recognizer_set_partial_words(
        &self,
        recognizer: *mut VoskRecognizer,
        partial_words: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_recognizer_set_partial_words)(recognizer, partial_words)
    }
    #[doc = " Set NLSML output"]
    #[doc = " @param nlsml - boolean value"]
    pub unsafe fn vosk_recognizer_set_nlsml(
        &self,
        recognizer: *mut VoskRecognizer,
        nlsml: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_recognizer_set_nlsml)(recognizer, nlsml)
    }
    #[doc = " Accept voice data"]
    #[doc = ""]
    #[doc = "  accept and process new chunk of voice data"]
    #[doc = ""]
    #[doc = "  @param data - audio data in PCM 16-bit mono format"]
    #[doc = "  @param length - length of the audio data"]
    #[doc = "  @returns 1 if silence is occured and you can retrieve a new utterance with result method"]
    #[doc = "           0 if decoding continues"]
    #[doc = "           -1 if exception occured"]
    pub unsafe fn vosk_recognizer_accept_waveform(
        &self,
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self.vosk_recognizer_accept_waveform)(recognizer, data, length)
    }
    #[doc = " Same as above but the version with the short data for language bindings where you have"]
    #[doc = "  audio as array of shorts"]
    pub unsafe fn vosk_recognizer_accept_waveform_s(
        &self,
        recognizer: *mut VoskRecognizer,
        data: *const ::std::os::raw::c_short,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self.vosk_recognizer_accept_waveform_s)(recognizer, data, length)
    }
    #[doc = " Same as above but the version with the float data for language bindings where you have"]
    #[doc = "  audio as array of floats"]
    pub unsafe fn vosk_recognizer_accept_waveform_f(
        &self,
        recognizer: *mut VoskRecognizer,
        data: *const f32,
        length: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int {
        (self.vosk_recognizer_accept_waveform_f)(recognizer, data, length)
    }
    #[doc = " Returns speech recognition result"]
    #[doc = ""]
    #[doc = " @returns the result in JSON format which contains decoded line, decoded"]
    #[doc = "          words, times in seconds and confidences. You can parse this result"]
    #[doc = "          with any json parser"]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = "  {"]
    #[doc = "    \"text\" : \"what zero zero zero one\""]
    #[doc = "  }"]
    #[doc = " </pre>"]
    #[doc = ""]
    #[doc = " If alternatives enabled it returns result with alternatives, see also vosk_recognizer_set_alternatives()."]
    #[doc = ""]
    #[doc = " If word times enabled returns word time, see also vosk_recognizer_set_word_times()."]
    pub unsafe fn vosk_recognizer_result(
        &self,
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char {
        (self.vosk_recognizer_result)(recognizer)
    }
    #[doc = " Returns partial speech recognition"]
    #[doc = ""]
    #[doc = " @returns partial speech recognition text which is not yet finalized."]
    #[doc = "          result may change as recognizer process more data."]
    #[doc = ""]
    #[doc = " <pre>"]
    #[doc = " {"]
    #[doc = "    \"partial\" : \"cyril one eight zero\""]
    #[doc = " }"]
    #[doc = " </pre>"]
    pub unsafe fn vosk_recognizer_partial_result(
        &self,
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char {
        (self.vosk_recognizer_partial_result)(recognizer)
    }
    #[doc = " Returns speech recognition result. Same as result, but doesn't wait for silence"]
    #[doc = "  You usually call it in the end of the stream to get final bits of audio. It"]
    #[doc = "  flushes the feature pipeline, so all remaining audio chunks got processed."]
    #[doc = ""]
    #[doc = "  @returns speech result in JSON format."]
    pub unsafe fn vosk_recognizer_final_result(
        &self,
        recognizer: *mut VoskRecognizer,
    ) -> *const ::std::os::raw::c_char {
        (self.vosk_recognizer_final_result)(recognizer)
    }
    #[doc = " Resets the recognizer"]
    #[doc = ""]
    #[doc = "  Resets current results so the recognition can continue from scratch"]
    pub unsafe fn vosk_recognizer_reset(&self, recognizer: *mut VoskRecognizer) -> () {
        (self.vosk_recognizer_reset)(recognizer)
    }
    #[doc = " Releases recognizer object"]
    #[doc = ""]
    #[doc = "  Underlying model is also unreferenced and if needed released"]
    pub unsafe fn vosk_recognizer_free(&self, recognizer: *mut VoskRecognizer) -> () {
        (self.vosk_recognizer_free)(recognizer)
    }
    #[doc = " Set log level for Kaldi messages"]
    #[doc = ""]
    #[doc = "  @param log_level the level"]
    #[doc = "     0 - default value to print info and error messages but no debug"]
    #[doc = "     less than 0 - don't print info messages"]
    #[doc = "     greather than 0 - more verbose mode"]
    pub unsafe fn vosk_set_log_level(&self, log_level: ::std::os::raw::c_int) -> () {
        (self.vosk_set_log_level)(log_level)
    }
    #[doc = "  Init, automatically select a CUDA device and allow multithreading."]
    #[doc = "  Must be called once from the main thread."]
    #[doc = "  Has no effect if HAVE_CUDA flag is not set."]
    pub unsafe fn vosk_gpu_init(&self) -> () {
        (self.vosk_gpu_init)()
    }
    #[doc = "  Init CUDA device in a multi-threaded environment."]
    #[doc = "  Must be called for each thread."]
    #[doc = "  Has no effect if HAVE_CUDA flag is not set."]
    pub unsafe fn vosk_gpu_thread_init(&self) -> () {
        (self.vosk_gpu_thread_init)()
    }
    #[doc = " Creates the batch recognizer object"]
    #[doc = ""]
    #[doc = "  @returns model object or NULL if problem occured"]
    pub unsafe fn vosk_batch_model_new(&self) -> *mut VoskBatchModel {
        (self.vosk_batch_model_new)()
    }
    #[doc = " Releases batch model object"]
    pub unsafe fn vosk_batch_model_free(&self, model: *mut VoskBatchModel) -> () {
        (self.vosk_batch_model_free)(model)
    }
    #[doc = " Wait for the processing"]
    pub unsafe fn vosk_batch_model_wait(&self, model: *mut VoskBatchModel) -> () {
        (self.vosk_batch_model_wait)(model)
    }
    #[doc = " Creates batch recognizer object"]
    #[doc = "  @returns recognizer object or NULL if problem occured"]
    pub unsafe fn vosk_batch_recognizer_new(
        &self,
        model: *mut VoskBatchModel,
        sample_rate: f32,
    ) -> *mut VoskBatchRecognizer {
        (self.vosk_batch_recognizer_new)(model, sample_rate)
    }
    #[doc = " Releases batch recognizer object"]
    pub unsafe fn vosk_batch_recognizer_free(&self, recognizer: *mut VoskBatchRecognizer) -> () {
        (self.vosk_batch_recognizer_free)(recognizer)
    }
    #[doc = " Accept batch voice data"]
    pub unsafe fn vosk_batch_recognizer_accept_waveform(
        &self,
        recognizer: *mut VoskBatchRecognizer,
        data: *const ::std::os::raw::c_char,
        length: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_batch_recognizer_accept_waveform)(recognizer, data, length)
    }
    #[doc = " Set NLSML output"]
    #[doc = " @param nlsml - boolean value"]
    pub unsafe fn vosk_batch_recognizer_set_nlsml(
        &self,
        recognizer: *mut VoskBatchRecognizer,
        nlsml: ::std::os::raw::c_int,
    ) -> () {
        (self.vosk_batch_recognizer_set_nlsml)(recognizer, nlsml)
    }
    #[doc = " Closes the stream"]
    pub unsafe fn vosk_batch_recognizer_finish_stream(
        &self,
        recognizer: *mut VoskBatchRecognizer,
    ) -> () {
        (self.vosk_batch_recognizer_finish_stream)(recognizer)
    }
    #[doc = " Return results"]
    pub unsafe fn vosk_batch_recognizer_front_result(
        &self,
        recognizer: *mut VoskBatchRecognizer,
    ) -> *const ::std::os::raw::c_char {
        (self.vosk_batch_recognizer_front_result)(recognizer)
    }
    #[doc = " Release and free first retrieved result"]
    pub unsafe fn vosk_batch_recognizer_pop(&self, recognizer: *mut VoskBatchRecognizer) -> () {
        (self.vosk_batch_recognizer_pop)(recognizer)
    }
    #[doc = " Get amount of pending chunks for more intelligent waiting"]
    pub unsafe fn vosk_batch_recognizer_get_pending_chunks(
        &self,
        recognizer: *mut VoskBatchRecognizer,
    ) -> ::std::os::raw::c_int {
        (self.vosk_batch_recognizer_get_pending_chunks)(recognizer)
    }
}
