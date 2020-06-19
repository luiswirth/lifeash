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

extern crate hashlife as hl;

mod logging;
mod simulator;

use simulator::Simulator;

const BI_BLOCK_PATTERN: &str = "oo$oo!";

fn main() -> Result<()> {
    logging::setup_subscriber();

    info!("starting simulator");
    let mut simulator = Simulator::new();

    simulator.read_rls(BI_BLOCK_PATTERN);

    info!("start simulation loop");
    simulator.run();

    Ok(())
}
