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

extern crate lifeash as la;

mod cremator;
mod graphics;
mod logging;

use cremator::Cremator;

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
#[allow(dead_code)]
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
#[allow(dead_code)]
const HALFMAX_PATTER: &str = r#"
x = 65, y = 80, rule = b3/s23
5bobo49bobo5b$4bo2bo49bo2bo4b$3b2o55b2o3b$2bo59bo2b$b4o55b4ob$o4bo53bo
4bo$o2bo24b3o3b3o24bo2bo$o2bo24bo2bobo2bo24bo2bo$bo26bo7bo26bob$2b4obo
20bo7bo20bob4o2b$3bo3bo21bobobobo21bo3bo3b$4bo24bobobobo24bo4b$4bobo
23bo3bo23bobo4b$29b3ob3o29b$3b3o26bo26b3o3b$3b2o23b9o23b2o3b$3b3o21bo
9bo21b3o3b$26b13o26b$4bobo18bo13bo18bobo4b$4bo19b17o19bo4b$3bo3bo15bo
17bo15bo3bo3b$2b4obo14b21o14bob4o2b$bo19bo21bo19bob$o2bo16b25o16bo2bo$
o2bo15bo25bo15bo2bo$o4bo12b29o12bo4bo$b4o12bo29bo12b4ob$2bo13b33o13bo
2b$3b2o10bo33bo10b2o3b$4bo2bobo4b37o4bobo2bo4b$5bobo2bo2bo37bo2bo2bobo
5b$8bo3b20ob20o3bo8b$9bo21bobo21bo9b$10b21o3b21o10b2$8b21o3bo3b21o8b$
7bo21bobobobo21bo7b$6bo3b2o2bob2ob2ob2ob5obobob5ob2ob2ob2obo2b2o3bo6b$
6bo4bobo2b2o4b2o7bobo7b2o4b2o2bobo4bo6b$6bo8bo5bo5b3obobob3o5bo5bo8bo
6b$7b3o20b2ob2o20b3o7b$9bo17b2o3bo3b2o17bo9b$6b2o2bo3b2o11b4obob4o11b
2o3bo2b2o6b$4bo5b2obo17bobo17bob2o5bo4b$3bo6bo18bobobobo18bo6bo3b$3bo
4b2obo2bo10b2o2b3ob3o2b2o10bo2bob2o4bo3b$3b5o3b2o5bo6b2o2b2o3b2o2b2o6b
o5b2o3b5o3b$13b2o4bo25bo4b2o13b$10b2o3b2ob2obo7b3ob3o7bob2ob2o3b2o10b$
13bo4bobo23bobo4bo13b$9bo4b5obo4bo3b3ob3o3bo4bob5o4bo9b$8b2o2b2o4bo3b
3o3bo7bo3b3o3bo4b2o2b2o8b$9b2o3bo4b2o7b4ob4o7b2o4bo3b2o9b$10b2o8bo10bo
bo10bo8b2o10b$30b2ob2o30b$22bo3bob2obobob2obo3bo22b$21bobobobob2o3b2ob
obobobo21b$21bobobobo9bobobobo21b$22bobob3o7b3obobo22b$24bobo3b5o3bobo
24b$24b2o6bo6b2o24b2$27bo2bobobo2bo27b$27b11o27b2$29bo5bo29b$28bobo3bo
bo28b$28bob2ob2obo28b$26bobob2ob2obob2o25b$25bobo9bobo25b$25bo2b3o3b3o
28b$26b2o37b$29bo2bo2b2o28b$26b3obobobo2bo27b$26bo3bobobob2o27b$27bobo
4bo30b$28b2obo2bob2o27b$30bobobobo28b$30bobobobo28b$31b2ob2o!"#;
const ACORN_PATTERN: &str = r#"
x = 7, y = 3, rule = B3/S23
bo5b$3bo3b$2o2b3o!
"#;

fn main() -> Result<()> {
    logging::setup_subscriber();

    info!("starting simulator");
    let mut cremator = Cremator::new();

    cremator.read_rls(ACORN_PATTERN);

    info!("start simulation loop");
    cremator.run();

    Ok(())
}
