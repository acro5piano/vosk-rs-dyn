mod vosk;
mod vosk_binding;

fn main() {
    println!("{:?}", vosk::create_model());

    println!("Hello, world!");
}
