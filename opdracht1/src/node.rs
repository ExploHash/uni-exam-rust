
pub struct Node<'a, T> {
  item: T,
  parent: Option<&'a Node<'a, T>>,
  children: Vec<&'a Node<'a, T>>
}

impl<T: PartialEq + PartialOrd> Node<'_, T> {
  pub fn new(item: T) -> Self {
    Node {
      item,
      parent: None,
      children: Vec::new()
    }
  }

  fn value(&self) -> &T {
    &self.item
  }

  pub fn add_child<'a>(&mut self, node: &Node<T>){
    if self.has_in_tree(node) {
      print!("Already has child in tree");
      return;
    }
    
    self.children.push(node);
  }

  pub fn has_in_tree(&self, node: &Node<T>) -> bool {
    //Get root node
    let root = self.get_root_node();

    self.has_in_tree_overloaded(root, node)
  }

  fn has_in_tree_overloaded(&self, parent: &Node<T>, node: &Node<T>) -> bool {
    for child_node in parent.children.iter() {
      if child_node.item == node.item || self.has_in_tree_overloaded(parent, node) {
        return true;
      }      
    }

    return false;
  }

  pub fn get_root_node(&self) -> &Node<T> {
    let mut root = self;
    while self.parent.is_some() {
      root = self.parent.unwrap()
    }

    return root;
  }

  // pub fn has_direct_child(&self, node: &Node<T>){
  //   self.has_direct_child_overload(node, 0, self.children.len());
  // }

  // fn has_direct_child_overload(&self, node: &Node<T>, start: usize, end: usize) -> bool {
  //   let count = end - start;
  //   let searchIndex = start + count / 2;

  //   if count == 0 {
  //     return false;
  //   } else if self.children[searchIndex].item == node.item {
  //     return true
  //   } else if self.children[searchIndex].item > node.item {
  //     return self.has_direct_child_overload(node, start, searchIndex - 1)
  //   } else {
  //     return self.has_direct_child_overload(node, searchIndex, end)
  //   }
  // }

  pub fn remove_child(&self, item: &Node<T>){

  }
}

impl Node<'_, String> {
  pub fn print_tree(&self){
    self.print_tree_overload("", Some(self), false, true);
  }

  fn print_tree_overload(&self, prefix: &str, node: Option<&Node<String>>, is_left: bool, is_first: bool){
    if node.is_none() {
      return;
    }

    if !is_first {
      println!("{}", prefix);
      if is_left {
        println!("{}", "├──");
      } else {
        println!("{}", "└──");
      }
    }

    println!("{}", node.unwrap().value());

    let mut new_prefix: String = prefix.to_owned();
    
    if !is_first {
      if is_left {
        new_prefix.push_str("│  ");
      }else {
        new_prefix.push_str("   ");
      }
    }

    let mut to_print: Option<&Node<String>> = None;

    for child in node.unwrap().children.iter() {
      self.print_tree_overload(&new_prefix, to_print, true, false);
      to_print = Some(child);
    }

    self.print_tree_overload(&new_prefix, to_print, false, false);
  }
}

