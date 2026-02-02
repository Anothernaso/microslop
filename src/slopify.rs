//! Tools to turn text into slop, just like what MicroSlop are doing to Wandoze 11.

use rand::Rng;

/// Allows self to turn into slop.
pub trait Slopify {
    /// Turns self into slop, consuming self.
    ///
    /// # Examples
    ///
    /// ```rust
    /// String::new("MicroSlop Wandoze").slopify();
    /// ```
    ///
    fn slopify(self) -> Self;
}

/// Allows slop to be created from &self.
pub trait ToSlop {
    /// Crates sloppy version based on &self.
    ///
    /// # Examples
    ///
    /// ```rust
    /// "MicroSlop Wandoze".to_slop();
    /// ```
    ///
    fn to_slop(&self) -> String;
}

impl Slopify for String {
    /// Turns the String into slop, consuming itself.
    ///
    /// # Examples
    ///
    /// ```rust
    /// String::new("MicroSlop Wandoze").slopify();
    /// ```
    ///
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
    /// Creates a new piece of slop from this String
    ///
    /// # Examples
    ///
    /// ```rust
    /// let str = "MicroSlop Wandoze".to_owned();
    /// let sloppy_str = str.to_slop(); // This doesn't move str
    ///
    /// println!("{}", str); // str is still valid
    /// ```
    ///
    fn to_slop(&self) -> String {
        self.clone().slopify()
    }
}

impl ToSlop for &str {
    /// Creates a new piece of slop from this &str
    ///
    /// # Examples
    ///
    /// ```rust
    /// let str = "MicroSlop Wandoze";
    /// let sloppy_str = str.to_slop(); // This returns an owned String
    ///
    /// println!("{}", str); // str is still valid
    /// ```
    ///
    fn to_slop(&self) -> String {
        self.to_string().slopify()
    }
}
