use mongodb::bson::oid::ObjectId;
use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};

/// Deserializes a hex string from an ObjectId.
pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let object_id = ObjectId::deserialize(deserializer)?;
    Ok(object_id.to_string())
}

/// Serializes a hex string as an ObjectId.
pub fn serialize<S: Serializer>(val: &str, serializer: S) -> Result<S::Ok, S::Error> {
    let int_flake: u64 = val.to_string().parse::<u64>().unwrap();
    let hex_flake: String = format!("{:024X}", int_flake);
    match ObjectId::parse_str(&hex_flake) {
        Ok(oid) => oid.serialize(serializer),
        Err(why) => Err(ser::Error::custom(format!(
            "cannot convert {hex_flake} to ObjectId: {why}"
        ))),
    }
}


pub fn snowflake_to_oid(val: &str) -> Option<ObjectId>{
    let int_flake: u64 = val.to_string().parse::<u64>().unwrap();
    let hex_flake: String = format!("{:024X}", int_flake);
    match ObjectId::parse_str(&hex_flake) {
        Ok(oid) => Some(oid),
        Err(_) => None
    }
}