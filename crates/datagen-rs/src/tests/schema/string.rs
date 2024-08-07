use crate::assert_enum;
use crate::generate::generated_schema::generate::IntoGenerated;
use crate::generate::generated_schema::GeneratedSchema;
use crate::schema::string::StringGenerator;
use crate::tests::util::root_schema;
use crate::validation::validate::Validate;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike};

#[test]
fn test_date_time_any() {
    let schema = StringGenerator::DateTime {
        format: None,
        from: None,
        to: None,
    };

    let generated = schema.into_generated(root_schema()).unwrap();
    let string = assert_enum!(generated, GeneratedSchema::String);
    assert_eq!(string.len(), 20);
}

#[test]
fn test_date_time_between() {
    let schema = StringGenerator::DateTime {
        format: None,
        from: Some("2020-12-19T16:39:01Z".to_string()),
        to: Some("2020-12-19T16:41:59Z".to_string()),
    };

    let generated = schema.into_generated(root_schema()).unwrap();
    let string = assert_enum!(generated, GeneratedSchema::String);
    let parsed = DateTime::parse_from_rfc3339(&string).unwrap();
    assert_eq!(parsed.year(), 2020);
    assert_eq!(parsed.month(), 12);
    assert_eq!(parsed.day(), 19);
    assert_eq!(parsed.hour(), 16);
}

#[test]
fn test_date_time_between_invalid() {
    let schema = StringGenerator::DateTime {
        format: None,
        from: Some("2020-12-19T16:39:01Z".to_string()),
        to: Some("2020-12-19T16:39:01Z".to_string()),
    };

    let generated = schema.into_generated(root_schema());
    assert!(generated.is_err());
    assert_eq!(
        generated.unwrap_err().to_string(),
        "'from' date must be at least one minute before the 'to' date"
    );
}

#[test]
fn test_date_time_from() {
    let schema = StringGenerator::DateTime {
        format: None,
        from: Some("2020-12-19T16:39:01Z".to_string()),
        to: None,
    };

    let generated = schema.into_generated(root_schema()).unwrap();
    let string = assert_enum!(generated, GeneratedSchema::String);
    let parsed = DateTime::parse_from_rfc3339(&string).unwrap();
    assert!(parsed > DateTime::parse_from_rfc3339("2020-12-19T16:39:00Z").unwrap());
}

#[test]
fn test_date_time_to() {
    let schema = StringGenerator::DateTime {
        format: None,
        from: None,
        to: Some("2020-12-19T16:39:01Z".to_string()),
    };

    let generated = schema.into_generated(root_schema()).unwrap();
    let string = assert_enum!(generated, GeneratedSchema::String);
    let parsed = DateTime::parse_from_rfc3339(&string).unwrap();
    assert!(parsed < DateTime::parse_from_rfc3339("2020-12-19T16:39:02Z").unwrap());
}

#[test]
fn test_date_time_format() {
    let schema = StringGenerator::DateTime {
        format: Some("%Y-%m-%d %H:%M:%S".to_string()),
        from: Some("2020-12-19T16:39:01Z".to_string()),
        to: Some("2020-12-19T16:40:01Z".to_string()),
    };

    let generated = schema.into_generated(root_schema()).unwrap();
    let string = assert_enum!(generated, GeneratedSchema::String);
    let parsed = NaiveDateTime::parse_from_str(&string, "%Y-%m-%d %H:%M:%S").unwrap();
    assert_eq!(parsed.year(), 2020);
    assert_eq!(parsed.month(), 12);
    assert_eq!(parsed.day(), 19);
    assert_eq!(parsed.hour(), 16);
}

#[test]
fn test_validate_date_time_format_valid() {
    let res = StringGenerator::DateTime {
        from: None,
        to: None,
        format: Some("%Y-%m-%d %H:%M:%S".to_string()),
    }
    .validate_root();

    assert!(res.is_ok());
}

#[test]
fn test_validate_date_time_from_to_valid() {
    let res = StringGenerator::DateTime {
        from: Some("2020-12 16:39:01".to_string()),
        to: Some("2020-12 16:39:01".to_string()),
        format: None,
    }
    .validate_root();

    assert!(res.is_err());
    let err = res.unwrap_err();
    assert_eq!(err.len(), 2);
    assert_eq!(
        err[0].to_string(),
        r#"from must be a valid RFC 3339 date at from
Caused by:
  input contains invalid characters
Invalid value was:
  "2020-12 16:39:01""#
    );
    assert_eq!(
        err[1].to_string(),
        r#"to must be a valid RFC 3339 date at to
Caused by:
  input contains invalid characters
Invalid value was:
  "2020-12 16:39:01""#
    );
}
