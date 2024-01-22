use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize, ser};

#[derive(Debug, Clone, PartialEq)]
pub struct Snowflake{
    snowflake: String
}

impl<'de> Deserialize<'de> for Snowflake{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: serde::Deserializer<'de> {
        let object_id = ObjectId::deserialize(deserializer)?;
        Ok(Snowflake::from(object_id))
    }
}

impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer {
        let int_flake: u64 = self.snowflake.parse::<u64>().unwrap();
        let hex_flake: String = format!("{:024X}", int_flake);
        match ObjectId::parse_str(&hex_flake) {
            Ok(oid) => oid.serialize(serializer),
            Err(why) => Err(ser::Error::custom(format!(
                "cannot convert {hex_flake} to ObjectId: {why}"
            ))),
        }
    }
}


impl From<ObjectId> for Snowflake{
    fn from(val: ObjectId) -> Snowflake{
        let int_flake = u64::from_str_radix(&val.to_string(), 16).unwrap();

        Snowflake{
            snowflake: format!("{int_flake}"),
        }
    }
}

impl From<String> for Snowflake{
    fn from(value: String) -> Self {
        Snowflake{
            snowflake: value
        }
    }
}

impl From<&str> for Snowflake{
    fn from(value: &str) -> Self {
        Snowflake{
            snowflake: value.to_string()
        }
    }
}

impl Into<String> for Snowflake{
    fn into(self) -> String {
        self.snowflake
    }
}

// impl TryInto<ObjectId> for Snowflake{
//     fn try_into(self) -> Result<ObjectId, Self::Error> {
//         let int_flake: u64 = self.snowflake.parse::<u64>().unwrap();
//         let hex_flake: String = format!("{:024X}", int_flake);
//         ObjectId::parse_str(&hex_flake)
//     }

//     type Error = oid::Error;
// }

impl Into<ObjectId> for Snowflake{
    fn into(self) -> ObjectId {
        let int_flake: u64 = self.snowflake.parse::<u64>().unwrap();
        let hex_flake: String = format!("{:024X}", int_flake);
        ObjectId::parse_str(&hex_flake).unwrap()
    }
}



#[test]
fn test_snowflake_conversion(){
    let str_flake = "169536101357191168";
    let num1 = Snowflake::from(str_flake);
    let num2: String = num1.clone().into();
    assert_eq!(str_flake, num2);

    let num3: ObjectId = num1.clone().into();
    let num4: Snowflake = Snowflake::from(num3);
    assert_eq!(&num1, &num4);
}