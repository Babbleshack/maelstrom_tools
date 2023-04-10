use std::io::{
    self, BufRead, BufReader, BufWriter, Read, Stderr, StderrLock, Stdin, StdinLock, Stdout,
    StdoutLock, Write,
};

pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub struct IOHandler {
    stdin: BufReader<StdinLock<'static>>,
    stdout: BufWriter<StdoutLock<'static>>,
    stderr: BufWriter<StderrLock<'static>>,
}

impl IOHandler {
    pub fn new() -> Self {
        Self {
            stdin: BufReader::new(std::io::stdin().lock()),
            stdout: BufWriter::new(std::io::stdout().lock()),
            stderr: BufWriter::new(std::io::stderr().lock()),
        }
    }

    pub fn read_line(&mut self) -> std::io::Result<String> {
        let mut s = String::new();
        self.stdin.read_line(&mut s)?;
        Ok(s.to_string())
    }

    pub fn write(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.stdout.write_all(buf)?;
        self.stdout.flush()?;
        Ok(())
    }

    pub fn log(&mut self, message: String, level: LogLevel) -> std::io::Result<()> {
        let message = match level {
            LogLevel::ERROR => format!("ERROR: {}", message),
            LogLevel::WARN => format!("WARNING: {}", message),
            LogLevel::INFO => format!("INFO: {}", message),
            LogLevel::DEBUG => format!("DEBUG: {}", message),
        };
        self.stderr.write_all(message.as_bytes())?;
        self.stderr.flush()?;
        Ok(())
    }
}

impl Default for IOHandler {
    fn default() -> Self {
        Self::new()
    }
}
