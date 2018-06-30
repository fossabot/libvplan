extern crate libvplan;

use libvplan::Client;

#[test]
fn client_creation() {
    let client = Client::new("username", "password");
    assert_eq!(client.is_ok(), true);
}
