extern crate leftwm;
use leftwm::child_process::Nanny;
use std::env;
use std::process::Command;

fn main() {
    if let Ok(booter) = std::env::current_exe() {
        //boot everything in ~/.config/autostart
        Nanny::new().autostart();

        //Fix for JAVA apps so they repaint correctly
        env::set_var("_JAVA_AWT_WM_NONREPARENTING", "1");

        let mut worker = booter.clone();
        worker.pop();
        worker.push("leftwm-worker");

        loop {
            Command::new(&worker)
                .status()
                .expect("failed to start leftwm");
        }
    }
}
