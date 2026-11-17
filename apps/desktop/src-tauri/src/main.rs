// Esconde o console no Windows em release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    oxid_desktop::run();
}
