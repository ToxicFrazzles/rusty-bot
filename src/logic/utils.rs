use regex::Regex;

pub fn at_to_snowflake<S: Into<String>>(who: S) -> Option<i64>{
    let who = who.into();
    let re = Regex::new(r"\b(?P<snowflake>[0-9]{18})\b").unwrap();
    let Some(caps) = re.captures(&who) else {
        return None;
    };
    Some(caps["snowflake"].parse::<u64>().unwrap() as i64)
}


#[test]
fn test_at_to_snowflake(){
    assert_eq!(at_to_snowflake("<@169536101357191168>"), Some(169536101357191168));
    assert_eq!(at_to_snowflake("169536101357191168"), Some(169536101357191168));
    assert_eq!(at_to_snowflake(" 169536101357191168 "), Some(169536101357191168));
    assert_eq!(at_to_snowflake(" 169536101357191168"), Some(169536101357191168));
    assert_eq!(at_to_snowflake("169536101357191168 "), Some(169536101357191168));
    assert_eq!(at_to_snowflake("<@1695361013571911683>"), None);
    assert_eq!(at_to_snowflake("1695361013571911681"), None);
}