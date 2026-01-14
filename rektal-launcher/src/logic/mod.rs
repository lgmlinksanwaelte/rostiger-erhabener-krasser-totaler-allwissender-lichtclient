/// The central logic module for R.E.K.T.A.L. launcher
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::roles::Role;
use crate::state::State;

///Module internal state managed with lazy_static and Mutex
lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State {
        name: String::new(),
        role: Role::Operator,
        start_local: false,
    });
}

/// Start the R.E.K.T.A.L. application with the current state
pub(crate) fn start_rektal() -> std::io::Result<()> {
    // Lock auf den State holen
    let state = STATE.lock().unwrap();
    println!("Starting R.E.K.T.A.L. as role: {}", state.role.as_str());

    // TODO: Hier könnt Ihr args aus state oder anderen Variablen an Command übergeben
    let status = Command::new("echo")
        //.args(...)
        .status()?;

    println!("R.E.K.T.A.L. Exit-Code: {}", status);
    Ok(())
}

/// Save the selected role into the module-internal state
pub(crate) fn save_role(role: Role) {
    let mut state = STATE.lock().unwrap();
    state.role = role;
    println!("Role saved: {}", state.role.as_str());
}

/// Save the entered name into the module-internal state
pub(crate) fn save_name(name: String) {
    let mut state = STATE.lock().unwrap();
    state.name = name;
    println!("Name saved: {}", state.name);
}

/// Set whether to start R.E.K.T.A.L. locally or not
pub(crate) fn set_start_local(start: bool) {
    let mut state = STATE.lock().unwrap();
    state.start_local = start;
    println!("Start local: {}", state.start_local);
}
