use barrel::{backend::Pg, Migration};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.drop_table("drop_this_table");

    m.make::<Pg>()
}
