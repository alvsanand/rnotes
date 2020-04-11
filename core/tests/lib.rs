extern crate dotenv;
extern crate rnotes_core;

use rnotes_core::BDPool;

#[test]
fn test_pool() {
    let pool = BDPool::new().unwrap();
    pool.get().unwrap();
}
