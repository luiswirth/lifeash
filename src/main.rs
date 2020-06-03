#[allow(unused)]
use color_eyre::{Help, Report, Result};
#[allow(unused)]
use eyre::{eyre, WrapErr};

#[allow(unused)]
pub use tracing::{
    debug, debug_span, error, error_span, info, info_span, instrument, trace, trace_span, warn,
    warn_span,
};

mod app;
mod logging;
mod simulator;
mod treelife;
mod universe;

fn main() -> Result<()> {
    logging::setup();

    simulator::start_simulator()?;

    Ok(())
}

// use logging to figure out problem
