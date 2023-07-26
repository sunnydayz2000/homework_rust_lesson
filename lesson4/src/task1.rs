enum Role {
    Admin(Admin),
    User(User),
    Guest(Guest),
}

struct Admin {}
struct User {}
struct Guest {}

impl Admin {
    fn i_m_admin(&self) {
        println!("I'm admin");
    }
}
impl User {
    fn i_m_user(&self) {
        println!("I'm user");
    }
}

impl Guest {
    fn i_m_guest(&self) {
        println!("I'm guest");
    }
}

pub fn task1() {
    let roles: Vec<Role> = vec![
        Role::Admin(Admin {}),
        Role::User(User {}),
        Role::Guest(Guest {}),
    ];
    for r in roles {
        match r {
            Role::Admin(admin) => admin.i_m_admin(),
            Role::User(user) => user.i_m_user(),
            Role::Guest(guest) => guest.i_m_guest(),
        }
    }

}