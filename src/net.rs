use anyhow::{Context, Result};
use std::io::{BufRead, BufReader, BufWriter, Read, Write};

pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub struct IOHandler<R: Read, W: Write, L: Write> {
    rx: BufReader<R>,
    tx: BufWriter<W>,
    log: BufWriter<L>,
}

impl<R: Read, W: Write, L: Write> IOHandler<R, W, L> {
    pub fn new(rx: R, tx: W, lx: L) -> Self {
        Self {
            rx: BufReader::new(rx),
            tx: BufWriter::new(tx),
            log: BufWriter::new(lx),
        }
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut s = String::new();
        self.rx
            .read_line(&mut s)
            .context("error reading line from rx")?;
        Ok(s.to_string())
    }

    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.tx.write_all(buf).context("error writing to tx")?;
        self.tx.write_all(b"\n").context("writing newline")?;
        self.tx.flush().context("error flushing buffer")?;
        Ok(())
    }

    pub fn log(&mut self, message: &str, level: LogLevel) -> Result<()> {
        let message = match level {
            LogLevel::ERROR => format!("ERROR: {}", message),
            LogLevel::WARN => format!("WARNING: {}", message),
            LogLevel::INFO => format!("INFO: {}", message),
            LogLevel::DEBUG => format!("DEBUG: {}", message),
        };
        self.log.write_all(message.as_bytes())?;
        self.log.flush()?;
        Ok(())
    }
}
