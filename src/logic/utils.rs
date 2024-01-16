use regex::Regex;
use tracing::{event, Level};

pub fn at_to_snowflake<S: Into<String>>(who: S) -> Option<i64>{
    let who = who.into();
    let re = Regex::new(r"\b(?P<snowflake>[0-9]{17,19})\b").unwrap();
    let Some(caps) = re.captures(&who) else {
        event!(Level::WARN, "No snowflake found in {:?}", &who);
        return None;
    };
    Some(caps["snowflake"].parse::<u64>().unwrap() as i64)
}


#[test]
fn test_at_to_snowflake(){
    let test_snowflakes: Vec<(&str, i64)> = vec![
        ("95487713607692288", 95487713607692288),
        ("169536101357191168",169536101357191168)
    ];
    for (t,e) in test_snowflakes{
        assert_eq!(at_to_snowflake(format!("<@{t}>")), Some(e));
        assert_eq!(at_to_snowflake(format!("{t}")), Some(e));
        assert_eq!(at_to_snowflake(format!(" {t} ")), Some(e));
        assert_eq!(at_to_snowflake(format!(" {t}")), Some(e));
        assert_eq!(at_to_snowflake(format!("{t} ")), Some(e));
        assert_eq!(at_to_snowflake(format!("<@{t}321>")), None);
        assert_eq!(at_to_snowflake(format!("{t}321")), None);
    }
}