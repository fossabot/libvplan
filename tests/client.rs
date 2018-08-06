extern crate libvplan;

use libvplan::Client;

#[test]
fn client_creation() {
    let _client = Client::new("username", "password");
    assert!(true);
}
