use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // Neste exemplo, o node leaf é criado sem filhos e sem um pai

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    // Printamos quantas R<T> (Referências fortes) leaf tem e 
    // quantas Weak<T> (Referências fracas) leaf tem
    // Saída esperada: 1, 0

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Dentro desse escopo o node branch é criado e leaf (node filho)
        // é atribuido no attr children

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // Atribuímos um pai para para leaf, que é o node branch, 
        // através do Rc::downgrade() que retorna uma Weak<T>
        // Dessa forma, associamos branch como pai de leaf no attr parent

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // Weak<T> é um ponteiro de referência fraca de Rc<T>
    // Ele é utilizado pra evitar ciclos de referência
    // Métodos
    // Rc::downgrade(&rc) cria uma Weak<T> (referência fraca) a partir de uma Rc<T> (referência forte)
    // Essa referência fraca não impede que o valor original seja dropado
    // É usada principalmente para evitar ciclos de referência, como em estruturas em árvore ou grafo

    // weak.upgrade() tenta transformar uma Weak<T> de volta em Rc<T>
    // Retorna Some(Rc<T>) se o valor ainda existe (ou seja, se há pelo menos uma referência forte viva)
    // Retorna None se todas as Rc<T> já foram dropadas, ou seja, o valor foi desalocado da memória

    // weak.upgrade() desreferência de maneira segura o Weak<T>, tentando transformar em um Rc<T>
    // para isso ele retornar um Option<Rc<T>>

    // Rc::weak_count(&rc) retorna quantas referências fracas (Weak<T>) existem para aquele Rc<T>
    // Essas referências fracas não contam para manter o valor vivo, apenas apontam para ele


    // Uma Weak<T> é um referência fraca derivada de Rc<T>, se o número de
    // referências fortes for 0, o dado é dropado, independente do número de
    // referências fracas, isso impede do valor ser dropado, pois quando chega no
    // final do escopo, não existem referências recursivas que impedem daquele dado
    // na memória ser libero
}
