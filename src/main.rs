use druid::{AppLauncher, PlatformError, WindowDesc};
use serialport::SerialPortInfo;

mod data;
mod gui;
mod serial;
use data::TheAppState;

static WINDOW_WIDTH: f64 = 450.0;
static WINDOW_HEIGHT: f64 = 450.0;

fn main() -> Result<(), PlatformError> {
    // Initialize the AppState
    let my_app_state = TheAppState {
        current_port: "None selected".to_string(),
        port_open: false,
        current_voltage: "12.00".to_string(),
        current_amperage: "1.000".to_string(),
        output_info: "Welcome to KD3005P-rs! \n".to_string(),
        the_ports: Vec::<SerialPortInfo>::new(),
    };

    // Window builder. We set title and size
    let main_window = WindowDesc::new(gui::ui_builder)
        .title("Korad KD3005P-rs")
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

    // Run the app
    AppLauncher::with_window(main_window)
        .use_simple_logger() // Neat!
        .launch(my_app_state)
}
