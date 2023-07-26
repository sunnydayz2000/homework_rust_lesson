trait RoleTrait {
    fn who_am_i(&self) -> &str;
}

struct Admin {}
struct User {}
struct Guest {}

impl RoleTrait for Admin {
    fn who_am_i(&self) -> &str {
        "I'm admin"
    }
}
impl RoleTrait for User {
    fn who_am_i(&self) -> &str {
        "I'm user"
    }
}

impl RoleTrait for Guest {
    fn who_am_i(&self) -> &str {
        "I'm guest"
    }
}

pub fn task2() {
    let roles: Vec<Box<dyn RoleTrait>> = vec![
        Box::new(Admin {}),
        Box::new(User {}),
        Box::new(Guest {}),
    ];
    for r in roles {
        println!("{}", r.who_am_i());
    }

}
