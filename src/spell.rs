pub struct AllSpells(pub Vec<Spell>);

#[derive(Default, Clone)]
pub struct Spell {
    pub name: String,
    pub law_rating: i8,
    pub reusable: bool,
    pub casting_chance: u8,
    pub cast_range: u8,
    pub tries: u8,
    pub no_line_of_sight_needed: bool,
}

impl Spell {
    pub const fn get_sep(&self) -> &str {
        return "-";
    }
}

