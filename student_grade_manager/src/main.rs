use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::collections::HashMap;

const FILENAME: &str = "grades.txt";
const MAX_STUDENTS: usize = 1000;

#[derive(Clone, Debug)]
struct Student {
    id: String,
    name: String,
    math: f32,
    english: f32,
    physics: f32,
    average: f32,
    rank: usize,
}

fn save_students_to_file(students: &[Student]) {
    let mut file = match File::create(FILENAME) {
        Ok(f) => f,
        Err(_) => {
            println!("无法打开文件。");
            return;
        }
    };
    writeln!(file, "学号\t\t\t姓名\t\t高等数学\t\t英语\t\t物理\t\t平均成绩\t\t名次").unwrap();
    for s in students {
        writeln!(file, "{}\t\t{}\t\t{:.2}\t\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{}",
            s.id, s.name, s.math, s.english, s.physics, s.average, s.rank).unwrap();
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
            println!("无法打开文件。");
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

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn insert_student(students: &mut Vec<Student>) {
    if students.len() >= MAX_STUDENTS {
        println!("成绩表已满，无法插入新记录。");
        return;
    }
    let id = input("请输入学生的学号：");
    if id.contains(' ') {
        println!("根据中国传媒大学本科生学籍管理规定，学号中不能包含空格，请重新输入。");
        return;
    }
    if students.iter().any(|s| s.id == id) {
        println!("根据中国传媒大学本科生学籍管理规定，学号已存在，请重新输入。");
        return;
    }
    let name = input("请输入学生的姓名：");
    if name.contains(' ') {
        println!("根据中国传媒大学本科生学籍管理规定，姓名中不能包含空格，请重新输入。");
        return;
    }
    let math = loop {
        let s = input("请输入学生的高等数学成绩：");
        match s.parse::<f32>() {
            Ok(v) if v >= 0.0 && v <= 100.0 => break v,
            _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
        }
    };
    let english = loop {
        let s = input("请输入学生的英语成绩：");
        match s.parse::<f32>() {
            Ok(v) if v >= 0.0 && v <= 100.0 => break v,
            _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
        }
    };
    let physics = loop {
        let s = input("请输入学生的物理成绩：");
        match s.parse::<f32>() {
            Ok(v) if v >= 0.0 && v <= 100.0 => break v,
            _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
        }
    };
    let average = (math + english + physics) / 3.0;
    let student = Student { id, name, math, english, physics, average, rank: 0 };
    students.push(student);
    println!("学生信息插入成功。");
    save_students_to_file(students);
}

fn delete_student(students: &mut Vec<Student>) {
    let id = input("请输入要删除的学生学号：");
    let pos = students.iter().position(|s| s.id == id);
    if let Some(idx) = pos {
        students.remove(idx);
        println!("学生信息删除成功。");
        save_students_to_file(students);
    } else {
        println!("未找到对应学号的学生。");
    }
}

fn update_student(students: &mut Vec<Student>) {
    let id = input("请输入要修改的学生学号：");
    if let Some(student) = students.iter_mut().find(|s| s.id == id) {
        student.math = loop {
            let s = input("请输入学生的高等数学成绩：");
            match s.parse::<f32>() {
                Ok(v) if v >= 0.0 && v <= 100.0 => break v,
                _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
            }
        };
        student.english = loop {
            let s = input("请输入学生的英语成绩：");
            match s.parse::<f32>() {
                Ok(v) if v >= 0.0 && v <= 100.0 => break v,
                _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
            }
        };
        student.physics = loop {
            let s = input("请输入学生的物理成绩：");
            match s.parse::<f32>() {
                Ok(v) if v >= 0.0 && v <= 100.0 => break v,
                _ => println!("成绩必须为0到100之间的数字，请重新输入。"),
            }
        };
        student.average = (student.math + student.english + student.physics) / 3.0;
        println!("学生信息修改成功。");
        save_students_to_file(students);
    } else {
        println!("未找到对应学号的学生。");
    }
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

fn print_students(students: &[Student]) {
    println!("学号\t\t\t姓名\t\t高等数学\t\t英语\t\t物理\t\t平均成绩\t\t名次");
    for s in students {
        println!("{:13}\t\t{}\t\t{:.2}\t\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{}",
            s.id, s.name, s.math, s.english, s.physics, s.average, s.rank);
    }
}

fn main() {
    let mut students = read_students_from_file();
    loop {
        println!("\n成绩管理程序");
        println!("1. 插入学生记录");
        println!("2. 删除学生记录");
        println!("3. 修改学生记录");
        println!("4. 开发中,无法使用");
        println!("5. 计算每名学生的平均成绩并且输出成绩表");
        println!("0. 退出程序");
        let choice = input("请输入选项（只允许输入数字，否则后果自负）：");
        match choice.as_str() {
            "1" => insert_student(&mut students),
            "2" => delete_student(&mut students),
            "3" => update_student(&mut students),
            // "4" => sort_by_math(&mut students), // 未实现
            "5" => {
                calculate_average(&mut students);
                sort_by_average(&mut students);
                assign_ranks(&mut students);
                print_students(&students);
                save_students_to_file(&students);
            },
            "0" => {
                println!("程序已退出。");
                break;
            },
            _ => println!("无效的选项。"),
        }
    }
}
