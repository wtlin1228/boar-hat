#![warn(missing_docs, missing_debug_implementations)]

//! `MockStream` provides a simple way to simulate input and capture output in memory.
//!
//! This create provides a mock stream for simulating input and output streams for testing purposes.
//! It implements the `Read` and `Write` traits, allowing it to be used in place of real input/output
//! streams, while capturing data written to it for inspection during tests.
//!
//! # Example
//!
//! ```
//! # use mock_stream::MockStream;
//! # use std::io::{Read, Write};
//! #
//! fn foo<S>(mut stream: S)
//! where
//!     S: Read + Write,
//! {
//!     let mut buf: Vec<u8> = vec![];
//!     stream.read(&mut buf).unwrap();
//!     stream.write(b"hello").unwrap();
//!     stream.flush().unwrap();
//! }
//!
//! let input_data = b"example input data";
//!
//! // Create a new `MockStream` instance with the input data
//! let mut mock_stream = MockStream::new(input_data);
//! foo(&mut mock_stream);
//!
//! // Get the data written to the output stream
//! let received_data = mock_stream.get_received();
//! assert_eq!(received_data, b"hello");
//! ```

use std::io::Cursor;
use std::io::{Read, Write};

/// A mock stream that simulates input and captures output in memory.
#[derive(Debug)]
pub struct MockStream {
    input_stream: Cursor<Vec<u8>>,
    output_stream: Cursor<Vec<u8>>,
}

impl MockStream {
    /// Creates a new `MockStream` instance with the provided input data.
    ///
    /// # Arguments
    ///
    /// * `input` - A slice of bytes representing the input data.
    ///
    /// # Example
    ///
    /// ```
    /// # use mock_stream::MockStream;
    /// let input_data = b"example input data";
    /// let mock_stream = MockStream::new(input_data);
    /// ```
    pub fn new(input: &[u8]) -> Self {
        Self {
            input_stream: Cursor::new(input.to_owned()),
            output_stream: Cursor::new(vec![]),
        }
    }

    /// Gets a reference to the data that has been written to the output stream.
    ///
    /// # Returns
    ///
    /// A reference to a vector containing the data that has been written to the output stream.
    pub fn get_received(&self) -> &Vec<u8> {
        self.output_stream.get_ref()
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.input_stream.read(buf)
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.output_stream.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
