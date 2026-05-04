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
pub fn suma_pares(lista: &[i32]) -> i32 {
  // lista
  //   .iter()
  //   .fold(0, |acc, item| if es_par(*item) { acc + item } else { acc } )
  let mut suma: i32 = 0;
  for item in lista {
    if es_par(*item) {
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


// Ejercicio 4
fn cantidad_impares(lista: &[i32]) -> u32 {
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


// Ejercicio 5
fn duplicar_valores(lista: &[f64]) -> Vec<f64> {
  let mut nueva_lista: Vec<f64> = Vec::new();

  for item in lista {
    nueva_lista.push(*item * 2.0);
  }

  return nueva_lista;
}

#[test]
fn duplicar_valores_con_valores() {
  assert_eq!(duplicar_valores(&[3.5, 2.9, 5.0]), vec![7.0, 5.8, 10.0]);
}
#[test]
fn duplicar_valores_sin_valores() {
  assert_eq!(duplicar_valores(&[]), vec![]);
}


// Ejercicio 6
fn longitud_de_cadenas(lista: &[String]) -> Vec<usize> {
  let mut nueva_lista: Vec<usize> = Vec::new();

  for item in lista {
    nueva_lista.push(item.len());
  }

  return nueva_lista;
}

#[test]
fn longitud_de_cadenas_con_3_cadenas() {
  assert_eq!(longitud_de_cadenas(&[String::from("Hola"), String::from(" "), String::from("Mundo")]), vec![4, 1, 5])
}


// Ejercicio 7
fn cantidad_de_mayores(lista: &[i32], limite: i32) -> u32 {
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

