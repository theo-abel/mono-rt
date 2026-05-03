use mono_rt::MonoError;

#[derive(Default)]
pub struct Harness {
    passed: u32,
    failed: u32,
}

impl Harness {
    pub fn new() -> Self {
        Self::default()
    }

    /// Runs `f` and records the result, printing `[PASS]` or `[FAIL]` to stdout.
    pub fn run(&mut self, name: &str, f: impl FnOnce() -> Result<(), MonoError>) {
        match f() {
            Ok(()) => {
                println!("[PASS] {name}");
                self.passed += 1;
            }
            Err(e) => {
                println!("[FAIL] {name}: {e}");
                self.failed += 1;
            }
        }
    }

    /// Prints the final summary and returns `true` if every test passed.
    #[must_use]
    pub fn finish(self) -> bool {
        println!("--- {} passed, {} failed ---", self.passed, self.failed);
        self.failed == 0
    }
}
