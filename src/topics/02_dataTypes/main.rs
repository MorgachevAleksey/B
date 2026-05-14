fn main() {
    let x : u32 = 100; //аннотация типа
    let x = 100u32; //суффикс типа

    let y = 2.0; //f64 по-умолчанию
    let y :f32 = 2.0; //f32

    let a1 = 43;
    let a2 = 5;
    let a3 = a1 % a2;
    let a4 = a1 / a2;
    println!("43 % 5 = {a3} \n 43 / 5 = {a4}");
}