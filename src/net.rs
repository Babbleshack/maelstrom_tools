use anyhow::Result;
use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    sync::Mutex,
};

pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub struct IOHandler<R: Read, W: Write, L: Write> {
    rx: Mutex<BufReader<R>>,
    tx: Mutex<BufWriter<W>>,
    log: Mutex<BufWriter<L>>,
}

impl<R: Read, W: Write, L: Write> IOHandler<R, W, L> {
    pub fn new(rx: R, tx: W, log: L) -> Self {
        let rx = BufReader::new(rx);
        let tx = BufWriter::new(tx);
        let log = BufWriter::new(log);
        Self {
            rx: Mutex::new(rx),
            tx: Mutex::new(tx),
            log: Mutex::new(log),
        }
    }
    pub fn log(&mut self, message: &str, level: LogLevel) -> Result<()> {
        let message = match level {
            LogLevel::ERROR => format!("ERROR: {}", message),
            LogLevel::WARN => format!("WARNING: {}", message),
            LogLevel::INFO => format!("INFO: {}", message),
            LogLevel::DEBUG => format!("DEBUG: {}", message),
        };
        let mut log = self.log.lock().expect("error locking log");
        log.write_all(message.as_bytes())?;
        log.flush()?;
        Ok(())
    }

    pub fn read_line(&mut self) -> Result<String> {
        let mut rx = self.rx.lock().unwrap();
        let mut buf = String::new();
        rx.read_line(&mut buf)?;
        Ok(buf)
    }

    pub fn write_line(&mut self, buf: String) -> Result<()> {
        self.write(&buf.as_bytes())?;
        self.write("\r\n".as_bytes())?;
        self.flush()?;
        Ok(())
    }
}

impl<R: Read, W: Write, L: Write> Read for IOHandler<R, W, L> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut rx = self.rx.lock().expect("error locking rx");
        rx.read(buf)
    }
}
impl<R: Read, W: Write, L: Write> Write for IOHandler<R, W, L> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut tx = self.tx.lock().expect("error locking tx");
        tx.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut tx = self.tx.lock().expect("error flushing tx");
        tx.flush()
    }
}
