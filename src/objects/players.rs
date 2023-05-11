use crate::constants::privileges::{ClientPrivileges, Privileges};

#[derive(Debug, Copy, Clone)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub safe_name: String,
    pub privileges: Privileges,
    pub client_priv: Option<ClientPrivileges>,
}

impl Player {
    pub fn new(id: i32, username: String, safe_name: String, privileges: Privileges) -> Self {
        Self {
            id,
            username,
            safe_name,
            privileges,
            client_priv: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlayerList {
    pub players: Vec<Player>,
}
impl PlayerList {
    pub fn new() {
        Self {
            players: Vec::new(),
        }
    }
    pub fn test() -> i8 {
        return 0
    }
}
