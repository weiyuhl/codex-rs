pub trait ExperimentalApi {
    fn experimental_reason(&self) -> Option<&'static str> {
        None
    }
}

impl<T: ?Sized> ExperimentalApi for T {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExperimentalField {
    pub type_name: &'static str,
    pub field_name: &'static str,
    pub reason: &'static str,
}

pub fn experimental_fields() -> Vec<&'static ExperimentalField> {
    Vec::new()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::ExperimentalApi as ExperimentalApiTrait;
    use pretty_assertions::assert_eq;

    #[allow(dead_code)]
    #[derive(ExperimentalApi)]
    enum EnumVariantShapes {
        Unit,
        Tuple(u8),
        Named {
            value: u8,
        },
        StableTuple(u8),
    }

    #[allow(dead_code)]
    #[derive(ExperimentalApi)]
    struct NestedFieldShape {
        inner: Option<EnumVariantShapes>,
    }

    #[allow(dead_code)]
    #[derive(ExperimentalApi)]
    struct NestedCollectionShape {
        inners: Vec<EnumVariantShapes>,
    }

    #[allow(dead_code)]
    #[derive(ExperimentalApi)]
    struct NestedMapShape {
        inners: HashMap<String, EnumVariantShapes>,
    }

    #[allow(dead_code)]
    #[derive(ExperimentalApi)]
    struct ExperimentalFieldShape {
        optional_collection: Option<Vec<EnumVariantShapes>>,
    }

    #[test]
    fn derive_supports_all_enum_variant_shapes() {
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&EnumVariantShapes::Unit),
            Some("enum/unit")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&EnumVariantShapes::Tuple(1)),
            Some("enum/tuple")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&EnumVariantShapes::Named { value: 1 }),
            Some("enum/named")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&EnumVariantShapes::StableTuple(1)),
            None
        );
    }

    #[test]
    fn derive_supports_nested_experimental_fields() {
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedFieldShape {
                inner: Some(EnumVariantShapes::Named { value: 1 }),
            }),
            Some("enum/named")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedFieldShape { inner: None }),
            None
        );
    }

    #[test]
    fn derive_supports_nested_collections() {
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedCollectionShape {
                inners: vec![
                    EnumVariantShapes::StableTuple(1),
                    EnumVariantShapes::Tuple(2)
                ],
            }),
            Some("enum/tuple")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedCollectionShape {
                inners: Vec::new()
            }),
            None
        );
    }

    #[test]
    fn derive_supports_nested_maps() {
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedMapShape {
                inners: HashMap::from([(
                    "default".to_string(),
                    EnumVariantShapes::Named { value: 1 },
                )]),
            }),
            Some("enum/named")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&NestedMapShape {
                inners: HashMap::new(),
            }),
            None
        );
    }

    #[test]
    fn derive_marks_optional_experimental_fields_when_some() {
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&ExperimentalFieldShape {
                optional_collection: Some(Vec::new()),
            }),
            Some("field/optionalCollection")
        );
        assert_eq!(
            ExperimentalApiTrait::experimental_reason(&ExperimentalFieldShape {
                optional_collection: None,
            }),
            None
        );
    }
}
