use crate::tp02::ej01::es_par;

// Ejercicio 4
pub fn cantidad_impares(lista: &[i32]) -> u32 {
  // lista
  //   .iter()
  //   .fold(0, |acc, item| if !es_par(*item) { acc + 1 } else { acc })
  
  let mut count:u32 = 0;
  for item in lista {
    if !es_par(*item) {
      count += 1;
    }
  }

  return count;
} 

#[test]
fn cantidad_impares_con_solo_impares() {
  assert_eq!(cantidad_impares(&[33, 43, 3]), 3);
}
#[test]
fn cantidad_impares_con_unos_pares() {
  assert_eq!(cantidad_impares(&[7, 32, 3, 42]), 2);
}
#[test]
fn cantidad_impares_sin_impares() {
  assert_eq!(cantidad_impares(&[2, 32, 24]), 0);
}
