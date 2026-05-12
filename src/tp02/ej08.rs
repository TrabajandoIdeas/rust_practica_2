// Ejercicio 8
pub fn sumar_arreglos<const SIZE: usize>(lista_a: &[f64], lista_b: &[f64]) -> [f64;SIZE] {
  if lista_a.len() != lista_b.len() && lista_a.len() != SIZE {
    panic!("Both arrays should have the same size");
  }

  let mut suma: [f64; SIZE] = [0.0; SIZE];

  for index in 0..lista_a.len() {
    suma[index] = lista_a[index] + lista_b[index];
  }

  return suma;
}

#[test]
fn sumar_arreglos_variados() {
  assert_eq!(sumar_arreglos(&[23.5, 10.0], &[82.5, 87.9]), [106.0, 97.9])
}
#[test]
fn sumar_arreglos_vacios() {
  assert_eq!(sumar_arreglos(&[], &[]), []);
}
#[test]
#[should_panic(expected = "Both arrays should have the same size")]
fn sumar_arreglos_panic() {
  sumar_arreglos::<2>(&[2.0], &[3.0,4.0]);
}
