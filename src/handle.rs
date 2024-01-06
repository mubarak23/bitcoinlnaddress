use bitcoin::{
  network::constants::Network,
  util::key::PrivateKey,
  util::address::{Address, P2PKH, P2SH, P2WPKH, P2WSH},
  consensus::encode::deserialize,
};

fn generate_random_address() -> String {
  let key_pair = PrivateKey::new_random().unwrap();
  let address = P2PKH::from_pubkey(&key_pair.public_key(Network::Bitcoin).to_bytes()).address(Network::Bitcoin);
  address.to_string()
}
