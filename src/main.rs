// mod vosk;
// mod vosk_binding;

use portaudio_rs::device::DeviceInfo;
use portaudio_rs::stream::{Stream, StreamCallbackResult, StreamFlags, StreamParameters};
use std::collections::BTreeMap;
use vosk::{DecodingState, Model, Recognizer};

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

    //  ----------------------- main -------------------------------

    let devices = list_devices().expect("portaudio failed");

    let i = portaudio_rs::device::get_default_input_index().expect("no default input");
    println!("Using default input device {}", i);

    let info = devices.get(&i).expect("no device info");

    let mut last_partial = String::new();

    let input_par = StreamParameters {
        device: i,
        channel_count: 1,
        suggested_latency: info.default_low_input_latency,
        data: 42, // random
    };
    let stream = Stream::open(
        Some(input_par), // input channels
        None,            // output channels
        16000.0,         // sample rate
        portaudio_rs::stream::FRAMES_PER_BUFFER_UNSPECIFIED,
        StreamFlags::empty(),
        Some(Box::new(move |input, _out: &mut [i16], _time, _flags| {
            let status = recognizer.accept_waveform(input);
            let result = recognizer.partial_result();
            println!("{}", result.partial);

            match status {
                DecodingState::Running => {
                    let result = recognizer.partial_result();
                    // if result.partial != last_partial {
                    //     last_partial.clear();
                    //     last_partial.insert_str(0, &result.partial);
                    //     if !result.partial.is_empty() {
                    //         println!("{}", result.partial);
                    //     }
                    // }
                    // println!("{:?}", result.partial_result);
                }
                DecodingState::Finalized => {
                    let result = recognizer.final_result();
                    if let Some(multi) = result.multiple() {
                        let first = multi.alternatives.iter().next();
                        // println!("{:?}", first);
                    }

                    // let mut most_likely = None;
                    //
                    // while let Some(val) = multi.next() {
                    //     if max_val.as_ref().map_or(true, |m| &val > m) {
                    //         max_iter = self.clone();
                    //         max_val = Some(val);
                    //     }
                    // }

                    // println!("{:?}", result);
                }
                _ => {}
            }

            if status == DecodingState::Finalized {}
            StreamCallbackResult::Continue
        })),
    )
    .unwrap();
    stream.start().expect("failed to start the stream");
    std::thread::park();
}

fn list_devices() -> Result<BTreeMap<u32, DeviceInfo>, portaudio_rs::PaError> {
    portaudio_rs::initialize()?;
    let n = portaudio_rs::device::get_count()?;
    let inputs = (0..n)
        .into_iter()
        .filter_map(|index| {
            let info = portaudio_rs::device::get_info(index)?;
            if info.max_input_channels > 0 {
                Some((index, info))
            } else {
                None
            }
        })
        .collect::<BTreeMap<_, _>>();
    if inputs.is_empty() {
        println!("No input devices found.");
    } else {
        println!("Input devices:");
        for (index, info) in inputs.iter() {
            println!("Index={} Name={}", index, info.name);
        }
    }
    Ok(inputs)
}
