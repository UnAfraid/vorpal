use vorpal_api::vorpal::common::v0::{System};

pub mod agent;
pub mod config;
pub mod package;
pub mod store;
pub mod worker;

pub trait BuildSystem {
    fn from_str(system: &str) -> Self;
}

impl BuildSystem for System {
    fn from_str(system: &str) -> Self {
        match system {
            "aarch64-linux" => System::Aarch64Linux,
            "aarch64-macos" => System::Aarch64Macos,
            "x86_64-linux" => System::X8664Linux,
            "x86_64-macos" => System::X8664Macos,
            _ => System::default(),
        }
    }
}

pub fn get_build_system<T: BuildSystem>(system: &str) -> T {
    T::from_str(system)
}
