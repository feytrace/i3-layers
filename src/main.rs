use i3ipc::{
    event::{inner::WindowChange, Event},
    I3Connection, I3EventListener, Subscription,
};

use std::{
    sync::mpsc,
    thread,
};

fn main() {
    let (tx, rx) = mpsc::channel::<&'static str>();

    thread::spawn(move || {
        let mut conn = I3Connection::connect().expect("Failed to connect to i3");

        for cmd in rx {
            if let Err(e) = conn.run_command(cmd) {
                eprintln!("i3 command failed: {:?}", e);
            }
        }
    });

    layout_manager(tx).expect("Layer manager failed");
}

fn layout_manager(tx: mpsc::Sender<&'static str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = I3EventListener::connect()?;
    listener.subscribe(&[Subscription::Window])?;

    let mut vertical = false;

    for event in listener.listen() {
        if let Event::WindowEvent(e) = event? {
            if e.change == WindowChange::New {
                vertical = !vertical;

                let cmd = if vertical {
                    "split v"
                } else {
                    "split h"
                };

                tx.send(cmd)?;
            }
        }
    }

    Ok(())
}
