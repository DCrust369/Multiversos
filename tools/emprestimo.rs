fn main() {
    let valor = String::from("DCrust");
    let emprestimo = &valor; // borrow nativo
    println!("{}", emprestimo);
}
