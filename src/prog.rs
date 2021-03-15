fn main(){
    println!("Hola, probando el programa");
    let argv: Vec<_> = std::env::args().collect();
    println!("{:?}", argv);
}