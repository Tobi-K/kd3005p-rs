use core::time::Duration;
use serialport::{DataBits, FlowControl, Parity, SerialPort, SerialPortInfo, StopBits};

// define the strings for the supported commands
pub const ISET_COMMAND: &str = "ISET1:";
pub const IGET_COMMAND: &str = "ISET1?";
pub const VSET_COMMAND: &str = "VSET1:";
pub const VGET_COMMAND: &str = "VSET1?";

pub const IOUT_COMMAND: &str = "IOUT1?";
pub const VOUT_COMMAND: &str = "VOUT1?";

pub const ON_COMMAND: &str = "OUT1";
pub const OFF_COMMAND: &str = "OUT0";

const ID_COMMAND: &str = "*IDN?";
const STATUS_COMMAND: &str = "STATUS?";

pub fn list_serial_ports() -> Vec<SerialPortInfo> {
    let ports = serialport::available_ports().expect("No ports found!"); // get available ports
    ports // return them
}

fn open_port(current_port: &String, my_output: &mut String) -> Result<Box<dyn SerialPort>, String> {
    let open_string = format!("Trying to open port {}! \n", &current_port);
    print!("{}", open_string); // print info
    my_output.push_str(&open_string);
    if cfg!(target_os = "linux") {
        // if on linux, check path
        if !std::path::Path::new(current_port).exists() {
            // path does not exist?
            return Err("Port path does not exist".to_string()); // return error
        }
    }
    // define the port
    let serial_port = serialport::new(current_port, 9_600).timeout(Duration::from_millis(100));

    match serial_port.open() {
        // try to open it
        Ok(opened_port) => {
            // it worked? Great, configure the port
            let mut port = opened_port;
            port.set_timeout(Duration::from_millis(100))
                .expect("Setting timeout failed!");
            port.set_data_bits(DataBits::Eight)
                .expect("Setting data bits failed!");
            port.set_parity(Parity::None)
                .expect("Setting parity failed!");
            port.set_stop_bits(StopBits::One)
                .expect("Setting stop bit failed!");
            port.set_flow_control(FlowControl::None)
                .expect("Setting flow control failed!");
            println!("Port {} opened successfully!", &current_port); // print info
            Ok(port) // return the opened and configured port
        }
        // if it did not work, return the error string
        Err(e) => Err(format!("Error: Failed to open port, error code was: {}", e)),
    }
}

pub fn get_set_amperage_voltage(
    current_port: &String,
    desired_command: &str,
    answer_string: &mut String,
    desired_setting: &String,
    my_output: &mut String,
) {
    let mut say_hello = String::new();
    match open_port(current_port, &mut say_hello) {
        // try to open the selected port
        Ok(returned_port) => {
            // it worked? continue with the command
            let mut port = returned_port; // fetch the port
            my_output.push_str(&say_hello);
            let mut my_command = String::new();
            match desired_command {
                IGET_COMMAND => {
                    say_hello = "Get output amperage! \n".to_string();
                    my_command = format!("{}", IGET_COMMAND);
                }
                VGET_COMMAND => {
                    say_hello = "Get output voltage! \n".to_string();
                    my_command = format!("{}", VGET_COMMAND);
                }
                ISET_COMMAND => {
                    say_hello = format!("Set amperage to {} A! \n", desired_setting);
                    my_command = format!("{}{}", ISET_COMMAND, desired_setting);
                }
                VSET_COMMAND => {
                    say_hello = format!("Set voltage to {} V! \n", desired_setting);
                    my_command = format!("{}{}", VSET_COMMAND, desired_setting);
                }
                IOUT_COMMAND => {
                    say_hello = format!("Get actual amperage! \n");
                    my_command = format!("{}", IOUT_COMMAND);
                }
                VOUT_COMMAND => {
                    say_hello = format!("Get actual voltage! \n");
                    my_command = format!("{}", VOUT_COMMAND);
                }
                ID_COMMAND => {
                    say_hello = "Send ID command! \n".to_string();
                    my_command = format!("{}", ID_COMMAND);
                }
                _ => {
                    say_hello = format!("Unknown command, something went wrong! \n");
                }
            };
            print!("{}", say_hello); // tell user what you do
            my_output.push_str(&say_hello);

            let mut transmit_output = String::new();
            let mut my_answer_string = String::new();
            transmit_serial(
                &mut port,
                &my_command,
                &mut my_answer_string,
                &mut transmit_output,
            ); // transmit the message
            my_output.push_str(&transmit_output);
            answer_string.push_str(&my_answer_string);
        }
        Err(e) => {
            // if it did not work
            println!("{}", e); // print the error...
            my_output.push_str(&e);
            return; // ... and return
        }
    };
}

