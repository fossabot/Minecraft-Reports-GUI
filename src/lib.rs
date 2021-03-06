extern crate chrono;
#[macro_use] extern crate failure;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate strum;
#[macro_use] extern crate strum_macros;

use std::io::Read;

use failure::Error;

pub mod report;
pub mod minecraft;

pub fn read_report_raw<R: Read>(reader: R) -> Result<report::raw::Report, Error> {
    Ok(report::read_raw_report(reader)?)
}
