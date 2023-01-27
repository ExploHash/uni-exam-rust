mod node;

use crate::node::Node;

fn main() {
    let mut test: Test = Test::new();
    test.test_all();
}

struct Test<'a> {
    a: Node<'a, String>,
    b: Node<'a, String>,
    c: Node<'a, String>,
    d: Node<'a, String>,
    e: Node<'a, String>,
    f: Node<'a, String>,
    g: Node<'a, String>,
    k: Node<'a, String>,
    m: Node<'a, String>
}

impl Test<'_> {
    pub fn new() -> Self {
        Test {
            a: Node::new("a".to_string()),
            b: Node::new("b".to_string()),
            c: Node::new("c".to_string()),
            d: Node::new("d".to_string()),
            e: Node::new("e".to_string()),
            f: Node::new("f".to_string()),
            g: Node::new("g".to_string()),
            k: Node::new("k".to_string()),
            m: Node::new("m".to_string())
        }
    }

    fn setup(&mut self) {
        self.a = Node::new("a".to_string());
        self.b = Node::new("b".to_string());
        self.c = Node::new("c".to_string());
        self.d = Node::new("d".to_string());
        self.e = Node::new("e".to_string());
        self.f = Node::new("f".to_string());
        self.g = Node::new("g".to_string());
        self.k = Node::new("k".to_string());
        self.m = Node::new("m".to_string());
    }

    fn test_all(&mut self){
        println!();
        self.test_normal();
        self.test_insert_twice();
        self.test_insert_delete_insert();
    }

    fn test_normal(&mut self){
        println!("{}", "Test normal tree");
        println!("{}", "=================");
        self.setup();

        self.a.add_child(&self.b);
        self.a.add_child(&self.c);
        self.b.add_child(&self.k);
        self.b.add_child(&self.m);
        self.c.add_child(&self.d);
        self.d.add_child(&self.e);
        self.d.add_child(&self.f);
        self.d.add_child(&self.g);


        self.a.print_tree();

        println!();
    }

    fn test_insert_twice(&mut self){
        println!("{}", "Test add twice");
        println!("{}", "==================");

        self.setup();

        self.a.add_child(&self.b);
        self.a.add_child(&self.c);
        self.c.add_child(&self.e);
        self.c.add_child(&self.d);
        self.b.add_child(&self.e);

        self.a.print_tree();

        println!();
    }

    fn test_insert_delete_insert(&mut self){
        println!("{}", "Test add, remove, add");
        println!("{}", "==========================");

        self.setup();

        self.a.add_child(&self.b);
        self.a.add_child(&self.c);
        self.c.add_child(&self.e);
        self.c.add_child(&self.d);

        self.c.remove_child(&self.e);

        self.b.add_child(&self.e);

        self.a.print_tree();

        println!();
    }


}