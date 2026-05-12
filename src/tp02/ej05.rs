// Ejercicio 5
pub fn duplicar_valores<const SIZE: usize>(lista: &[f64]) -> [f64;SIZE] {
  let mut nueva_lista = [0.0; SIZE];
  let mut index = 0;

  while index < lista.len() {
      nueva_lista[index] = lista[index] * 2.0;
      index += 1;
  }

  return nueva_lista;
}

#[test]
fn duplicar_valores_con_valores() {
  assert_eq!(duplicar_valores(&[3.5, 2.9, 5.0]), [7.0, 5.8, 10.0]);
}
#[test]
fn duplicar_valores_sin_valores() {
  assert_eq!(duplicar_valores(&[]), []);
}
