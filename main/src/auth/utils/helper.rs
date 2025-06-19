use rand::Rng;
use regex::Regex;

pub fn is_strict_email(email: &str) -> bool {
    let pattern = r"^[^\s@]+@[^\s@]+\.[^\s@]+$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(email)
}

pub async fn generate_verification_code() -> String {
    format!("{:06}", rand::thread_rng().gen_range(0..1_000_000))
}
