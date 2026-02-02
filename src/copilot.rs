use rand::Rng;

pub const COMB_CHARS: &[char] = &[
    '\u{0300}', '\u{0301}', '\u{0302}', '\u{0303}', '\u{0304}', '\u{0305}', '\u{0306}', '\u{0307}',
    '\u{0308}', '\u{0309}', '\u{030A}', '\u{030B}', '\u{030C}', '\u{030D}', '\u{030E}', '\u{030F}',
    '\u{0310}', '\u{0311}', '\u{0312}', '\u{0313}', '\u{0314}', '\u{0315}', '\u{0316}', '\u{0317}',
    '\u{0318}', '\u{0319}', '\u{031A}', '\u{031B}', '\u{031C}', '\u{031D}', '\u{031E}', '\u{031F}',
    '\u{0320}', '\u{0321}', '\u{0322}', '\u{0323}', '\u{0324}', '\u{0325}', '\u{0326}', '\u{0327}',
    '\u{0328}', '\u{0329}',
];

/// Allows self to hallucinate
pub trait Hallucinate {
    fn hallucinate(self) -> Self;
}

/// Allows an hallucination to be created from self
pub trait ToHallucination {
    fn to_hallucination(&self) -> String;
}

impl Hallucinate for String {
    fn hallucinate(mut self) -> String {
        let mut chars: Vec<char> = self.chars().collect();
        let mut rng = rand::rng();

        for i in (0..chars.len()).rev() {
            let comb_count = rng.random_range(3..=6);
            for _ in 0..comb_count {
                let comb_char = COMB_CHARS[rng.random_range(0..COMB_CHARS.len())];
                chars.insert(i, comb_char);
            }
        }

        self.clear();
        self.extend(chars);

        self
    }
}

impl ToHallucination for String {
    fn to_hallucination(&self) -> String {
        self.clone().hallucinate()
    }
}

impl ToHallucination for &str {
    fn to_hallucination(&self) -> String {
        self.to_string().hallucinate()
    }
}
