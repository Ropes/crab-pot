//! This example performs a loopback test using real hardware ports
//!
//! Additionally, some data will be collected and logged during the test to provide some
//! rudimentary benchmarking information. When 'split-port' is specified, the serial port will
//! be split into two channels that read/write "simultaneously" from multiple threads.
//!
//! You can also provide the length (in bytes) of data to test with, and the number of iterations to perform or
//! a list of raw bytes to transmit.
//!
//! To run this example:
//!
//! 1) `cargo run --example loopback /dev/ttyUSB0`
//!
//! 2) `cargo run --example loopback /dev/ttyUSB0 --split-port`
//!
//! 3) `cargo run --example loopback /dev/ttyUSB0 -i 100 -l 32 -b 9600`
//!
//! 4) `cargo run --example loopback /dev/ttyUSB8 --bytes 222,173,190,239`

use std::time::{Duration, Instant};

use clap::Parser;
use serialport::SerialPort;

/// Serialport Example - Loopback
#[derive(Parser)]
struct Args {
    /// The device path to a serialport
    port: String,

    /// The number of read/write iterations to perform
    #[clap(short, long, default_value = "100")]
    iterations: usize,

    /// The number of bytes written per transaction
    ///
    /// Ignored when bytes are passed directly from the command-line
    #[clap(short, long, default_value = "8")]
    length: usize,

    /// The baudrate to open the port with
    #[clap(short, long, default_value = "115200")]
    baudrate: u32,

    /// Bytes to write to the serial port
    ///
    /// When not specified, the bytes transmitted count up
    #[clap(long, use_value_delimiter = true)]
    bytes: Option<Vec<u8>>,

    /// Split the port to read/write from multiple threads
    #[clap(long)]
    split_port: bool,
}
