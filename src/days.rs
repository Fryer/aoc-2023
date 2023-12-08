pub struct Part<'a> {
    pub name: &'a str,
    pub run: fn(&str) -> String
}

impl Part<'_> {
    pub const fn new(name: &str, run: fn(&str) -> String) -> Part {
        return Part { name, run };
    }
}

pub type Day<'a> = &'a [Part<'a>];
