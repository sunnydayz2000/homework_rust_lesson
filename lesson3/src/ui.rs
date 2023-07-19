use std::collections::HashMap;
use std::io::{self, Write};

use crate::structs::{User, Role, Classroom, Course};
use crate::crypto::sha256;
use crate::db;
pub fn login(system: &sled::Tree) -> Option<User> {
    let mut username = String::new();
    let mut password = String::new();

    print!("请输入用户名: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();

    print!("请输入密码: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    // 移除输入的末尾的换行符
    let username = username.trim_end();
    let password = password.trim_end();

    let user_id = sha256(username);


    // 从sled数据库中获取用户
    match system.get(&user_id) {
        Ok(Some(user_bytes)) => {
            // 反序列化用户结构
            let user: User = bincode::deserialize(&user_bytes).unwrap();
            //println!("用户信息：{:?}", user);
            // 检查密码是否匹配
            if user.authenticate(password) {
                Some(user)
            } else {
                println!("登录失败，密码错误。");
                None
            }
        }
        Ok(None) => {
            println!("无此用户名。");
            None
        }
        Err(e) => {
            println!("查询用户失败，错误信息：{}", e);
            None
        }
    }
}
pub fn show_user_interface(user: User) {
    match user.role {
        Role::Student => {
            // 显示学生的功能界面
            println!("欢迎学生 {} 登录系统！", user.name);
            println!("请选择操作：");
            println!("1. 查看成绩");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim().parse::<u32>() {
                Ok(1) => {
                    view_grades();
                },
                _ => {
                    println!("无效的选项。");
                }
            }
        },
        Role::Teacher => {
            // 显示老师的功能界面
            println!("欢迎老师 {} 登录系统！", user.name);
            println!("请选择操作：");
            println!("1. 账户管理");
            println!("2. 教室管理");
            println!("3. 课程管理");
            println!("4. 成绩管理");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim().parse::<u32>() {
                Ok(1) => {
                    manage_accounts();
                },
                Ok(2) => {
                    manage_classrooms();
                },
                Ok(3) => {
                    manage_courses();
                },
                Ok(4) => {
                    manage_grades(&user.name);
                },
                _ => {
                    println!("无效的选项。");
                }
            }

        },
        Role::Principal => {
            // 显示校长的功能界面
            println!("欢迎校长 {} 登录系统！", user.name);
            println!("请选择操作：");
            println!("1. 管理账户");
            println!("2. 管理教室");

            let mut choice = String::new();
            io::stdin().read_line(&mut choice).unwrap();
            match choice.trim().parse::<u32>() {
                Ok(1) => {
                    manage_accounts();
                },
                Ok(2) => {
                    manage_classrooms();
                },
                _ => {
                    println!("无效的选项。");
                }
            }
        },
    }
}


fn manage_accounts() {
    loop {
        println!("请选择操作：");
        println!("1. 添加账户");
        println!("2. 修改密码");
        println!("3. 删除账户");
        println!("4. 返回上一级");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                add_account();
            },
            Ok(2) => {
                change_password();
            },
            Ok(3) => {
                delete_account();
            },
            Ok(4) => {
                break;
            },
            _ => {
                println!("无效的选项。");
            }
        }
    }
}

fn manage_classrooms() {
    loop {
        println!("请选择操作：");
        println!("1. 添加教室");
        println!("2. 删除教室");
        println!("3. 返回上一级");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                add_classroom();
            },
            Ok(2) => {
                delete_classroom();
            },
            Ok(3) => {
                break;
            },
            _ => {
                println!("无效的选项。");
            }
        }
    }
}

fn manage_grades(teacher_username: &str) {
    println!("请输入学生的用户名：");
    let mut student_username = String::new();
    io::stdin().read_line(&mut student_username).unwrap();
    let student_username = student_username.trim();
    let student_id = sha256(&student_username);

    println!("请输入课程ID：");
    let mut course_id = String::new();
    io::stdin().read_line(&mut course_id).unwrap();
    let course_id = course_id.trim();

    println!("请输入学生的成绩：");
    let mut grade_str = String::new();
    io::stdin().read_line(&mut grade_str).unwrap();
    let grade = grade_str.trim().parse::<f32>().expect("请输入有效的成绩！");

    db::grade_student(teacher_username, &student_id, &course_id, grade).expect("打分失败！");
}

