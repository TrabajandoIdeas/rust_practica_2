mod utils;

fn main() {
  println!("12 es par?: {}",utils::es_par(12));
  println!("13 es par?: {}",utils::es_par(13));

  println!("7 es primo?: {}", utils::es_primo(7));
  println!("8 es primo?: {}", utils::es_primo(8));

  println!("Suma de pares en [7, 32, 3, 5, 42]: {}", utils::suma_pares(&[7, 32, 3, 5, 42]));
  println!("Cantidad de impares en [7, 32, 3, 42]: {}", utils::cantidad_impares(&[7, 32, 3, 42]));

  println!("Duplicar valores [3.5, 2.9, 5.0]: {:?}", utils::duplicar_valores(&[3.5, 2.9, 5.0]));
  println!("Cantidad mayores a 30 en [32, 23, 65, 2, 4, 40]: {}", utils::cantidad_de_mayores(&[32, 23, 65, 2, 4, 40], 30));
  println!("Sumar arreglos [23.5, 10.0] + [82.5, 87.9]: {:?}", utils::sumar_arreglos(&[23.5, 10.0], &[82.5, 87.9]));
  println!("Cantidad en rango (15, 40) en [33, 43, 3]: {}", utils::cantidad_en_rango(&[33, 43, 3], 15, 40));
}