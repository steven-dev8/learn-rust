// Part1: Struct vs Enum
// struct Mago {mana: u8}
// struct Guerreiro {forca: u8}
// struct Arqueiro {precisao: u8}

// Definição de Enum:
// Enums é uma coleção de tipos que permitem compartilhar os mesmos métodos,
// com ou sem comportamentos diferentes (match)

// Nos structs comentados acima, eu teria que implementar manualmente os metódos
// para cada um deles, o que seria muito ineficiênte
enum Classes {
    Mago { mana: u8 }, // struct-like são structs dentro de Enums
    Guerreiro { forca: u8 },
    Arqueiro { precisao: u8 },
}

// Enums aceitam uma gama de estruturas de dados
enum IpAddr {
    V4(u8, u8, u8, u8), // tupla
    V6(String) // String
}

// Implementando métodos no Enum
impl IpAddr { 
    fn connect(&self) {
        // Diferentes condições para váriados tipos
        match self { 
            IpAddr::V4(w, x, y, z) => println!("Connecting to Ipv4 {w}.{x}.{y}.{z}"),
            IpAddr::V6(ip) => println!("Connecting to Ipv6 {ip}"),
        };
    }
    // Método que se aplica a todos os tipos
    fn desconnect_all(&self) {
        println!("Desconnecting connections to IP");
    }
} 

fn main() {
    let ip1 = IpAddr::V4(1,0,0,127);
    ip1.connect();
    ip1.desconnect_all();

    let player = Classes::Mago{mana:100};
}