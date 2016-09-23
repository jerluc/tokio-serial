//! Bindings for serial ports and futures
//!
//! This crate provides bindings between `mio_serial`, the mio crate for serial
//! ports, and `futures`. The APIs and bindings in this crate are somewhat
//! similar to the TCP and UDP bindings in the `futures-mio` crate.

#![deny(missing_docs)]

extern crate futures;
extern crate tokio_core;
extern crate mio_serial;

use std::ffi::OsStr;
use std::fmt;
use std::io::{self, Read, Write};

use futures::Async;
use tokio_core::reactor::{PollEvented, Handle};
use tokio_core::io::Io;

// Re-exports
pub use mio_serial::{PortSettings};
// pub use mio_serial::BaudRate::*;
// pub use mio_serial::CharSize::*;
// pub use mio_serial::Parity::*;
// pub use mio_serial::StopBits::*;
// pub use mio_serial::FlowControl::*;

/// A structure representing an open serial port.
pub struct SerialPort {
    io: PollEvented<mio_serial::SerialPort>,
}

impl SerialPort {
    /// open serial port named by port_name with custom settings.
    ///
    pub fn open_with_settings<T: AsRef<OsStr> + ?Sized>(port_name: &T,
            settings: &PortSettings,
            handle: &Handle) -> io::Result<SerialPort> {
        SerialPort::_open_with_settings(port_name, settings, handle)
    }

    fn _open_with_settings<T: AsRef<OsStr> + ?Sized>(port_name: &T,
            settings: &PortSettings,
            handle: &Handle) -> io::Result<SerialPort> {
        let port = try!(mio_serial::SerialPort::open_with_settings(port_name, settings));
        SerialPort::new(port, handle)
    }

    fn new(port: mio_serial::SerialPort, handle: &Handle) -> io::Result<SerialPort> {
        let io = try!(PollEvented::new(port, handle));
        Ok(SerialPort { io: io })
    }

    /// Test whether this serial port is ready to be read or not.
    pub fn poll_read(&self) -> Async<()> {
        self.io.poll_read()
    }

    /// Test whether this serial port is ready to be written to or not.
    pub fn poll_write(&self) -> Async<()> {
        self.io.poll_write()
    }
}

impl Read for SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.io.read(buf)
    }
}

impl Write for SerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.io.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl Io for SerialPort {
    fn poll_read(&mut self) -> Async<()> {
        <SerialPort>::poll_read(self)
    }

    fn poll_write(&mut self) -> Async<()> {
        <SerialPort>::poll_write(self)
    }
}

impl<'a> Read for &'a SerialPort {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&self.io).read(buf)
    }
}

impl<'a> Write for &'a SerialPort {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&self.io).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&self.io).flush()
    }
}

impl<'a> Io for &'a SerialPort {
    fn poll_read(&mut self) -> Async<()> {
        <SerialPort>::poll_read(self)
    }

    fn poll_write(&mut self) -> Async<()> {
        <SerialPort>::poll_write(self)
    }
}

/*
impl fmt::Debug for SerialPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.io.get_ref().fmt(f)
    }
}
*/

#[cfg(unix)]
mod sys {
    use std::os::unix::prelude::*;
    use super::SerialPort;

    impl AsRawFd for SerialPort {
        fn as_raw_fd(&self) -> RawFd {
            self.io.get_ref().as_raw_fd()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}