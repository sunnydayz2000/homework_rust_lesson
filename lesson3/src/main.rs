mod structs;
mod db;
mod ui;
mod crypto;

use structs::{User, Role};
use ui::{login, show_user_interface};
use db::USERS;

fn main() -> sled::Result<()> {
    // let db = sled::open("my_database")?;
    //
    // let users_tree = db.open_tree("users")?;
    // let courses_tree = db.open_tree("courses")?;
    // let classrooms_tree = db.open_tree("classrooms")?;

    // let admin_user_name = "admin".to_string();
    // let password = "123".to_string();
    // let user = User {
    //     id: sha256(&admin_user_name),
    //     name: "admin".to_string(),
    //     password: sha256(&password),
    //     role: Role::Principal,
    // };

    // // 将用户结构序列化为字节
    // let user_bytes = bincode::serialize(&user).unwrap();
    //
    // // 保存用户到数据库
    // users_tree.insert(&user.id, user_bytes)?;




    if let Some(user) = login(&USERS) {
        show_user_interface(user);
    } else {
        println!("登录失败，用户名或密码错误。");
    }

    // 同样的方式可以用来保存和获取课程和教室

    Ok(())
}
