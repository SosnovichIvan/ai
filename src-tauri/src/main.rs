// Не открывает консоль на Windows в release-сборке.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ai_works_lib::run();
}
