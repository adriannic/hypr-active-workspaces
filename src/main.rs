use hyprland::{data::Monitors, event_listener::EventListener, shared::HyprData};

fn main() -> hyprland::Result<()> {
    let workspaces: Vec<i32> = Monitors::get()?
        .map(|mon| mon.active_workspace.id)
        .collect();
    println!("{:?}", workspaces);

    let mut event_listener = EventListener::new();

    event_listener.add_workspace_change_handler(|_| {
        let workspaces: Vec<i32> = Monitors::get()
            .unwrap()
            .map(|mon| mon.active_workspace.id)
            .collect();
        println!("{:?}", workspaces);
    });

    event_listener.add_workspace_moved_handler(|_| {
        let workspaces: Vec<i32> = Monitors::get()
            .unwrap()
            .map(|mon| mon.active_workspace.id)
            .collect();
        println!("{:?}", workspaces);
    });

    event_listener.start_listener()
}
