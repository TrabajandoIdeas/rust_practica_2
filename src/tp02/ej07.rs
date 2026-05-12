// Ejercicio 7
pub fn cantidad_de_mayores(lista: &[i32], limite: i32) -> u32 {
  let mut count = 0;

  for item in lista {
    if *item > limite {
      count += 1;
    }
  }

  return count;
}

#[test]
fn cantidad_de_mayores_con_variedad() {
  assert_eq!(cantidad_de_mayores(&[32, 23,65,2,4,40], 30), 3);
}
#[test]
fn cantidad_de_mayores_cont_todos_mayores() {
  assert_eq!(cantidad_de_mayores(&[32, 23,65,2,4,40], 1), 6);
}
#[test]
fn cantidad_de_mayores_cont_todos_menores() {
  assert_eq!(cantidad_de_mayores(&[32, 23,65,2,4,40], 70), 0);
}
