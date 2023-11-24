use std::time::{SystemTime, UNIX_EPOCH};

use md5;
use md5::Digest;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

pub fn current_time() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).expect("Clock may have gone backwards").as_secs()
}

pub fn expiry() -> i64 {
    let expiry = current_time() + 3600;
    expiry as i64
}

pub fn hash(secret: String) -> Digest {
    md5::compute(secret)
}

pub fn identity() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();
    format!("{:x}", hash(rand_string))
}
