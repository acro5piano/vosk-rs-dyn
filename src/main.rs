// mod vosk;
// mod vosk_binding;

use vosk::{Model, Recognizer};

fn main() {
    // println!("{:?}", vosk::create_recognizer());

    println!("Hello, world!");

    let samples = vec![100, -2, 700, 30, 4, 5];
    let model_path = "model";

    let model = Model::new(model_path).unwrap();
    let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

    recognizer.set_max_alternatives(10);
    recognizer.set_words(true);
    recognizer.set_partial_words(true);

    for sample in samples.chunks(100) {
        recognizer.accept_waveform(sample);
        println!("{:#?}", recognizer.partial_result());
    }

    println!("{:#?}", recognizer.final_result().multiple().unwrap());
}
