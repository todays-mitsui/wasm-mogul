use super::bound_vars::BoundVars;
use std::fmt::Display;

/// ラムダ式や関数定義における識別子を表現する
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn rename(&self, vars: &BoundVars) -> Self {
        let base_name = self.0.to_uppercase();

        if !vars.contains(base_name.as_str()) {
            return Self(base_name);
        }

        let mut name = base_name.clone();
        let mut i = -1;
        while vars.contains(name.as_str()) {
            i += 1;
            name = format!("{}{}", base_name, i);
        }

        Self(name)
    }
}

// ========================================================================== //

impl From<&str> for Identifier {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for Identifier {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for Identifier {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

// ========================================================================== //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_name() {
        let mut set = BoundVars::new();

        set.insert("x");
        set.insert("X");
        set.insert("X0");
        set.insert("X1");
        set.insert("X2");
        set.insert("X3");
        set.insert("X4");
        set.insert("X5");
        set.insert("X7");

        assert_eq!(Identifier::from("x").rename(&set).as_str(), "X6");
        assert_eq!(Identifier::from("X").rename(&set).as_str(), "X6");
        assert_eq!(Identifier::from("y").rename(&set).as_str(), "Y");
    }
}
