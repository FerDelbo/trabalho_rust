pub struct Text(pub String);

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text(value.to_string())
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text(value)
    }
}


pub struct Integer(pub i32);

impl From<i32> for Integer {
    fn from(value: i32) -> Self {
        Integer(value)
    }
}

pub struct Boolean(pub bool);

impl From<bool> for Boolean {
    fn from(value: bool) -> Self {
        Boolean(value)
    }
}

pub struct Decimal(pub f64);

impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        Decimal(value)
    }
}

pub struct Date(pub String);

impl From<&str> for Date {
    fn from(value: &str) -> Self {
        Date(value.to_string())
    }
}

impl From<String> for Date {
    fn from(value: String) -> Self {
        Date(value)
    }
}