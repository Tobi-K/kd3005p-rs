use druid::widget::{Button, Container, Flex, Label, Scroll, TextBox};
use druid::{Color, FontDescriptor, FontFamily, Widget, WidgetExt};

use crate::serial::*;

use crate::data::*;

pub fn ui_builder() -> impl Widget<TheAppState> {
    // define the port selection colum:
    let current_port_text = TextBox::new().lens(TheAppState::current_port).padding(5.0); // text field that shows current port
    let mut select_col = Flex::column() // column to hold all the fields
        .with_child(current_port_text); // add the text field

    let initial_serial_vector = list_serial_ports(); // get all serial ports
    for i in initial_serial_vector {
        // add a button for each found port
        let button = Button::new(format!("{}", i.port_name))
            .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
                my_app_state.current_port = i.port_name.to_string();
                println!("{}", &my_app_state.current_port);
            })
            .padding(5.0);
        select_col.add_child(button);
    }

    // define button to fetch ID information
    let id_button = Button::new("KD3005P ID".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            get_id(&my_app_state.current_port, &mut my_output);
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    // define button to fetch status information
    let status_button = Button::new("KD3005P status".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            get_status(&my_app_state.current_port, &mut my_output);
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    // define the box for voltage setting
    let voltage_label = Label::new("Voltage in [V]".to_string()).padding(5.0); // label
    let voltage_text = TextBox::new()
        .lens(TheAppState::current_voltage)
        .padding(5.0); // text field

    let set_voltage_button = Button::new("Set Voltage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                VSET_COMMAND,
                &mut answer_string,
                &my_app_state.current_voltage,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let get_voltage_button = Button::new("Get Voltage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            let dummy_value = "".to_string();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                VGET_COMMAND,
                &mut answer_string,
                &dummy_value,
                &mut my_output,
            );
            if !answer_string.is_empty() {
                my_app_state.current_voltage = answer_string;
            }
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let get_actual_voltage_button = Button::new("Actual Voltage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            let dummy_value = "".to_string();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                VOUT_COMMAND,
                &mut answer_string,
                &dummy_value,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            if !answer_string.is_empty() {
                my_app_state
                    .output_info
                    .push_str(&(format!("The actual output voltage is {} V!", answer_string)));
            }
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let voltage_col = Flex::column() //create a column for it and add it all
        .with_child(voltage_label)
        .with_child(voltage_text)
        .with_child(set_voltage_button)
        .with_child(get_voltage_button)
        .with_child(get_actual_voltage_button);

    // define the box for current setting
    let amperage_label = Label::new("Amperage in [A]".to_string()).padding(5.0); // label
    let amperage_text = TextBox::new()
        .lens(TheAppState::current_amperage)
        .padding(5.0); // text field
    let set_amperage_button = Button::new("Set Amperage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                ISET_COMMAND,
                &mut answer_string,
                &my_app_state.current_amperage,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let get_amperage_button = Button::new("Get Amperage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            let dummy_value = "".to_string();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                IGET_COMMAND,
                &mut answer_string,
                &dummy_value,
                &mut my_output,
            );
            if !answer_string.is_empty() {
                my_app_state.current_amperage = answer_string;
            }
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let get_actual_amperage_button = Button::new("Actual Amperage".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            let dummy_value = "".to_string();
            get_set_amperage_voltage(
                &my_app_state.current_port,
                IOUT_COMMAND,
                &mut answer_string,
                &dummy_value,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            if !answer_string.is_empty() {
                my_app_state
                    .output_info
                    .push_str(&(format!("The actual output amperage is {} A!", answer_string)));
            }
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let on_button = Button::new("Output ON".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            turn_on_off(
                &my_app_state.current_port,
                ON_COMMAND,
                &mut answer_string,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let off_button = Button::new("Output OFF".to_string())
        .on_click(move |_ctx, my_app_state: &mut TheAppState, _env| {
            let mut my_output = String::new();
            let mut answer_string = String::new();
            turn_on_off(
                &my_app_state.current_port,
                OFF_COMMAND,
                &mut answer_string,
                &mut my_output,
            );
            my_app_state.output_info.clear();
            my_app_state.output_info.push_str(&my_output);
        })
        .padding(5.0); //button

    let amperage_col = Flex::column() //create a column for it and add it alle
        .with_child(amperage_label)
        .with_child(amperage_text)
        .with_child(set_amperage_button)
        .with_child(get_amperage_button)
        .with_child(get_actual_amperage_button);

    let status_row = Flex::row() // define a row for the status buttons
        .with_child(id_button)
        .with_child(status_button);

    // define the container row to hold voltage and current setting
    let voltage_current_row = Flex::row()
        .with_child(
            Container::new(voltage_col)
                .border(Color::grey8(0x55), 2.0)
                .padding(2.0),
        )
        .with_child(
            Container::new(amperage_col)
                .border(Color::grey8(0x55), 2.0)
                .padding(2.0),
        );

    let on_off_row = Flex::row() // define a row for the status buttons
        .with_child(on_button)
        .with_child(off_button);

    // define the container column for ALL settings
    let settings_col = Flex::column()
        .with_child(status_row)
        .with_child(voltage_current_row)
        .with_child(on_off_row)
        .padding(5.0);

    let info_label = Label::raw() // label
        .with_font(FontDescriptor::new(FontFamily::MONOSPACE)) // font for label
        .lens(TheAppState::output_info)
        .padding(5.0);

    // the row containing all the serial stuff
    let mut serial_row = Flex::row();
    // add it all to the main container row, with borders and padding

    serial_row.add_child(
        Container::new(select_col)
            .border(Color::grey8(0x55), 2.0)
            .padding(5.0),
    );
    serial_row.add_child(
        Container::new(settings_col)
            .border(Color::grey8(0x55), 2.0)
            .padding(5.0),
    );

    let info_scroll = Scroll::new(Container::new(info_label));
    let main_col = Flex::column()
        .with_child(serial_row)
        .with_child(info_scroll);
    main_col
}
