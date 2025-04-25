use super::Condition;
use super::ConditionError;

use flate2::read::GzDecoder;

use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

#[derive(thiserror::Error, Debug)]
/// Error type to allow
pub enum Error {
    #[error("missing /proc/config.gz. does your kernel have CONFIG_IKCONFIG_PROC=y?")]
    MissingProcConfig,

    #[error("invariant violated: {0}")]
    InvariantViolation(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl ConditionError for Error {
    fn is_ignorable(&self) -> bool {
        matches!(self, Self::MissingProcConfig)
    }
}

#[derive(Debug)]
pub struct HasKconfig {
    pub conf: String,
    pub allow_module: bool,
}

impl HasKconfig {
    pub fn new(conf: String) -> Self {
        Self {
            conf,
            allow_module: true,
        }
    }
}

impl Condition for HasKconfig {
    type Err = Error;

    fn check(&self) -> Result<bool, Self::Err> {
        let config = File::open("/proc/config.gz").map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Error::MissingProcConfig,
            _ => e.into(),
        })?;

        let mut config = BufReader::new(GzDecoder::new(config));
        let mut line = String::new();

        let pat = self.conf.clone() + "=";
        while config.read_line(&mut line)? != 0 {
            if let Some(m) = line.strip_prefix(&pat) {
                let value = m.trim();
                return match value {
                    "y" => Ok(true),
                    "m" => Ok(self.allow_module),
                    "" => Ok(false),
                    e => Err(Error::InvariantViolation(format!(
                        "unexpected value in config.gz: `{}`",
                        e
                    ))),
                };
            }
            line.clear();
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(HasKconfig::new("CONFIG_SCHED_CLASS_EXT".into())
            .check()
            .unwrap());
    }
}
