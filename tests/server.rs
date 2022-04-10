// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::thread;

use std::os::unix::net::UnixListener;

/// Split lines on CRLF
fn split_lines(lines: &str) -> Vec<String> {
    lines
        .trim_end()
        .split("\r\n")
        .map(|s| format!("{}\r\n", s))
        .collect::<Vec<String>>()
}

/// Handle the communication for tests.
///
/// The communication is a list of (question, answer). If the client sends the expected question
/// in the sequence, the answer is returned.
fn serve_streams(
    instream: &mut dyn Read,
    outstream: &mut dyn Write,
    communication: &[(&'static str, &'static str)],
) -> io::Result<()> {
    let mut input = BufReader::new(instream);
    let mut output = BufWriter::new(outstream);
    for (questions, answer) in communication.iter() {
        for question in split_lines(questions).iter() {
            let mut line = String::new();
            input.read_line(&mut line)?;
            if line != *question {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("read <{}> instead of <{}>", line, *question),
                ));
            }
        }
        output.write_all(answer.as_bytes())?;
        output.flush()?;
    }
    Ok(())
}

/// Server traits
pub trait Server {
    fn serve(&mut self) -> io::Result<()>;
}

/// Server on a named socket.
pub struct UnixServer {
    listener: UnixListener,
    communication: Vec<(&'static str, &'static str)>,
}

impl UnixServer {
    /// Create a new server on a named socket.
    ///
    /// Argument `communication` is an array of pairs. The first item is a list of strings
    /// the server will receive and the second item is the answer.
    pub fn new<P>(
        socket_path: P,
        communication: &[(&'static str, &'static str)],
    ) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let listener = UnixListener::bind(socket_path.as_ref())?;
        Ok(Self {
            listener,
            communication: communication.to_vec(),
        })
    }
}

impl Server for UnixServer {
    fn serve(&mut self) -> io::Result<()> {
        let (mut stream, _) = self.listener.accept()?;
        serve_streams(&mut stream.try_clone()?, &mut stream, &self.communication)
    }
}

/// Run the server in a thread
pub fn run_server(mut server: Box<dyn Server + Send>) -> thread::JoinHandle<io::Result<()>> {
    thread::spawn(move || -> io::Result<()> {
        server.serve()?;
        Ok(())
    })
}

pub fn run_unix<P>(
    socket_path: P,
    communication: &'static [(&'static str, &'static str)],
) -> io::Result<thread::JoinHandle<io::Result<()>>>
where
    P: AsRef<Path>,
{
    Ok(run_server(Box::new(UnixServer::new(
        &socket_path,
        communication,
    )?)))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_split_lines() {
        const ONE_LINE: &str = "one line\r\n";
        let one_line = super::split_lines(ONE_LINE);
        assert_eq!(&[ONE_LINE], one_line.as_slice());
    }
}
