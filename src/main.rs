#![feature(hash_set_entry)]
//#![feature(type_alias_impl_trait)]

#[allow(unused)]
use color_eyre::{Help, Report, Result};
#[allow(unused)]
use eyre::{eyre, WrapErr};

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

mod core;
mod logging;
mod node;
mod simulator;
mod universe;

use simulator::Simulator;

fn main() -> Result<()> {
    logging::setup();

    info!("starting simulator");
    let mut simulator = Simulator::new();
    info!("reading pattern");
    simulator.read_pattern()?;
    info!("finished reading pattern");

    info!("start simulation loop");
    simulator.run();

    Ok(())
}

// interesting patterns:
// Octagon 2: bobbob$oboobo$bobbob$bobbob$oboobo$bobbob!
