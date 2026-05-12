// Ejercicio 2
pub fn es_primo(numero: i32) -> bool {
  if numero <=0 || super::ej01::es_par(numero)  {
    return false;
  }
  
  let mut count: i32 = 3;
  let limit: i32 = numero / 2;
  loop {
    if count == limit {
      break true;
    }
    if (numero % count) == 0 {
      break false;
    }

    count += 2;
  }
}

#[test]
fn es_primo_con_un_primo() {
  assert!(es_primo(7))
}
#[test]
fn es_primo_con_un_no_primo() {
  assert!(!es_primo(8))
}