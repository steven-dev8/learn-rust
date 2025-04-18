#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // PART 1

    // Com atributos sem vinculo
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // Com tupla
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
    
    // Com struct
    let rect2 = Rect {width: 30, height: 50};

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );

    // PART 2: Formatação de Struct com println!
    println!("rect2 is {rect2:?}")
    // Para formatação e exibição de Struct no println!, devemos especificar
    // algumas coisas no nosso código.
    // 1. adicionar um atributo de derive chamado que recebe Debug na linha anterior
    // da assinatura do struct
    // 2. Na formatação da instancia do struct, você pode usar {:?} ou para structs
    // mais complexas {:#?}
    // Esse tipo de formatação é ótima para debug do código.
    // Há uma segunda opção que é com dbg!
}

// Valores não relacionados
fn area1(width: u32, height: u32) -> u32 {
    width * height
}
// Valores relacionados
fn area2(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}
// Valores relacionados e semânticos
fn area3(dimension: &Rect) -> u32 {
    dimension.width * dimension.height
}