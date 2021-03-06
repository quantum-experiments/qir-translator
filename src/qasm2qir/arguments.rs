use qasm::Argument;

pub struct Register(pub String, pub i32);
impl Register {
    pub fn as_qir_name(&self) -> String {
        format!("{}{}", self.0, self.1)
    }
}

// Needed so that we have a local type to avoid the orphan rule for
// trait coherence.
pub struct Pair<Register1, Register2>(pub Register1, pub Register2);

impl TryFrom<&Argument> for Register {
    type Error = String;

    fn try_from(value: &Argument) -> Result<Self, Self::Error> {
        match value {
            Argument::Qubit(name, idx) => Ok(Register(name.to_owned(), *idx)),
            _ => Err(format!("Expected qubit argument, got {:?}.", value))
        }
    }
}

impl TryFrom<&Vec<Argument>> for Register {
    type Error = String;

    fn try_from(value: &Vec<Argument>) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            return Err(format!("Wrong number of arguments: expected 1, got {}.", value.len()));
        }
        (&value[0]).try_into()
    }
}

impl TryFrom<&Vec<Argument>> for Pair<Register, Register> {
    type Error = String;

    fn try_from(value: &Vec<Argument>) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!("Wrong number of arguments: expected 1, got {}.", value.len()));
        }
        Ok(Pair(
            (&value[0]).try_into()?,
            (&value[1]).try_into()?,
        ))
    }
}
