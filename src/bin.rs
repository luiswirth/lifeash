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

mod graphics;
mod logging;
mod simulator;

use simulator::Simulator;

#[allow(dead_code)]
const BI_BLOCK_PATTERN: &str = "oo$oo!";
#[allow(dead_code)]
const OCTAGON2_PATTERN: &str = "bobbob$oboobo$bobbob$bobbob$oboobo$bobbob!";
#[allow(dead_code)]
const B52BOMBER_PATTERN: &str = r#"#N B-52 bomber
x = 39, y = 21, rule = B3/S23
b2o36b$b2o17bo18b$19bobo12bobo2b$20bo12bo5b$2o7b2o23bo2bob$2obo5b2o23b
obobo$3bo23bo7bo2bo$3bo23b2o7b2ob$o2bo17b2o5bo10b$b2o18bo17b$21b3o15b$
36b2ob$36b2ob$b2o36b$o2bo35b$obobo16bobo4b2o5b2o2b$bo2bo17b2o4b2o5b2ob
o$5bo12bo3bo15bo$2bobo12bobo18bo$18bo16bo2bo$36b2o!"#;
const P43GLIDERLOOP_PATTERN: &str = r#"
x = 65, y = 65, rule = B3/S23
27b2o$27bobo$29bo4b2o$25b4ob2o2bo2bo$25bo2bo3bobob2o$28bobobobo$29b2ob
obo$33bo2$19b2o$20bo8bo$20bobo5b2o$21b2o$35bo$36bo$34b3o2$25bo$25b2o$
24bobo4b2o22bo$31bo21b3o$32b3o17bo$34bo17b2o2$45bo$46b2o12b2o$45b2o14b
o$3b2o56bob2o$4bo9b2o37bo5b3o2bo$2bo10bobo37b2o3bo3b2o$2b5o8bo5b2o35b
2obo$7bo13bo22b2o15bo$4b3o12bobo21bobo12b3o$3bo15b2o22bo13bo$3bob2o35b
2o5bo8b5o$b2o3bo3b2o37bobo10bo$o2b3o5bo37b2o9bo$2obo56b2o$3bo14b2o$3b
2o12b2o$19bo2$11b2o17bo$12bo17b3o$9b3o21bo$9bo22b2o4bobo$38b2o$39bo2$
28b3o$28bo$29bo$42b2o$35b2o5bobo$35bo8bo$44b2o2$31bo$30bobob2o$30bobob
obo$27b2obobo3bo2bo$27bo2bo2b2ob4o$29b2o4bo$35bobo$36b2o!"#;

fn main() -> Result<()> {
    logging::setup_subscriber();

    info!("starting simulator");
    let mut simulator = Simulator::new();

    simulator.read_rls(P43GLIDERLOOP_PATTERN);

    info!("start simulation loop");
    simulator.run();

    Ok(())
}
