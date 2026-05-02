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


// Ejercicio 2
pub fn es_primo(numero: i32) -> bool {
  if numero <=0 || es_par(numero)  {
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


// Ejercicio 3
fn suma_pares(lista: &[i32]) -> i32 {
  lista
    .iter()
    .fold(0, |acc, item| if es_par(*item) { acc + item } else { acc } )
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


// Ejercicio 4
fn cantidad_impares(lista: &[i32]) -> u32 {
  lista
    .iter()
    .fold(0, |acc, item| if !es_par(*item) { acc + 1 } else { acc })
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