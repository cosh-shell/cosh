use toml::value::Map;

// #[derive(Serialize)]
// #[derive(Deserialize)]
pub struct Configuration {

    version: String,
    aliases: Map<String, String>, // -> aliases come in the form <Original-Command, Alias>.

}