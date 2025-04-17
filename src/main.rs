fn main() {
    let name = "Диас"; // нельзя изменить
    println!("Салам, {}!", name);

    let mut age = 25; // можно изменить, потому что mut
    age = 26;
    println!("Жасың: {}", age);
}