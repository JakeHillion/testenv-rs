use std::fs;
use std::io;

pub fn get_sched_ext_state() -> Result<bool, io::Error> {
    let path = "/sys/kernel/sched_ext/state";

    if fs::metadata(path).is_err() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("sched_ext state file not found at {}. is your kernel new enough with sched_ext enabled?", path),
        ));
    }

    let state = fs::read_to_string(path)
        .map_err(|e| io::Error::new(e.kind(), format!("Failed to read sched_ext state: {}", e)))?
        .trim()
        .to_string();

    match_sched_ext_state(&state)
}

fn match_sched_ext_state(state: &str) -> Result<bool, io::Error> {
    match state {
        "enabled" => Ok(true),
        "disabled" => Ok(false),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Invalid sched_ext state: {}", state),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_sched_ext_state() {
        assert!(match_sched_ext_state("enabled").unwrap());
        assert!(!match_sched_ext_state("disabled").unwrap());

        let result = match_sched_ext_state("invalid");
        assert!(result.is_err());
    }

    #[test]
    #[ignore = "This is an example of a test with system preconditions. It fails in the Nix sandbox."]
    fn test_get_sched_ext_state() {
        get_sched_ext_state().unwrap();
    }
}
