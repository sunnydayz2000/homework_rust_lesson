macro_rules! role_structs {
    ( $( $role:ident ),* ) => {
        $(
            struct $role {
                name: String,
            }

            impl $role {
                fn new(name: &str) -> $role {
                    $role {
                        name: name.to_string(),
                    }
                }

                fn role(&self) -> String {
                    format!("{} is a {}", self.name, stringify!($role))
                }
            }
        )*
    };
}

macro_rules! role_instances {
    ( $( $role:ident, $name:expr ),* ) => {
        $(
            let role = format!("{}_{}", stringify!($role), $name.replace(" ", "_"));
            let mut role_instance: $role = $role::new($name);
            println!("{}", role_instance.role());
        )*
    };
}

fn main() {
    role_structs!(Principal, Teacher, Student);

    role_instances!(
        Principal, "John",
        Principal, "Mary",
        Teacher, "Alice",
        Teacher, "Bob",
        Student, "Xiao Ming",
        Student, "Xiao Hong"
    );
}