// src/main.rs

// ...

fn manage_courses() {
    loop {
        println!("请选择操作：");
        println!("1. 添加课程");
        println!("2. 删除课程");
        println!("3. 返回上一级");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim().parse::<u32>() {
            Ok(1) => {
                add_course();
            },
            Ok(2) => {
                delete_course();
            },
            Ok(3) => {
                break;
            },
            _ => {
                println!("无效的选项。");
            }
        }
    }
}

fn add_account() {
    println!("请输入新账户的用户名：");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("请输入新账户的密码：");
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();
    let password = password.trim();

    println!("请选择新账户的角色：1. 学生，2. 教师，3. 校长");
    let mut role = String::new();
    io::stdin().read_line(&mut role).unwrap();
    let role = match role.trim().parse::<u32>() {
        Ok(1) => Role::Student,
        Ok(2) => Role::Teacher,
        Ok(3) => Role::Principal,
        _ => panic!("无效的角色选项。"),
    };

    db::add_user(&username, &password, role).expect("添加账户失败！");
    println!("添加账户成功！");
}

fn change_password() {
    println!("请输入需要修改密码的账户用户名：");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    println!("请输入新密码：");
    let mut new_password = String::new();
    io::stdin().read_line(&mut new_password).unwrap();
    let new_password = new_password.trim();

    db::change_user_password(&username, &new_password).expect("修改密码失败！");
    println!("修改密码成功！");
}

fn delete_account() {
    println!("请输入需要删除的账户用户名：");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    db::delete_user(&username).expect("删除账户失败！");
    println!("删除账户成功！");
}

fn add_classroom() {
    println!("请输入新教室的ID：");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    let classroom = Classroom {
        id: id.to_string(),
        // 根据实际需求，可能需要输入更多关于教室的信息
        course_ids: vec![],
    };

    db::add_classroom(classroom).expect("添加教室失败！");
    println!("添加教室成功！");
}

fn delete_classroom() {
    println!("请输入需要删除的教室ID：");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    db::delete_classroom(&id).expect("删除教室失败！");
    println!("删除教室成功！");
}


fn add_course() {
    println!("请输入新课程的ID：");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    println!("请输入新课程的名称：");
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    println!("请输入新课程的描述：");
    let mut description = String::new();
    io::stdin().read_line(&mut description).unwrap();
    let description = description.trim();

    println!("请输入教师的用户名：");
    let mut teacher_username = String::new();
    io::stdin().read_line(&mut teacher_username).unwrap();
    let teacher_id = sha256(&teacher_username.trim());

    println!("请输入教室的ID：");
    let mut classroom_id = String::new();
    io::stdin().read_line(&mut classroom_id).unwrap();
    let classroom_id = classroom_id.trim();

    let course = Course {
        id: id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        teacher_id: teacher_id,
        classroom_id: classroom_id.to_string(),
        grades: HashMap::new(),
    };

    db::add_course(course).expect("添加课程失败！");
    println!("添加课程成功！");
}

fn delete_course() {
    println!("请输入需要删除的课程ID：");
    let mut id = String::new();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    db::modify_course(&id, Course {
        id: "".to_string(),
        name: "".to_string(),
        description: "".to_string(),
        teacher_id: "".to_string(),
        classroom_id: "".to_string(),
        grades: HashMap::new(),
    }).expect("删除课程失败！");
    println!("删除课程成功！");
}


fn view_grades() {
    println!("请输入你的姓名：");
    let mut student_name = String::new();
    io::stdin().read_line(&mut student_name).unwrap();
    let student_id = sha256(&student_name.trim());

    let grades = db::get_student_grades(&student_id).expect("获取成绩失败！");
    for (course_name, grade) in &grades {
        println!("课程 {} 的成绩是：{}", course_name, grade);
    }
}