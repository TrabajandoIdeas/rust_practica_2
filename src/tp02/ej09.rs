// Ejercicio 9
pub fn cantidad_en_rango(lista: &[i32], inferior: i32, superior: i32) -> i32 {
  if inferior > superior {
    panic!("Inferior tiene que ser mayor a superior")
  }
  
  let mut count = 0;
  for item in lista {
    if inferior < *item && *item < superior {
      count += 1;
    }
  }
  return count;
}

#[test]
fn cantidad_en_rango_con_algunos_valores() {
  assert_eq!(cantidad_en_rango(&[33, 43, 3],15, 40), 1);
}
#[test]
fn cantidad_en_rango_sin_valores_en_rango() {
  assert_eq!(cantidad_en_rango(&[7, 32, 3], 45, 50), 0);
}
#[test]
#[should_panic(expected="Inferior tiene que ser mayor a superior")]
fn cantidad_en_rango_con_inferior_mayor_a_superior() {
  cantidad_en_rango(&[2, 32, 24], 30, 20);
}
