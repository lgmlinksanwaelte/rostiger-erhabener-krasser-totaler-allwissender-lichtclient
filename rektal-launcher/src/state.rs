use crate::roles::Role;
pub struct State {
    pub(crate) name: String,
    pub(crate) role: Role,
    pub(crate) start_local: bool,
}