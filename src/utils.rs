// Ejercicio 1
pub fn es_par(numero: i32) -> bool {
  (numero % 2) == 0
}

#[test]
fn es_par_con_numero_par() {
  assert_eq!(es_par(22), true);
}
#[test]
fn es_par_con_numero_impar() {
  assert_eq!(es_par(23), false);
}

