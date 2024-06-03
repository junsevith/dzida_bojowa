use std::{env, io};
use std::io::BufRead;

fn main() {
    println!("Podaj przedmiot: (z dużej)");
    let binding = io::stdin().lock().lines().next().unwrap().unwrap();
    let item = binding.trim();

    println!("Podaj przedmiot w dopełniaczu: (kogo/czego) (z małej)");
    let binding = io::stdin().lock().lines().next().unwrap().unwrap();
    let dop = binding.trim();


    println!("Podaj liczbę podzieleń:");
    let count = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().parse::<usize>().expect("Niepoprawna wartość");

    dzida(item, dop, count);
}

fn dzida(mianownik: &str, dopelniacz: &str, count: usize) {
    if count <= 0 {
        return;
    }
    println!("{} składa się z przedniej części {}, środkowej części {} i tylnej części {}.", mianownik, dopelniacz, dopelniacz, dopelniacz);
    dzida(&("Przednia część ".to_owned() + dopelniacz), &("przedniej części ".to_owned() + dopelniacz), count - 1);
    dzida(&("Środkowa część ".to_owned() + dopelniacz), &("środkowej części ".to_owned() + dopelniacz), count - 1);
    dzida(&("Tylna część ".to_owned() + dopelniacz), &("tylnej części ".to_owned() + dopelniacz), count - 1);
}