
use sled::{Db, Tree};
use lazy_static::lazy_static;
use crate::structs::{User, Role, Classroom, Course};
use crate::crypto::sha256;
use std::collections::HashMap;

lazy_static! {
    pub static ref DB: Db = sled::open("my_database").expect("open database");
    pub static ref USERS: Tree = DB.open_tree("users").expect("open users tree");
    pub static ref COURSES: Tree = DB.open_tree("courses").expect("open courses tree");
    pub static ref CLASSROOMS: Tree = DB.open_tree("classrooms").expect("open classrooms tree");
}


pub fn add_user(username: &str, password: &str, role: Role) -> sled::Result<()> {
    let user = User {
        id: sha256(username),
        name: username.to_string(),
        password: sha256(password),
        role: role,
    };
    let user_bytes = bincode::serialize(&user).unwrap();
    USERS.insert(&user.id, user_bytes)?;
    Ok(())
}

pub fn change_user_password(username: &str, new_password: &str) -> sled::Result<()> {
    let user_id = sha256(username);
    if let Some(user_bytes) = USERS.get(&user_id)? {
        let mut user: User = bincode::deserialize(&user_bytes).unwrap();
        user.password = sha256(new_password);
        let user_bytes = bincode::serialize(&user).unwrap();
        USERS.insert(&user_id, user_bytes)?;
    }
    Ok(())
}

pub fn delete_user(username: &str) -> sled::Result<()> {
    let user_id = sha256(username);
    USERS.remove(&user_id)?;
    Ok(())
}

pub fn add_course(course: Course) -> sled::Result<()> {
    let course_bytes = bincode::serialize(&course).unwrap();
    COURSES.insert(&course.id, course_bytes)?;
    Ok(())
}

pub fn modify_course(course_id: &str, new_course: Course) -> sled::Result<()> {
    let course_bytes = bincode::serialize(&new_course).unwrap();
    COURSES.insert(course_id, course_bytes)?;
    Ok(())
}

pub fn grade_student(teacher_username: &str, student_username: &str, course_id: &str, grade: f32) -> sled::Result<()> {
    let teacher_id = sha256(teacher_username);
    if let Some(teacher_bytes) = USERS.get(&teacher_id)? {
        let teacher: User = bincode::deserialize(&teacher_bytes).unwrap();
        if teacher.role != Role::Teacher {
            return Err(sled::Error::Unsupported("你没有权限进行打分!".into()));
        }
    }

    if let Some(course_bytes) = COURSES.get(course_id)? {
        let mut course: Course = bincode::deserialize(&course_bytes).unwrap();
        course.grades.insert(student_username.to_string(), grade);
        let course_bytes = bincode::serialize(&course).unwrap();
        COURSES.insert(course_id, course_bytes)?;
    }
    Ok(())
}

pub fn get_student_grades(student_id: &str) -> sled::Result<HashMap<String, f32>> {
    let mut grades = HashMap::new();

    for result in COURSES.iter() {
        let (_key, value) = result?;
        let course: Course = bincode::deserialize(&value).unwrap();
        if let Some(grade) = course.grades.get(student_id) {
            grades.insert(course.name.clone(), *grade);
        }
    }

    Ok(grades)
}


pub fn add_classroom(classroom: Classroom) -> sled::Result<()> {
    let classroom_bytes = bincode::serialize(&classroom).unwrap();
    CLASSROOMS.insert(&classroom.id, classroom_bytes)?;
    Ok(())
}

pub fn delete_classroom(classroom_id: &str) -> sled::Result<()> {
    CLASSROOMS.remove(classroom_id)?;
    Ok(())
}