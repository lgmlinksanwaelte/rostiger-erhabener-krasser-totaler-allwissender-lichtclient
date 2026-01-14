/// this enum defines the roles that can be assigned to a user
/// The roles are:
/// - Programmer
/// - blind Programmer
/// - Operator
/// - Interface
#[derive(Clone)]
pub enum Role {
    Programmer,
    BlindProgrammer,
    Operator,
    Interface,
}

///Implementation of method as_str to get the Role Name as a &str
impl Role {
    pub(crate) fn as_str(&self) -> &str {
        match self {
            Role::Programmer => "Programmer",
            Role::BlindProgrammer => "blind Programmer",
            Role::Operator => "Operator",
            Role::Interface => "Interface",
        }
    }
}