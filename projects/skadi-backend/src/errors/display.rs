use super::*;

impl Error for SkadiError {}


impl Debug for SkadiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for SkadiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}


impl Display for SkadiErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { SkadiErrorKind::UnknownError => { write!(f, "UnknownError") } }
    }
}