fn main() {
    println!("{:?}", call_dynamic().unwrap());

    println!("Hello, world!");
}

fn call_dynamic() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib = libloading::Library::new("./my-dylib/lib.so")?;
        let func: libloading::Symbol<unsafe extern "C" fn(x: f32) -> f32> = lib.get(b"add10")?;
        println!("{:?}", func(20.0).to_string());
        Ok(())
        // Ok(func(10.0))
    }
}
