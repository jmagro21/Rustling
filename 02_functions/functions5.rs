// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// Ces exercices avaient des erreurs sur les fonctions.
// On devait résoudre le fait qu'il y ai de mauvais appel, des return, et des utilisations spécifiques des variables
// On à appris à utiliser le return / le -> pour donner une valeur / 

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num
}
