extern crate leftwm;
#[macro_use]
extern crate log;
extern crate env_logger;

use leftwm::*;
use std::panic;

fn get_events<T: DisplayServer>(ds: &T) -> Vec<DisplayEvent> {
    ds.get_next_events()
}

fn main() {
    env_logger::init();
    let result = panic::catch_unwind(|| {
        let mut manager = Box::new(Manager::default());
        let config = config::load();
        manager.tags = config.get_list_of_tags();
        let mut display_server: XlibDisplayServer = DisplayServer::new(&config);
        let handler = DisplayEventHandler { config };
        event_loop(&mut manager, &mut display_server, &handler);
    });
    info!("Completed: {:?}", result);
}

fn event_loop(
    manager: &mut Manager,
    display_server: &mut XlibDisplayServer,
    handler: &DisplayEventHandler,
) {
    let mut socket = Socket::new();
    //main event loop
    let mut events_remainder = vec![];
    loop {
        if manager.mode == Mode::NormalMode {
            socket.write_manager_state(manager);
        }
        let mut events = get_events(display_server);
        events.append(&mut events_remainder);

        let mut needs_update = false;
        for event in events {
            needs_update = handler.process(manager, event) || needs_update;
        }

        //if we need to update the displayed state
        if needs_update {
            let windows: Vec<&Window> = (&manager.windows).iter().map(|w| w).collect();
            let focused = manager.focused_window();
            display_server.update_windows(windows, focused);
            let workspaces: Vec<&Workspace> = (&manager.workspaces).iter().map(|w| w).collect();
            let focused = manager.focused_workspace();
            display_server.update_workspaces(workspaces, focused);
        }

        //preform any actions requested by the handler
        while !manager.actions.is_empty() {
            if let Some(act) = manager.actions.pop_front() {
                if let Some(event) = display_server.execute_action(act) {
                    events_remainder.push(event);
                }
            }
        }
    }
}
