//// 2023/09/28 // 20:35 //

// #9 Estructuras (structs)

// Tipo Estructuras

// Una estructura es un tipo de datos personalizado que te va a permite nombrar y
// emaquetar varios valores relacionados que forman un grupo con significado.

// Si estás familiarizado con un lenguaje orientado a objetos, una estructura es
// como los atributos de datos de un objeto


// Estructuras vs Tuplas

// Las estructuras son más flexibles que las tuplas,
// ya que no dependen del orden de los datos para acceder a ellos.

// Cada struct tiene un nombre.

// Definición de una estructura

// Para definir una estructura, ingresamos la palabra clave struct seguida del
// nombre que demos a la estructura, debiendo ser un nombre descriptivo que
// defina perfectamente los datos que va a lmacenar. Después, entre llaves,
// definiremos los nombres y tipos de los datos que la van a conformar, a los que
// llamamos campos.

struct Usuario {
    nombre: String,
    email: String,
    edad: u8,
    activo: bool,
}

fn main() {

    let mut usuario = Usuario {
        nombre: String::from("Luis"),
        email: String::from("myemail@gmail.com"),
        edad: 47,
        activo: true,
    };

    usuario.email = String::from("nuevo@prueba.com");

    // Estructuras mutables

    // Toda la instancia debe ser mutable; Rust no nos permite marcar solo ciertos
    // campos como mutables.

    let usuario2 = nuevo_usuario(String::from("Pavel"), String::from("EMAIL@GMAIL.new"));

    let usuario3 = Usuario {
        nombre: String::from("Juanito García"),
        email: String::from("newemailthree@mail3.com"),
        ..usuario
    };

    // Estructuras de tupla
    // Las estructuras de tupla son un híbrido entre ambos tipos de datos: tienen el
    // significado que proporciona el nombre de la estructura, pero no tienen nombres
    // sus campos; o sea, solo tiene number el tipo de dato.

    let negro = Color(0,0,0);
    let origen = Coordenada(0,0,0);
    println!("The black color has {} light", negro.0);
    println!("The initial coordinates are {} {} {}", origen.0, origen.1, origen.2);


    // Estructuras de tipo unidad
    // Aunque nos parezca difícil de creer... ¡podremos definir estructuras que no tengan
    // campos! Las estructuras tipo unidad pueden ser útiles en situaciones en las que
    // necesita implementar un rasgo en algún tipo pero no tiene ningún dato que
    // desee almacenar en el tipo en sí.

    let ancho1 = 15;
    let alto1 = 26;
    let area1 = area(ancho1, alto1);
    println!("El área del rectángulo es: {}", area1);

    let rect2 = (15,24);
    let area2 = area2(rect2);
    println!("El área del rectángulo 2 es: {}", area2);

    let rect3 = Rectangulo {
        ancho: 15,
        alto: 34
    };

    let area3 = area3(&rect3);
    println!("El área del rectángulo es: {}", area3);

}

fn nuevo_usuario (nombre: String, email: String) -> Usuario {
    Usuario {
        nombre: nombre,
        email: email,
        edad: 0,
        activo: true,
    }
}


struct Color (i32, i32, i32);
struct Coordenada (i32, i32, i32);


// ÁREA RECTÁNGULO: SOLUCIÓN 1

fn area(ancho: u32, alto: u32) -> u32 {
    ancho * alto
}

fn area2(dimensionses: (u32, u32)) -> u32 {
    dimensionses.0 * dimensionses.1
}

fn area3(rectangulo: &Rectangulo) -> u32 {
    rectangulo.alto * rectangulo.ancho
}

struct Rectangulo {
    ancho: u32,
    alto: u32,
}


