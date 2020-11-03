#[macro_export]
macro_rules! define_de_dyntrait_json {
    ($name:ident, $trait:ident, [$($struct:ty),*]) => {
        fn $name(json: &str) -> Result<Box<dyn $trait>, std::io::Error> {
            let ret: Box<dyn $trait> = serde_json::from_str(json)?;
            let tag_name = ret.typetag_name();
            let mut ok = false;
            $(
                if <$struct as $trait>::typetag_name(&Default::default()) == tag_name {
                    ok = true;
                }
            )*
            if ok {
                Ok(ret)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("unexpected variant `{}`", tag_name),
                ))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[typetag::serde(tag = "type")]
    trait Trait {}

    #[derive(Default, Serialize, Deserialize)]
    struct A;
    #[typetag::serde]
    impl Trait for A {}

    #[derive(Default, Serialize, Deserialize)]
    struct B {
        id: u32,
    }
    #[typetag::serde]
    impl Trait for B {}

    #[test]
    fn it_deserializes_boxed_type_a() {
        define_de_dyntrait_json!(de, Trait, [A]);
        let my_box = Box::new(A {}) as Box<dyn Trait>;
        let json = serde_json::to_string(&my_box).unwrap();
        let _ = de(&json).unwrap();
    }

    #[test]
    #[should_panic]
    fn it_cannot_deserialize_boxed_type_b() {
        define_de_dyntrait_json!(de, Trait, [A]);
        let my_box = Box::new(B { id: 3 }) as Box<dyn Trait>;
        let json = serde_json::to_string(&my_box).unwrap();
        let _ = de(&json).unwrap();
    }

    #[test]
    fn it_deserializes_boxed_type_b() {
        define_de_dyntrait_json!(de, Trait, [A, B]);
        let my_box = Box::new(B { id: 3 }) as Box<dyn Trait>;
        let json = serde_json::to_string(&my_box).unwrap();
        let _ = de(&json).unwrap();
    }
}
