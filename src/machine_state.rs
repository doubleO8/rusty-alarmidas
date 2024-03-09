use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum MachineState {
    UNKNOWN = 100,
    NOT_RUNNING_W_ERROR = 6,
    NOT_RUNNING_WO_ERROR = 5,
    PRODUCTION_100 = 4,
    PRODUCTION_LT_100 = 3,
    PROGRAM_INTERRUPTION = 2,
    WARMUP = 1,
    NOT_SWITCHED_ON = 0,
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", lookup_by_id(*self));

        Ok(())
    }
}

pub const MACHINE_STATE_KEYS: [MachineState; 7] = [
    MachineState::NOT_SWITCHED_ON,
    MachineState::WARMUP,
    MachineState::PROGRAM_INTERRUPTION,
    MachineState::PRODUCTION_LT_100,
    MachineState::PRODUCTION_100,
    MachineState::NOT_RUNNING_WO_ERROR,
    MachineState::NOT_RUNNING_W_ERROR,
];

pub fn lookup_by_id(state_id: MachineState) -> &'static str {
    match state_id {
        MachineState::WARMUP => "warmup",
        MachineState::PRODUCTION_LT_100 => "production_lt_100",
        MachineState::PRODUCTION_100 => "production_100",
        MachineState::NOT_RUNNING_W_ERROR => "not_running_w_error",
        MachineState::NOT_RUNNING_WO_ERROR => "not_running_wo_error",
        MachineState::NOT_SWITCHED_ON => "not_switched_on",
        MachineState::PROGRAM_INTERRUPTION => "program_interruption",
        MachineState::UNKNOWN => "unknown",
    }
}

pub fn lookup_by_name(name: &str) -> MachineState {
    match name {
        "warmup" => MachineState::WARMUP,
        "production_lt_100" => MachineState::PRODUCTION_LT_100,
        "production_100" => MachineState::PRODUCTION_100,
        "not_running_w_error" => MachineState::NOT_RUNNING_W_ERROR,
        "not_running_wo_error" => MachineState::NOT_RUNNING_WO_ERROR,
        "not_switched_on" => MachineState::NOT_SWITCHED_ON,
        "program_interruption" => MachineState::PROGRAM_INTERRUPTION,
        &_ => MachineState::UNKNOWN,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_by_id() {
        let result = lookup_by_id(MachineState::WARMUP);
        assert_eq!(result, "warmup");
    }

    #[test]
    fn test_lookup_by_name() {
        let result = lookup_by_name("warmup");
        assert_eq!(result, MachineState::WARMUP);
    }
}
