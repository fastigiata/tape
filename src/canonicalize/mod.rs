use serde::{Deserialize, Serialize};

pub mod declaration;
pub mod convert_enigo;
pub mod convert_dq;

/// An **action** is a single event that can be performed by an [actor](../act/struct.Actor.html)
pub struct Action {}

/// A **script** is a sequence of [action](struct.Action.html)s recorded by a [recorder](../rec/struct.Recorder.html) for an [actor](../act/struct.Actor.html) to perform
#[derive(Debug, Serialize, Deserialize)]
pub struct Script {}

impl Script {}


#[cfg(test)]
mod unit_test {
    use crate::canonicalize::declaration::CanonicalButton;
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct Cfg {
        // pub inner: Vec<i32>,
        pub btn: Vec<CanonicalButton>,
    }

    #[test]
    fn tt() {
        // let p = toml::to_string(&Cfg { inner: vec![1, 2, 3] });
        let p = toml::to_string(&Cfg {
            btn: vec![
                CanonicalButton::Left,
                CanonicalButton::Right,
                CanonicalButton::Middle,
                CanonicalButton::Back,
                CanonicalButton::Forward,
                CanonicalButton::Unknown,
            ]
        });

        match p {
            Ok(v) => println!("{}", v),
            Err(e) => println!("{}", e),
        }
    }

    #[test]
    fn encode_test() {
        let btns = vec![
            CanonicalButton::Left,
            CanonicalButton::Right,
            CanonicalButton::Middle,
            CanonicalButton::Back,
            CanonicalButton::Forward,
            CanonicalButton::Unknown,
        ];

        println!("mouse buttons: {:?}", btns);

        match toml::to_string(&btns) {
            Ok(cfg) => println!("mouse buttons: {}", cfg),
            Err(err) => println!("error: {}", err),
        }
    }
}