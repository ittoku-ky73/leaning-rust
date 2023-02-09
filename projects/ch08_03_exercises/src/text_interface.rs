// ハッシュマップとベクターを使って、ユーザーが従業員名を会社の部署に追加できるようなテキストインターフェイスを作成。
// 例えば、"サリーを技術部に"や"アミールを営業部に"といった感じです。そしてユーザーに部門に属するすべての人、または部門別に会社に属するすべての人をアルファベット順に並べたリストを取得する

use std::collections::HashMap;
use std::io::{self, stdout, Write};

// 社員と部門の登録、そのリストの表示ができる
pub fn text_interface() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        read_line(&mut input, "Select action [a,l,q,?] > ");

        match input.trim() {
            "a" => register(&mut map),
            "l" => list(&mut map),
            "q" => {
                println!("Bye 👋");
                break;
            }
            "?" => {
                println!(
                    "{}\n{}\n{}\n{}",
                    "a - Add Employee and Department",
                    "l - retrieve a list of all people in a department, sorted alphabetically",
                    "q - quit",
                    "? - help"
                );
            }
            _ => (),
        }
    }
}

fn read_line(input: &mut String, text: &str) {
    print!("{text}");
    stdout().flush().unwrap();
    io::stdin().read_line(input).expect("Failed to read line");
}

fn register(map: &mut HashMap<String, Vec<String>>) {
    let mut dept = String::new();
    let mut employee = String::new();

    read_line(&mut dept, "Depertment > ");
    read_line(&mut employee, "Employee > ");

    // 改行を削除
    let dept = dept.lines().collect::<String>();
    let employee = employee.lines().collect::<String>();

    if map.contains_key(&dept) {
        map.get_mut(&dept).unwrap().push(employee);
    } else {
        map.entry(dept).or_insert(vec![employee]);
    }
}

fn list(map: &mut HashMap<String, Vec<String>>) {
    println!("Department Employee List");

    for (dept, employees) in map.iter_mut() {
        println!("{dept}");
        employees.sort();
        for employee in employees {
            println!("\t{employee}");
        }
    }
}
