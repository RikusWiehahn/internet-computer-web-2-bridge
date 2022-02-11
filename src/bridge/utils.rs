
use ic_cdk::export::Principal;
use uuid::{Builder, Variant, Version};

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
