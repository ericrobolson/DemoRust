use crate::location::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Error<Kind> {
    pub kind: Kind,
    pub location: Location,
}
