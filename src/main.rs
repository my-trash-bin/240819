use midir::MidiInput;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

fn main() -> Result<(), Box<dyn Error>> {
    let midi_in = MidiInput::new("test")?;

    println!("Available MIDI input ports:");
    let in_ports = midi_in.ports();
    for (i, port) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(port)?);
    }

    let connections: Arc<_> = Arc::new(Mutex::new(vec![]));

    for (index, port) in in_ports.iter().enumerate() {
        let port_name = midi_in.port_name(port)?;
        let device_name = format!("Device {}", index);
        let connections = Arc::clone(&connections);
        let port_name_copy = port_name.clone();
        let device_name_copy = device_name.clone();
        let conn_in =
            MidiInput::new(format!("[{}]({}) - {}", index, port_name, device_name).as_str())?
                .connect(
                    port,
                    &device_name,
                    move |_, message, _| {
                        println!(
                            "[{}]({}) - {}: {:?}",
                            index, port_name_copy, device_name_copy, message
                        );
                    },
                    (),
                )?;
        connections.lock().unwrap().push(conn_in);
        println!("Listening on [{}]({}) - {}", index, port_name, device_name);
    }

    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(())
}
