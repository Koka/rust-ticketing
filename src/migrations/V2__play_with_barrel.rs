use barrel::{backend::Pg, types, Migration};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table("drop_this_table", |t| {
        t.add_column("id", types::integer());
        t.add_column("name", types::varchar(255));
    });

    m.make::<Pg>()
}
