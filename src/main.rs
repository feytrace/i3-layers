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
    listener.subscribe(&[Subscription::{Window, Workspace}])?;

    let mut workspace_state: HashMap<name: Option<String>, is_vertical: bool> = HashMap::new();
    let mut maybe_current_workspace: Option<String> = Some("1".to_string());

    for event in listener.listen() {
        if let Event::WindowEvent(e) = event? {
            if e.change == WindowChange::New || e.change == WindowChange::Close {
                workspace_state.entry(maybe_current_workspace).and_modify(|is_vertical| *is_vertical = *!is_vertical)
                let cmd = if workspace_state.entry(maybe_current_workspace) {
                    "split v"
                } else {
                    "split h"
                };

                tx.send(cmd)?;
            }
        }
        if let Event::WorkspaceEvent(e) = event? {
           if e.change == WorkspaceChange::Focus {
                maybe_current_workspace = WorkspaceEvent::current();
           } 
           if e.change == WorkspaceChange::Move {
                maybe_current_workspace = WorkspaceEvent::current();
                workspace_state.entry(maybe_current_workspace).and_modify(|is_vertical| *is_vertical = *!is_vertical)
           }
        }
    }

    Ok(())
}
