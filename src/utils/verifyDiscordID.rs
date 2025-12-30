use once_cell::sync::Lazy;
use regex::Regex;

static DISCORD_ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{18,19}$").unwrap());

pub fn verify_discord_id(id: impl ToString) -> bool {
    let id_str = id.to_string();
    DISCORD_ID_REGEX.is_match(&id_str)
}
