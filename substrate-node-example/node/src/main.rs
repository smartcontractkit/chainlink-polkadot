//! Substrate Node Template CLI library.
#![warn(missing_docs)]
#![allow(clippy::all)]

mod chain_spec;
#[macro_use]
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
