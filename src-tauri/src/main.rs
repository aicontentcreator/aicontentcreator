use tauri::{Manager, AppHandle};
use tauri::{command, State};
use tokio::sync::Mutex;

//use tokio::signal;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{self, Duration};

//use daemon::daemon::daemon::init_daemon_actor_daemon_proxy;

use wallet::wallet_v1::wallet_settings::WalletSettings;
use wallet::wallet_v1::wallet::init_wallet_actor_wallet_proxy_with_seed;

use daemon::daemon::daemon::init_daemon_actor_daemon_proxy;
use daemon::settings::daemon_settings::DaemonSettings;

use crate::app_state::app_state::AppState;



mod app_state;
use crate::app_state::app_state::handle_frontend_request;


mod app_data;
use crate::app_data::app_paths::get_app_data_dir;


#[tokio::main]
async fn main() {
    let app_data_path = get_app_data_dir()
    .expect("Failed to access or create app data directory");

    println!("App data directory created or exists at: {:?}", app_data_path);

    let default_wallet_settings=WalletSettings::new_default_wallet_settings();
    let (app_wallet_actor, app_wallet_proxy) = init_wallet_actor_wallet_proxy_with_seed("MySeed".into(), default_wallet_settings).unwrap();
    tokio::spawn(app_wallet_actor.process());
    
    // Initialize the dmnactor and its dmnproxy
    //let channelsize=128;
    //let (dmnactor, dmnproxy) = init_daemon_actor_daemon_proxy("".into(), channelsize).unwrap();

    let (app_daemon_actor, app_daemon_proxy) = init_daemon_actor_daemon_proxy("".into(),app_wallet_proxy.clone(), DaemonSettings::new_default_daemon_settings()).await.unwrap();
    
    // Spawn the dmnactor's processing task
    tokio::spawn(app_daemon_actor.process());
    
    //
    tauri::Builder::default()
    .manage(AppState {
        shared_app_daemon_proxy: Mutex::new(app_daemon_proxy),
    })
    .invoke_handler(tauri::generate_handler![handle_frontend_request])
    .run(tauri::generate_context!())
    .expect("error while running Tauri application");

    /*
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_frontend_request])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
    */
}
