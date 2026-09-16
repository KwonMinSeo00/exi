// Command-Line Interface

use std::fmt;
use std::io::{self, Write};
use std::marker::PhantomData;
use std::path::PathBuf;
use sha2::{Digest, Sha256};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "EXI",
    version = "v0.0.1.0",
)]

pub struct Interface {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {

    // "exi parse <PATH>"
    Parse {

    },

    // "exi disasm <RVA> <PATH>"
    Disasm {

    },

    // "exi rva <PATH>"
    RVA {

    },

    // "exi imports <PATH>"
    Imports {

    },

    // "exi exports <PATH>"
    Exports {

    },

    // "exi strings <PATH>"
    Strings {

    },
}

#[derive(Args, Debug)]
pub struct Output {
    // Write Struct
}

impl Output {
    // Write Implication
}

pub struct Writer<'a, W> {
    pub phantom_data: PhantomData<W>
}

impl<W: Write> fmt::Write for Write<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        todo!("Write Function")
    }
}

pub fn write<W: Write>() -> Result<(), String> {
    todo!("Write Function")
}

pub fn run() -> Result<(), String> {
    todo!("Write Function")
}

pub fn main_entry() -> std::process::ExitCode {
    todo!("Write Entry Point")
}