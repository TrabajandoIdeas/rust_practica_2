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
