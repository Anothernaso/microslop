//! Tools to turn text into slop, just like MicroSlop Wandoze.

use rand::Rng;

/// Allows self to turn into slop.
pub trait Slopify {
    fn slopify(self) -> Self;
}

/// Allows slop to be created from self.
pub trait ToSlop {
    fn to_slop(&self) -> String;
}

impl Slopify for String {
    fn slopify(mut self) -> String {
        let mut chars: Vec<char> = self.chars().collect();
        let mut rng = rand::rng();

        for i in 0..chars.len() {
            let chance: f32 = rng.random();

            if chance < 0.35 {
                chars[i] = chars[i].to_ascii_lowercase()
            } else {
                chars[i] = chars[i].to_ascii_uppercase();
            }
        }

        self.clear();
        self.extend(chars);

        self
    }
}

impl ToSlop for String {
    fn to_slop(&self) -> String {
        self.clone().slopify()
    }
}

impl ToSlop for &str {
    fn to_slop(&self) -> String {
        self.to_string().slopify()
    }
}
