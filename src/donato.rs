use hbb_common::config;

pub const APP_NAME: &str = "Donato - Suporte";

pub fn init() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    config::OVERWRITE_SETTINGS.write().unwrap().extend([
        ("allow-auto-update".to_owned(), "N".to_owned()),
        ("custom-rendezvous-server".to_owned(), "rustdesk.apps.donato.inf.br".to_owned()),
        ("relay-server".to_owned(), "rustdesk.apps.donato.inf.br".to_owned()),
        ("key".to_owned(), "V5s3D7pSa7JTTVTCXTfIPvaIZD9q3CPJrg1fcawR4CE=".to_owned()),
    ]);
    config::HARD_SETTINGS.write().unwrap().insert("conn-type".to_owned(), "incoming".to_owned());
}
