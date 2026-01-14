use crate::roles;
use crate::logic;
use crate::logic::{save_name, save_role, start_rektal};

/// Enum representing different GUI actions in the R.E.K.T.A.Launcher application
/// that can be executed based on user interactions.
pub enum GuiAction {
    StartRektal,
    SaveRole(roles::Role),
    SaveName(String),

}


impl GuiAction { //TODO: alle Konsolenausgaben entfernen

    /// Implementation of the GuiAction enum providing a method to execute the corresponding action.
    /// Each variant of the enum corresponds to a specific user interaction
    /// in the GUI, such as clicking a button or changing a dropdown selection.
    pub fn execute(&self) {
        match self {
            GuiAction::StartRektal => {
                println!("Button wurde geklickt");
                start_rektal(); //TODO: Fehlerbehandlung
            }
            GuiAction::SaveRole(selection) => {
                println!("Dropdown geändert: {}", selection.as_str());
                save_role(selection.clone());
            }

            GuiAction::SaveName(name) => {
                println!("Name geändert: {}", name);
                save_name(name.clone());
            }
        }
    }
}