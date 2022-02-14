
use ic_cdk::{export::Principal};
use uuid::{Builder, Variant, Version};
use crate::keys::BRIDGE_ACCESS_KEY;


/*          
    Generate a random UUID
*/
pub async fn generate_uuid() -> Result<String, String> {
  let res = ic_cdk::call(Principal::management_canister(), "raw_rand", ()).await;
  if res.is_err() {
      return Err("Failed to generate UUID".to_string());
  }
  let (bytes,): (Vec<u8>,) = res.unwrap();
  let mut random_bytes: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
  for i in 0..16 {
      random_bytes[i] = bytes[i];
  }
  let uuid = Builder::from_bytes(random_bytes)
      .set_variant(Variant::RFC4122)
      .set_version(Version::Random)
      .build();
  Ok(uuid.to_string())
}

pub fn verify_access_key(access_key: &str) -> Result<(), String> {
    if access_key == BRIDGE_ACCESS_KEY {
        return Ok(());
    }
    return Err("Access key is not valid".to_string());
}