use core::slice;
use std::{io::{self, SeekFrom, Write}, ops::Not, string};

const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N],
}

impl Cadena {
    fn new() -> Cadena {
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    fn obt_longitud(&self) -> usize {
        self.longitud
    }

    fn add_char(&mut self, c: char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    fn obt_char(&self, posicion: usize) -> char {
      if posicion >0 && posicion<self.longitud {
        self.caracteres[posicion-1]
      } else {
          '\0'
      } 
    }
fn invcad(&self) -> String {
    let mut aux: String = String::new();

    let mut i: usize = self.longitud;

    while i > 0 {
        i -= 1;
        aux.push(self.caracteres[i]);
    }

     aux
}


fn masrchar(&self) {
    let mut conm = 0;
    let mut cara = '\0';

    for i in 0..self.longitud {
        let mut con = 1;

        for k in i+1..self.longitud {
            if self.caracteres[i] == self.caracteres[k] {
                con += 1;
            }
        }

        if con > conm {
            conm = con;
            cara = self.caracteres[i];
        }
    }

    println!("El caracter '{}' se repite {} veces", cara, conm);
}

    // limpia la cadena para poder ingresar una nuev
    fn vervocal(&self, c: char)->bool {
        if (c=='a'|| c=='e'||c=='i'||c=='o'||c=='u'||c=='A'||c=='E'||c=='I'||c=='O'||c=='U') {
            true
        }
        else {
            false
        }
    }

    fn vercons(&self, c:char)->bool{
        if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
        if !(c == 'a'|| c == 'e'|| c == 'i'|| c == 'o'|| c == 'u'||
             c == 'A'|| c == 'E'|| c == 'I'|| c == 'O'|| c == 'U') {
            return true
        }
    }
    false
        
    }

    fn novocal(&self)->String {
        let mut aux: String = String::new();
        for j in 0..self.longitud {
            let mut c:char = self.caracteres[j];
            if self.vervocal(c) == false {
                aux.push( self.caracteres[j]);
            }
        }
        aux
    }
   //cuenta caracter y vocales p

    fn cont_voc_cons(&self) {
        let mut co =0; 
        let mut vo =0;
        for i in 0..self.longitud {
            let mut c: char = self.caracteres[i];
            if self.vervocal(c){
                vo+=1;
            }
            else if self.vercons(c){
                co+=1;
            }
        }
        println!("Existen {} vocales y {} consonantes",vo,co);
    }
    
    fn reemplazar_car(&mut self, c: char, x:char) {
       for i in 0..self.longitud {
        if self.caracteres[i] ==c {
            self.caracteres[i] = x;
        }
           
       }
    }

    fn remplazar_pos_car(&mut self, posi: usize, nuevochar: char) {
        if posi >=1 && posi <= self.longitud {
            self.caracteres[posi -1] = nuevochar;
        }
    }

    //-------------------------------------------------------------------------------------------------------
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    // muestra la cadena completa carácter por carácter
    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
}
// ── helpers de entrada ──────────────────────────────────────────────
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}
fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

// ── menú ────────────────────────────────────────────────────────────
fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠════════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena          ║");
    println!("║  2. Mostrar cadena                 ║");
    println!("║  3. Longitud                       ║");
    println!("║  4. Obtener carácter (posición)    ║");
    println!("║  5. InvertirCadena                 ║");
    println!("║  6. Caracter que mas se repite     ║");
    println!("║  7. Contar Vocales y consonantes   ║");
    println!("║  8. Remplazar caracter de la cadena║");
    println!("║  9. Remplazar caracter en una posicion║");
    println!("╠══════════════════════════════════╣");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c = Cadena::new(); //definiendo la instancia de tipo Cadena

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada = leer_linea();

                c.limpiar(); // reiniciamos antes de cargar la nueva

                // ── proceso artesanal: carácter por carácter ──
                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obt_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obt_longitud()),

            "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obt_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obt_longitud()),
                    None    => println!("  Posición inválida."),
                }
            }
            "5" => println!(" Cadena Invertida : {}", c.invcad()),
            "6" => c.masrchar(),
            "7" => c.cont_voc_cons(),
            "8" => {
                println!("Ingrese el caracter a cambiar");
                let car  = leer_linea();
                println!("Ingrese el nuevo caracter");
                let xa: String = leer_linea();
                    let c1 = car.trim().chars().next();
                    let c2 = xa.trim().chars().next();
                if let (Some(c1), Some(c2)) = (c1, c2) 
                {
                     if car.trim().chars().count() == 1 && xa.trim().chars().count() == 1 
                        {
                         c.reemplazar_car(c1, c2);
                        } 
                        else 
                        {
                             println!("Error: Debes ingresar exactamente un solo caracter");
                        }
                }
                else 
                {
                            println!("Error: No ingresaste ningún caracter");
                }

                 }
            "9" => {
                       println!(" Ingresa la posicion que desea remplazar (1 = izquierda):");
                          match leer_numero() {
                            Some(pos) if pos >= 1 && pos <= c.obt_longitud() => {
                              println!(" Ingresa el nuevo carácter:");
                               let nuevocar = leer_linea();
                                match nuevocar.chars().next() {
                                    Some(nuevo) => {
                                        c.remplazar_pos_car(pos, nuevo);
                                        println!(" cadena resultante:");
                                        c.mostrar();
                                    }
                                    None => println!("No existe caracterr.")
                                }     
                            }
                            Some(_) => println!(" posicion fuera de rango (1 a {}).",c.obt_longitud()),
                            None => println!(" Posicion invalida "),   
                    }
                }

        "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}