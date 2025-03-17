mod read;
pub use read::input;

fn main() {
    let num: i32 = input("Digite um número: ");
    println!("Número digitado: {}", num);

    let nome: String = input("Digite seu nome: ");
    println!("Nome digitado: {}", nome);
}
