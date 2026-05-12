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


// Ejercicio 6
pub fn longitud_de_cadenas<const SIZE: usize>(lista: &[String]) -> [usize;SIZE] {
  let mut nueva_lista: [usize; SIZE] = [0;SIZE];

  for i in 0..lista.len() {
      nueva_lista[i] = lista[i].len();
  }
  
  return nueva_lista;
}

#[test]
fn longitud_de_cadenas_con_3_cadenas() {
  assert_eq!(longitud_de_cadenas(&[String::from("Hola"), String::from(" "), String::from("Mundo")]), [4, 1, 5])
}


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


