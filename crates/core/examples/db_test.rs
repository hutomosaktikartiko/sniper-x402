use chrono::Utc;
use core::{
    UserState,
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
    };
    let mut state = db.get_user(user_id).unwrap_or(UserState {
        config,
        history: Vec::new(),
        daily_spent: 0.0,
    });

    // add trade
    state.history.push(TradeLog {
        token: "$DOGGO".to_string(),
        entry_price: 0.000023,
        exit_price: Some(0.000049),
        profit_pct: Some(112.0),
        x402_cost_usdc: 0.0015,
        timestamp: Utc::now().timestamp() as u64,
    });
    state.daily_spent += 0.0015;

    db.save_user(user_id, &state);

    // update public
    db.update_public_stats(|stats| {
        stats.total_snipe += 1;
        stats.successfull_snipe += 1;
        stats.total_profit_usdc += 0.0015 * 112.0 / 100.0;
    });

    println!("Public stats: {:?}", db.get_public_stats());
    println!("User state: {:?}", db.get_user(user_id));
}
