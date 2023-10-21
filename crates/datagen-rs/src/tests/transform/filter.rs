use crate::generate::generated_schema::GeneratedSchema;
use crate::generate::generated_schema::IntoRandom;
use crate::generate_schema;
use crate::schema::any_value::AnyValue;
use crate::schema::object::Object;
use crate::schema::string::StringSchema;
use crate::schema::transform::Transform;
use crate::tests::util::root_schema;
use crate::transform::filter::{FilterTransform, FilterTransformOp};
use serde_json::json;

fn create_filter(operator: FilterTransformOp, other: GeneratedSchema) -> Transform {
    Transform::Filter(FilterTransform {
        operator,
        other,
        field: None,
    })
}

#[test]
fn test_simple_filter() {
    let schema = root_schema();

    let mut str = StringSchema::Constant {
        value: "test".to_string(),
        transform: Some(vec![create_filter(
            FilterTransformOp::Equals,
            GeneratedSchema::String("test".to_string()),
        )]),
    };
    let generated = str.clone().into_random(schema.clone()).unwrap();

    assert_eq!(
        generated,
        GeneratedSchema::String("test".to_string()).into()
    );

    if let StringSchema::Constant { transform, .. } = &mut str {
        transform.replace(vec![create_filter(
            FilterTransformOp::NotEquals,
            GeneratedSchema::String("test".to_string()),
        )]);
    }

    let generated = str.into_random(schema).unwrap();
    assert_eq!(generated, GeneratedSchema::None.into());
}

#[test]
fn test_filter_reference() {
    let schema = root_schema();

    let mut obj = Object {
        properties: vec![("test".to_string(), AnyValue::String("test".to_string()))]
            .into_iter()
            .collect(),
        transform: Some(vec![create_filter(
            FilterTransformOp::Equals,
            GeneratedSchema::String("test".to_string()),
        )]),
    };

    let generated = obj.clone().into_random(schema.clone()).unwrap();

    assert_eq!(
        generated,
        GeneratedSchema::Object(
            vec![(
                "test".to_string(),
                GeneratedSchema::String("test".to_string()).into()
            )]
            .into_iter()
            .collect()
        )
        .into()
    );

    obj.transform.replace(vec![create_filter(
        FilterTransformOp::NotEquals,
        GeneratedSchema::String("test".to_string()),
    )]);
    let generated = obj.into_random(schema).unwrap();

    assert_eq!(
        generated,
        GeneratedSchema::Object(
            vec![("test".to_string(), GeneratedSchema::None.into())]
                .into_iter()
                .collect()
        )
        .into()
    );
}

#[test]
fn test_filter_complex_reference() {
    let generated = generate_schema!({
        "type": "array",
        "length": {
            "value": 3
        },
        "items": {
            "type": "object",
            "properties": {
                "id": {
                    "type": "counter"
                },
                "related": {
                    "type": "array",
                    "length": {
                        "value": 3
                    },
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": {
                                "type": "string",
                                "value": "relatedId"
                            },
                            "relatedItem": {
                                "type": "reference",
                                "reference": "id",
                                "except": [
                                    "ref:../../related.relatedItem",
                                    "ref:../../id"
                                ]
                            }
                        },
                        "transform": [
                            {
                                "type": "filter",
                                "field": "ref:./relatedItem",
                                "operator": "notEquals",
                                "other": null
                            }
                        ]
                    },
                    "transform": [
                        {
                            "type": "sort",
                            "by": "relatedItem"
                        }
                    ]
                }
            }
        }
    })
    .unwrap();

    let res = json!([
        {
            "id": 0,
            "related": [
                null,
                null,
                null
            ]
        },
        {
            "id": 1,
            "related": [
                {
                    "id": "relatedId",
                    "relatedItem": 0
                },
                null,
                null,
            ]
        },
        {
            "id": 2,
            "related": [
                {
                    "id": "relatedId",
                    "relatedItem": 0
                },
                {
                    "id": "relatedId",
                    "relatedItem": 1
                },
                null,
            ]
        }
    ]);

    assert_eq!(
        serde_json::to_string_pretty(&generated).unwrap(),
        serde_json::to_string_pretty(&res).unwrap()
    );
}
