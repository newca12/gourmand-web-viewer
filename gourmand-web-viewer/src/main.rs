mod gui;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main()  -> eframe::Result {
    gui::run_gui()
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    gui::run_gui_wasm();
}
