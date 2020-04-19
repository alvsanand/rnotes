extern crate rnotes_server;

fn main() {
    rnotes_server::init_log();

    rnotes_server::start_server();
}
