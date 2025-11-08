use chrono::Utc;
use core::{
    UserState, WalletSession,
    db::{AppDb, TradeLog, UserConfig},
};
use std::path::PathBuf;

fn main() {
    let db = AppDb::open(&PathBuf::from("./data"));

    // save user
    let user_id = "test_user_id";
    let config = UserConfig {
        max_fdv: 80_000.0,
        min_liquidity: 5_000.0,
        budget_per_day: 1.0,
        take_profile_pct: 100.0,
        stop_loss_pct: 40.0,
        max_snipe_sol: 0.1,
    };

    let session = WalletSession {
        pubkey: "test_pubkey".to_string(),
        session_key: vec![0; 32],
        created_at: Utc::now().timestamp() as u64,
        expires_at: Utc::now().timestamp() as u64 + 60 * 60 * 24,
        daily_spent_usdc: 0.0,
        daily_spent_sol: 0.0,
    };
    let mut state = db.get_user(user_id).unwrap_or(UserState {
        config,
        session: Some(session),
        history: Vec::new(),
    });

    // add trade
    state.history.push(TradeLog {
        token: "$DOGGO".to_string(),
        entry_price: 0.000023,
        exit_price: Some(0.000049),
        profit_pct: Some(112.0),
        x402_cost_usdc: 0.0015,
        sol_spent: 0.0001,
        timestamp: Utc::now().timestamp() as u64,
    });

    db.save_user(user_id, &state);

    // update public
    db.update_public_stats(|stats| {
        stats.total_snipe += 1;
        stats.successfull_snipe += 1;
        stats.total_profit_usdc += 0.0015 * 112.0 / 100.0;
    });

    println!("Public stats: {:?}", db.get_public_stats());
    println!("User state: {:?}", db.get_user(user_id));
    println!("Is session active: {}", db.is_session_active(user_id));
    db.disconnect_user(user_id);
    println!("Is session active: {}", db.is_session_active(user_id));
    println!("User state: {:?}", db.get_user(user_id));
}
