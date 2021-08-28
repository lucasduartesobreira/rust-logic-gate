trait Boolean {
    fn boolean<'a>(&'a self, x: &'a dyn Boolean, y: &'a dyn Boolean) -> &'a dyn Boolean;
    fn get_value(&self) -> bool;
}

pub struct True(bool);
pub struct False(bool);

impl Boolean for True {
    fn boolean<'a>(&'a self, x: &'a dyn Boolean, _: &'a dyn Boolean) -> &'a dyn Boolean {
        x
    }

    fn get_value(&self) -> bool {
        self.0
    }
}

impl Boolean for False {
    fn boolean<'a>(&'a self, _: &'a dyn Boolean, y: &'a dyn Boolean) -> &'a dyn Boolean {
        y
    }

    fn get_value(&self) -> bool {
        self.0
    }
}

#[allow(dead_code)]
fn and<'a>(x: &'a dyn Boolean, y: &'a dyn Boolean) -> &'a dyn Boolean {
    x.boolean(y, &False(false))
}

#[allow(dead_code)]
fn or<'a>(x: &'a dyn Boolean, y: &'a dyn Boolean) -> &'a dyn Boolean {
    x.boolean(&True(true), y)
}

#[allow(dead_code)]
fn not(x: &dyn Boolean) -> &dyn Boolean {
    x.boolean(&False(false), &True(true))
}

#[cfg(test)]
mod tests {
    use crate::{and, not, or, False, True};

    #[test]
    fn test_and_port() {
        let t = True(true);
        let f = False(false);

        assert!(and(&t, &t).get_value(), "true and true should be true");
        assert!(!and(&f, &t).get_value(), "false and true should be false");
        assert!(!and(&f, &f).get_value(), "false and false should be false");
    }

    #[test]
    fn test_or_port() {
        let t = True(true);
        let f = False(false);

        assert!(or(&t, &t).get_value(), "true or true should be true");
        assert!(or(&f, &t).get_value(), "false or true should be true");
        assert!(!or(&f, &f).get_value(), "false and false should be false");
    }

    #[test]
    fn test_not_port() {
        let t = True(true);
        let f = False(false);

        assert!(!not(&t).get_value(), "not true should be false");
        assert!(not(&f).get_value(), "not false should be true");
    }
}
