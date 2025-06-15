use std::sync::Arc;
use tokio::sync::Mutex;

use crate::user_chats::user_chats::UserChats;

pub struct AppState {
    //pub shared_app_daemon_proxy: Mutex<DaemonProxy>,
    pub shared_user_chats: Arc<Mutex<UserChats>>,
}
