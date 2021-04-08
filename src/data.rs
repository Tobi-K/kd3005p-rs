use druid::{Data, Lens};
use serialport::SerialPortInfo;

#[derive(Clone, Data, Lens)]
pub struct TheAppState {
    pub current_port: String,
    pub port_open: bool,
    pub current_voltage: String,
    pub current_amperage: String,
    pub output_info: String,
    #[data(ignore)]
    pub the_ports: Vec<SerialPortInfo>,
}
