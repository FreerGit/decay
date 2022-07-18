mod settings;

#[tokio::main]
async fn main() -> () {
    //init settings
    // settings::Settings::new()
    settings::settings::Settings::new();
    //init client
    //init server for settings updates (@TODO l8r on)
    //start exectuor
}
