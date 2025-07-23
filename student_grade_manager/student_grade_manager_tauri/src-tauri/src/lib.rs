use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

const FILENAME: &str = "grades.txt";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub math: f32,
    pub english: f32,
    pub physics: f32,
    pub average: f32,
    pub rank: usize,
}

fn save_students_to_file(students: &[Student]) {
    let mut file = match File::create(FILENAME) {
        Ok(f) => f,
        Err(_) => {
            return;
        }
    };
    writeln!(file, "学号\t\t\t姓名\t\t高等数学\t\t英语\t\t物理\t\t平均成绩\t\t名次").ok();
    for s in students {
        writeln!(file, "{}\t\t{}\t\t{:.2}\t\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{}",
            s.id, s.name, s.math, s.english, s.physics, s.average, s.rank).ok();
    }
}

fn read_students_from_file() -> Vec<Student> {
    let mut students = Vec::new();
    if !Path::new(FILENAME).exists() {
        return students;
    }
    let file = match File::open(FILENAME) {
        Ok(f) => f,
        Err(_) => {
            return students;
        }
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    lines.next(); // 跳过表头
    for line in lines {
        if let Ok(l) = line {
            let parts: Vec<&str> = l.split_whitespace().collect();
            if parts.len() < 7 { continue; }
            let id = parts[0].to_string();
            let name = parts[1].to_string();
            let math = parts[2].parse().unwrap_or(0.0);
            let english = parts[3].parse().unwrap_or(0.0);
            let physics = parts[4].parse().unwrap_or(0.0);
            let average = parts[5].parse().unwrap_or(0.0);
            let rank = parts[6].parse().unwrap_or(0);
            students.push(Student { id, name, math, english, physics, average, rank });
        }
    }
    students
}

fn calculate_average(students: &mut Vec<Student>) {
    for s in students.iter_mut() {
        s.average = (s.math + s.english + s.physics) / 3.0;
    }
}

fn sort_by_average(students: &mut Vec<Student>) {
    students.sort_by(|a, b| b.average.partial_cmp(&a.average).unwrap());
}

fn assign_ranks(students: &mut Vec<Student>) {
    if students.is_empty() { return; }
    students[0].rank = 1;
    for i in 1..students.len() {
        if (students[i].average - students[i-1].average).abs() < 1e-6 {
            students[i].rank = students[i-1].rank;
        } else {
            students[i].rank = i + 1;
        }
    }
}

#[tauri::command]
pub fn get_students() -> Vec<Student> {
    let mut students = read_students_from_file();
    calculate_average(&mut students);
    sort_by_average(&mut students);
    assign_ranks(&mut students);
    students
}

#[tauri::command]
pub fn add_student(student: Student) -> Result<(), String> {
    let mut students = read_students_from_file();
    if students.iter().any(|s| s.id == student.id) {
        return Err("学号已存在".to_string());
    }
    students.push(student);
    calculate_average(&mut students);
    sort_by_average(&mut students);
    assign_ranks(&mut students);
    save_students_to_file(&students);
    Ok(())
}

#[tauri::command]
pub fn delete_student(id: String) -> Result<(), String> {
    let mut students = read_students_from_file();
    let len_before = students.len();
    students.retain(|s| s.id != id);
    if students.len() == len_before {
        return Err("未找到对应学号的学生".to_string());
    }
    calculate_average(&mut students);
    sort_by_average(&mut students);
    assign_ranks(&mut students);
    save_students_to_file(&students);
    Ok(())
}

#[tauri::command]
pub fn update_student(student: Student) -> Result<(), String> {
    let mut students = read_students_from_file();
    let mut found = false;
    for s in students.iter_mut() {
        if s.id == student.id {
            s.name = student.name.clone();
            s.math = student.math;
            s.english = student.english;
            s.physics = student.physics;
            found = true;
        }
    }
    if !found {
        return Err("未找到对应学号的学生".to_string());
    }
    calculate_average(&mut students);
    sort_by_average(&mut students);
    assign_ranks(&mut students);
    save_students_to_file(&students);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_students,
            add_student,
            delete_student,
            update_student
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
