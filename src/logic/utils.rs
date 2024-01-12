use regex::Regex;

pub fn at_to_snowflake<S: Into<String>>(who: S) -> Option<String>{
    let who = who.into();
    let re = Regex::new(r"\b(?P<snowflake>[0-9]{18})\b").unwrap();
    let Some(caps) = re.captures(&who) else {
        return None;
    };
    Some(caps["snowflake"].to_string())
}


#[test]
fn test_at_to_snowflake(){
    assert_eq!(at_to_snowflake("<@169536101357191168>"), Some("169536101357191168".to_string()));
    assert_eq!(at_to_snowflake("169536101357191168"), Some("169536101357191168".to_string()));
    assert_eq!(at_to_snowflake(" 169536101357191168 "), Some("169536101357191168".to_string()));
    assert_eq!(at_to_snowflake(" 169536101357191168"), Some("169536101357191168".to_string()));
    assert_eq!(at_to_snowflake("169536101357191168 "), Some("169536101357191168".to_string()));
    assert_eq!(at_to_snowflake("<@1695361013571911683>"), None);
    assert_eq!(at_to_snowflake("1695361013571911681"), None);
}