pub fn get_id(current_port: &String, my_output: &mut String) {
    let mut say_hello = String::new();
    match open_port(current_port, &mut say_hello) {
        // try to open the selected port
        Ok(returned_port) => {
            // it worked? continue with the command
            let mut port = returned_port; // fetch the port
            my_output.push_str(&say_hello);

            say_hello = "Send ID command!\n".to_string();
            print!("{}", say_hello); // tell user what you do
            my_output.push_str(&say_hello);

            let mut transmit_output = String::new();
            let mut answer_string = String::new();
            transmit_serial(
                &mut port,
                ID_COMMAND,
                &mut answer_string,
                &mut transmit_output,
            ); // transmit the message
            my_output.push_str(&transmit_output);
        }
        Err(e) => {
            // if it did not work
            println!("{}", e); // print the error...
            my_output.push_str(&e);
            return; // ... and return
        }
    };
}

pub fn turn_on_off(
    current_port: &String,
    desired_command: &str,
    answer_string: &mut String,
    my_output: &mut String,
) {
    let mut say_hello = String::new();
    match open_port(current_port, &mut say_hello) {
        // try to open the selected port
        Ok(returned_port) => {
            // it worked? continue with the command
            let mut port = returned_port; // fetch the port
            my_output.push_str(&say_hello);
            let mut my_command = String::new();
            match desired_command {
                ON_COMMAND => {
                    say_hello = "Turn output ON! \n".to_string();
                    my_command = format!("{}", ON_COMMAND);
                }
                OFF_COMMAND => {
                    say_hello = "Turn output OFF! \n".to_string();
                    my_command = format!("{}", OFF_COMMAND);
                }
                _ => {
                    say_hello = format!("Unknown command, something went wrong! \n");
                }
            };
            print!("{}", say_hello); // tell user what you do
            my_output.push_str(&say_hello);

            let mut transmit_output = String::new();
            let mut my_answer_string = String::new();
            transmit_serial(
                &mut port,
                &my_command,
                &mut my_answer_string,
                &mut transmit_output,
            ); // transmit the message
            my_output.push_str(&transmit_output);
            answer_string.push_str(&my_answer_string);
        }
        Err(e) => {
            // if it did not work
            println!("{}", e); // print the error...
            my_output.push_str(&e);
            return; // ... and return
        }
    };
}

pub fn get_status(current_port: &String, my_output: &mut String) {
    let mut say_hello = String::new();
    match open_port(current_port, &mut say_hello) {
        // try to open the selected port
        Ok(returned_port) => {
            // it worked? continue with the command
            let mut port = returned_port; // fetch the port
            my_output.push_str(&say_hello);

            say_hello = "Send status command!\n".to_string();
            print!("{}", say_hello); // tell user what you do
            my_output.push_str(&say_hello);

            let mut transmit_output = String::new();
            let mut answer_string = String::new();
            transmit_serial(
                &mut port,
                STATUS_COMMAND,
                &mut answer_string,
                &mut transmit_output,
            ); // transmit the message
            my_output.push_str(&transmit_output);
        }
        Err(e) => {
            // if it did not work
            println!("{}", e); // print the error...
            return; // ... and return
        }
    };
}

fn transmit_serial(
    port: &mut Box<dyn SerialPort>,
    command: &str,
    answer: &mut String,
    my_output: &mut String,
) {
    let mut transmit_output = format!(">>  {:?} \n", command.as_bytes());
    print!("{}", transmit_output); // tell user what you want to do
    my_output.push_str(&transmit_output);
    let output = command.as_bytes(); // define data to write to serial interface
    port.write(output).expect("Write failed!"); // write it

    let mut serial_buf: Vec<u8> = vec![0; 32]; // define the receive buffer
    let mut result_vec: Vec<u8> = Vec::new(); // define the print buffer
    let mut length = 1;
    while length > 0 {
        // as long as data is received
        length = port.read(serial_buf.as_mut_slice()).unwrap_or(0); // read from port
        result_vec.extend_from_slice(&serial_buf[..length]); // add data to print buffer
                                                             // println!("{:?}", result_vec);
                                                             // println!("{}", std::str::from_utf8(&result_vec).unwrap());
    }
    transmit_output = format!("<<  {:?} \n", result_vec);
    print!("{}", transmit_output); // print the result
    my_output.push_str(&transmit_output);
    let answer_string = std::str::from_utf8(&result_vec).unwrap();
    transmit_output = format!(" <  {} \n\n", answer_string);
    print!("{}", transmit_output); // print it as ASCII
    my_output.push_str(&transmit_output);
    answer.push_str(&answer_string);
}
