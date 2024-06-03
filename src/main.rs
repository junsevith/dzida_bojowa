use std::fs::File;
use std::io;
use std::io::{BufRead, Write};

fn main() {
    let mut file = File::create("dzida.txt").unwrap();

    println!("Podaj przedmiot: (z dużej)");
    let binding = io::stdin().lock().lines().next().unwrap().unwrap();
    let item = binding.trim();

    println!("Podaj przedmiot w dopełniaczu: (kogo/czego) (z małej)");
    let binding = io::stdin().lock().lines().next().unwrap().unwrap();
    let dop = binding.trim();


    println!("Podaj liczbę podzieleń:");
    let count = io::stdin().lock().lines().next().unwrap().unwrap()
        .trim().parse::<usize>().expect("Niepoprawna wartość");

    dzida(item, dop, count, &mut file);
}

fn dzida(mianownik: &str, dopelniacz: &str, count: usize, file: &mut File) {
    if count <= 0 {
        return;
    }
    file.write_all(format!("{} składa się z przedniej części {}, środkowej części {} i tylnej części {}.\n", mianownik, dopelniacz, dopelniacz, dopelniacz).as_bytes()).unwrap();
    dzida(&("Przednia część ".to_owned() + dopelniacz), &("przedniej części ".to_owned() + dopelniacz), count - 1, file);
    dzida(&("Środkowa część ".to_owned() + dopelniacz), &("środkowej części ".to_owned() + dopelniacz), count - 1, file);
    dzida(&("Tylna część ".to_owned() + dopelniacz), &("tylnej części ".to_owned() + dopelniacz), count - 1, file);
}