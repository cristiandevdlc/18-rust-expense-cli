use std::{env, fs::{self, OpenOptions}, io::Write};
fn main() {
    let args: Vec<String> = env::args().collect(); let file = "expenses.csv";
    match args.get(1).map(String::as_str) {
        Some("add") => { if args.len() < 5 { eprintln!("Uso: add descripción importe categoría"); return; } let mut output = OpenOptions::new().create(true).append(true).open(file).unwrap(); writeln!(output, "{},{},{}", args[2].replace(',', " "), args[3], args[4]).unwrap(); println!("Gasto guardado"); }
        Some("list") => match fs::read_to_string(file) { Ok(data) => print!("{data}"), Err(_) => println!("Sin gastos") },
        _ => println!("Uso: add DESCRIPCIÓN IMPORTE CATEGORÍA | list"),
    }
}
