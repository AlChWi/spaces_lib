fn hello_from_rust() -> TestStruct {
    TestStruct::new("Hello from Rust!")
}

pub struct TestStruct {
    pub text: String,
}

impl TestStruct {
    pub fn new(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}

uniffi::include_scaffolding!("Spaces");
