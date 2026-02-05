use i3ipc::event::inner::{WindowChange, WorkspaceChange};
use i3ipc::event::Event;
use i3ipc::{I3Connection, I3EventListener, Subscription};
use std::{collections::HashMap, sync::mpsc, thread};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let mut conn = I3Connection::connect().expect("Failed to connect to i3");
        for cmd in rx {
            if let Err(e) = conn.run_command(&cmd) {
                eprintln!("i3 command failed: {:?}", e);
            }
        }
    });

    let mut listener = I3EventListener::connect()?;
    listener.subscribe(&[Subscription::Window, Subscription::Workspace])?;

    let mut next_split: HashMap<Option<String>, bool> = HashMap::new();
    let mut maybe_current_workspace: Option<String> = Some("1".to_string());

    for event in listener.listen() {
        let event = event?;
        match event {
            Event::WindowEvent(e) => {
                if e.change == WindowChange::New {
                    let is_vertical = next_split
                        .entry(maybe_current_workspace.clone())
                        .or_insert(false);

                    let cmd = if *is_vertical { "split v" } else { "split h" };
                    tx.send(cmd.to_string())?;

                    *is_vertical = !*is_vertical;
                }
            }

            Event::WorkspaceEvent(e) => match e.change {
                WorkspaceChange::Focus => {
                    maybe_current_workspace = e.current.as_ref().and_then(|ws| ws.name.clone());
                }
                WorkspaceChange::Move => {
                    maybe_current_workspace = e.current.as_ref().and_then(|ws| ws.name.clone());

                    next_split
                        .entry(maybe_current_workspace.clone())
                        .or_insert(false);
                }
                _ => {}
            },

            _ => {}
        }
    }

    Ok(())
}
