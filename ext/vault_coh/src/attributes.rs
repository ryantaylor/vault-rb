use std::collections::HashMap;
use vault::commands::{BuildSquad, SelectBattlegroup, Unknown};
use vault::Message;

pub trait AttributesExt {
    fn attributes(&self) -> HashMap<&str, Option<&str>>;
}

impl AttributesExt for BuildSquad {
    fn attributes(&self) -> HashMap<&str, Option<&str>> {
        HashMap::from([
            ("tick", None),
            ("pbgid", None)
        ])
    }
}

impl AttributesExt for SelectBattlegroup {
    fn attributes(&self) -> HashMap<&str, Option<&str>> {
        HashMap::from([
            ("tick", None),
            ("pbgid", None)
        ])
    }
}

impl AttributesExt for Unknown {
    fn attributes(&self) -> HashMap<&str, Option<&str>> {
        HashMap::from([
            ("tick", None),
            ("action_type", None)
        ])
    }
}

impl AttributesExt for Message {
    fn attributes(&self) -> HashMap<&str, Option<&str>> {
        HashMap::from([
            ("tick", None),
            ("message", None)
        ])
    }
}
