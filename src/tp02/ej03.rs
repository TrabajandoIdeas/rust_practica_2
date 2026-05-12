// Ejercicio 3
pub fn suma_pares(lista: &[i32]) -> i32 {
  // lista
  //   .iter()
  //   .fold(0, |acc, item| if es_par(*item) { acc + item } else { acc } )
  let mut suma: i32 = 0;
  for item in lista {
    if super::ej01::es_par(*item) {
      suma += *item;
    }
  }
  return suma;
}

#[test]
fn suma_pares_con_solo_pares() {
  assert_eq!(suma_pares(&[32, 42, 2]), 76);
}
#[test]
fn suma_pares_con_unos_impares() {
  assert_eq!(suma_pares(&[7, 32, 3, 5, 42]), 74);
}
#[test]
fn suma_pares_sin_pares() {
  assert_eq!(suma_pares(&[5, 47, 25]), 0);
}
