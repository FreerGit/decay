mod exchange;
mod settings;

use settings::settings::Settings;

#[tokio::main]
async fn main() -> () {
    //init settings
    // settings::Settings::new()
    Settings::new();
    //init client
    //init server for settings updates (@TODO l8r on)
    //start exectuor
}
