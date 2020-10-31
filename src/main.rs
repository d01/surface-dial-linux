#![allow(clippy::collapsible_if, clippy::new_without_default)]

mod common;
mod config;
pub mod controller;
mod dial_device;
mod error;
mod fake_input;

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

use crate::controller::DialController;
use crate::dial_device::DialDevice;
use crate::error::Error;

use notify_rust::{Hint, Notification, Timeout};
use signal_hook::{iterator::Signals, SIGINT, SIGTERM};

fn main() {
    if let Err(e) = true_main() {
        println!("{}", e);
    }
}

fn true_main() -> DynResult<()> {
    println!("Started");

    let cfg = config::Config::from_disk()?;

    let dial = DialDevice::new(std::time::Duration::from_millis(750))?;
    println!("Found the dial");

    std::thread::spawn(move || {
        // TODO: use this persistent notification for the meta mode.

        let active_notification = Notification::new()
            .hint(Hint::Resident(true))
            .hint(Hint::Category("device".into()))
            .timeout(Timeout::Never)
            .summary("Surface Dial")
            .body("Active!")
            .icon("media-optical") // it should be vaguely circular :P
            .show()
            .expect("failed to send notification");

        let signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();
        for sig in signals.forever() {
            eprintln!("received signal {:?}", sig);
            active_notification.close();
            std::process::exit(1);
        }
    });

    let mut controller = DialController::new(
        dial,
        cfg.last_mode,
        vec![
            Box::new(controller::controls::ScrollZoom::new()),
            Box::new(controller::controls::Volume::new()),
            Box::new(controller::controls::Media::new()),
            Box::new(controller::controls::DPad::new()),
        ],
    );

    controller.run()
}
