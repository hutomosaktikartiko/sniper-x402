use bincode::{Decode, Encode, config, config::Configuration, decode_from_slice, encode_to_vec};
use chrono::Utc;
use sled::{Db, IVec};
use std::path::Path;

const BINCODE_CONFIG: Configuration = config::standard();

#[derive(Encode, Decode, Clone, Debug)]
pub struct UserConfig {
    pub max_fdv: f64,
    pub min_liquidity: f64,
    pub budget_per_day: f64,
    pub take_profile_pct: f64,
    pub stop_loss_pct: f64,
    pub max_snipe_sol: f64,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct TradeLog {
    pub token: String,
    pub entry_price: f64,
    pub exit_price: Option<f64>,
    pub profit_pct: Option<f64>,
    pub x402_cost_usdc: f64,
    pub sol_spent: f64,
    pub timestamp: u64,
}

#[derive(Encode, Decode, Clone, Debug)]
pub struct WalletSession {
    pub pubkey: String,
    pub session_key: Vec<u8>,
    pub created_at: u64,
    pub expires_at: u64,
    pub daily_spent_usdc: f64,
    pub daily_spent_sol: f64,
}

#[derive(Encode, Decode, Debug)]
pub struct UserState {
    pub config: UserConfig,
    pub session: Option<WalletSession>,
    pub history: Vec<TradeLog>,
}

#[derive(Encode, Decode, Default, Debug)]
pub struct PublicStats {
    pub total_users: u64,
    pub active_sessions: u64,
    pub total_snipe: u64,
    pub successfull_snipe: u64,
    pub total_profit_usdc: f64,
}

pub struct AppDb {
    user_dbs: Db,
    public_db: Db,
}

impl AppDb {
    pub fn open(data_dir: &Path) -> Self {
        let public_db_path = data_dir.join("public.db");
        let public_db = sled::open(public_db_path).expect("Failed to open public DB");

        let user_dbs_path = data_dir.join("users_dbs");
        std::fs::create_dir_all(&user_dbs_path).expect("Failed to create user DBs directory");
        let user_dbs =
            sled::open(user_dbs_path.join("index")).expect("Failed to open user DBs index");

        Self {
            user_dbs,
            public_db,
        }
    }

    pub fn get_user(&self, user_id: &str) -> Option<UserState> {
        self.user_dbs.get(user_id).ok()?.map(|ivec| {
            let (state, _): (UserState, _) =
                decode_from_slice(&ivec, BINCODE_CONFIG).expect("Failed to decode user state");
            state
        })
    }

    pub fn save_user(&self, user_id: &str, state: &UserState) -> Option<IVec> {
        let encoded = encode_to_vec(state, BINCODE_CONFIG).expect("Failed to encode user state");
        self.user_dbs
            .insert(user_id, encoded)
            .expect("Failed to save user state")
    }

    pub fn is_session_active(&self, user_id: &str) -> bool {
        self.get_user(user_id)
            .and_then(|u| u.session)
            .map(|s| Utc::now().timestamp() as u64 <= s.expires_at)
            .unwrap_or(false)
    }

    pub fn disconnect_user(&self, user_id: &str) {
        if let Some(mut state) = self.get_user(user_id) {
            state.session = None;
            self.save_user(user_id, &state);
        }
    }

    pub fn get_public_stats(&self) -> PublicStats {
        self.public_db
            .get("stats")
            .ok()
            .flatten()
            .map(|ivec| {
                let (stats, _): (PublicStats, _) = decode_from_slice(&ivec, BINCODE_CONFIG)
                    .expect("Failed to decode public stats");
                stats
            })
            .unwrap_or_default()
    }

    pub fn update_public_stats(&self, update: impl FnOnce(&mut PublicStats)) {
        let mut stats = self.get_public_stats();
        update(&mut stats);
        let encoded = encode_to_vec(&stats, BINCODE_CONFIG).expect("Failed to encode public stats");
        self.public_db
            .insert("stats", encoded)
            .expect("Failed to update public stats");
    }
}
