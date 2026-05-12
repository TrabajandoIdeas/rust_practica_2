mod tp02;

const SIZE: usize = 3;

fn main() {
  println!("12 es par?: {}",tp02::ej01::es_par(12));
  println!("13 es par?: {}",tp02::ej01::es_par(13));

  println!("7 es primo?: {}", tp02::ej02::es_primo(7));
  println!("8 es primo?: {}", tp02::ej02::es_primo(8));

  println!("Suma de pares en [7, 32, 3, 5, 42]: {}", tp02::ej03::suma_pares(&[7, 32, 3, 5, 42]));
  println!("Cantidad de impares en [7, 32, 3, 42]: {}", tp02::ej04::cantidad_impares(&[7, 32, 3, 42]));

  println!("Duplicar valores [3.5, 2.9, 5.0]: {:?}", tp02::ej05::duplicar_valores::<SIZE>(&[3.5, 2.9, 5.0]));
  println!("Longitud de cadenas {:?}", tp02::ej06::longitud_de_cadenas::<3>(&[String::from("Hola"), String::from(" "), String::from("Mundo")]));
  println!("Cantidad mayores a 30 en [32, 23, 65, 2, 4, 40]: {}", tp02::ej07::cantidad_de_mayores(&[32, 23, 65, 2, 4, 40], 30));
  println!("Sumar arreglos [23.5, 10.0] + [82.5, 87.9]: {:?}", tp02::ej08::sumar_arreglos::<SIZE>(&[23.5, 10.0], &[82.5, 87.9]));
  println!("Cantidad en rango (15, 40) en [33, 43, 3]: {}", tp02::ej09::cantidad_en_rango(&[33, 43, 3], 15, 40));
}
