use crate::interpret::branching_scope::Node::Child;
use crate::interpret::value::Value;
use std::collections::HashMap;
use std::fmt::Debug;

type Scope = HashMap<String, Value>;

#[derive(Debug)]
pub enum Node {
  Base,
  Child {
    data: Scope,
    parent: usize,
    ref_count: usize,
  },
}

impl Node {
  fn data_mut(&mut self) -> Option<&mut Scope> {
    match self {
      Node::Base => None,
      Child { data, .. } => Some(data),
    }
  }

  // fn ref_count(&self) -> usize {
  //   match self {
  //     Node::Base => 9999,
  //     Child { ref_count, .. } => *ref_count
  //   }
  // }
}

pub struct BranchingScope {
  nodes: HashMap<usize, Node>,
  current: usize,
}

impl BranchingScope {
  pub fn empty() -> BranchingScope {
    let mut nodes = HashMap::new();
    nodes.insert(0, Node::Base);
    BranchingScope { nodes, current: 0 }
  }

  fn add_ref_to_node(&mut self, id: usize) {
    let current_node = self.nodes.get_mut(&id).unwrap();
    match current_node {
      Node::Base => {}
      Child { ref_count, .. } => *ref_count += 1,
    }
  }

  fn remove_ref_from_node(&mut self, id: usize) {
    let current_node = self.nodes.get_mut(&id).unwrap();
    match current_node {
      Node::Base => {}
      Child { ref_count, .. } => {
        *ref_count -= 1;
      }
    }
  }

  pub fn branch(&mut self, src: usize) -> usize {
    self.current += 1;
    self.nodes.insert(
      self.current,
      Child {
        data: HashMap::new(),
        parent: src,
        ref_count: 0,
      },
    );
    self.add_ref_to_node(src);
    self.current
  }

  pub fn release(&mut self, id: usize) -> usize {
    let current_node = self.nodes.get(&id).unwrap();
    let (ref_count, parent) = match current_node {
      Node::Base => unreachable!(),
      Child {
        ref_count, parent, ..
      } => (*ref_count, *parent),
    };

    if ref_count == 0 {
      self.nodes.remove(&id);
      self.remove_ref_from_node(parent);
    }

    parent
  }

  fn scope_mut(&mut self, id: usize) -> Option<&mut Scope> {
    self.nodes.get_mut(&id).and_then(|n| n.data_mut())
  }

  fn find_first_with_key(&self, id: usize, key: &str) -> Option<&Scope> {
    let mut current = self.nodes.get(&id)?;

    while let Child { data, parent, .. } = current {
      if data.contains_key(key) {
        return Some(data);
      }
      current = self.nodes.get(parent)?;
    }
    None
  }

  fn find_first_with_key_mut(&mut self, id: usize, key: &str) -> Option<&mut Scope> {
    let mut current = id;
    loop {
      match self.nodes.get(&current)? {
        Node::Base => return None,
        Child { data, parent, .. } => {
          if data.contains_key(key) {
            break;
          } else {
            current = *parent
          }
        }
      };
    }
    self.scope_mut(current)
  }

  pub fn get(&self, id: usize, key: &str) -> Option<&Value> {
    self.find_first_with_key(id, key).and_then(|s| s.get(key))
  }

  pub fn define(&mut self, id: usize, key: &str, value: Value) {
    self
      .scope_mut(id)
      .and_then(|s| s.insert(key.to_string(), value));
  }

  pub fn assign(&mut self, id: usize, key: &str, value: Value) -> Option<()> {
    let s = self.find_first_with_key_mut(id, key)?;
    s.insert(key.to_string(), value);
    Some(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn branch_with(scope: &mut BranchingScope, base: usize, key: &str, value: f64) -> usize {
    let id = scope.branch(base);
    scope.define(id, key, Value::Number(value));
    id
  }

  #[test]
  fn get_when_value_defined_at_top_it_finds_it() {
    let mut list = BranchingScope::empty();
    let branch1 = branch_with(&mut list, 0, "key", 10.0);
    let branch2 = branch_with(&mut list, branch1, "key", 20.0);
    let branch3 = branch_with(&mut list, branch2, "key", 30.0);

    let value = list.get(branch3, "key").unwrap();

    assert_eq!(*value, Value::Number(30.0));
  }

  #[test]
  fn get_when_value_defined_at_parent_it_find_it() {
    let mut list = BranchingScope::empty();
    let branch1 = branch_with(&mut list, 0, "key", 10.0);
    let branch2 = branch_with(&mut list, branch1, "key2", 20.0);
    let branch3 = branch_with(&mut list, branch2, "key3", 20.0);

    let value = list.get(branch3, "key").unwrap();

    assert_eq!(*value, Value::Number(10.0));
  }

  #[test]
  fn get_when_key_is_not_defined_it_does_not_find_it() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = list.branch(branch1);
    let branch3 = list.branch(branch2);

    let value = list.get(branch3, "bar");

    assert!(value.is_none());
  }

  #[test]
  fn get_do_not_see_keys_in_sibling_node() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let _branch2 = list.branch(branch1);
    let branch3 = list.branch(branch1);

    let value = list.get(branch3, "bar");

    assert!(value.is_none());
  }

  #[test]
  fn define_creates_value_at_current_level() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = list.branch(branch1);

    list.define(branch2, "foo", Value::Number(3.0));
    let value = list.get(branch2, "foo").unwrap();

    assert_eq!(*value, Value::Number(3.0));
  }

  #[test]
  fn define_does_not_create_values_at_parent_levels() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = list.branch(branch1);

    list.define(branch2, "foo", Value::Number(3.0));
    let value = list.get(branch1, "foo");

    assert!(value.is_none());
  }

  #[test]
  fn define_can_create_a_value_already_present_at_parent_level() {
    let mut list = BranchingScope::empty();
    let branch1 = branch_with(&mut list, 0, "foo", 5.0);
    let branch2 = list.branch(branch1);

    list.define(branch2, "foo", Value::Number(3.1));
    let value = list.get(branch1, "foo").unwrap();
    assert_eq!(*value, Value::Number(5.0));
    let value = list.get(branch2, "foo").unwrap();
    assert_eq!(*value, Value::Number(3.1));
  }

  #[test]
  fn define_can_create_use_the_same_key_twice() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = list.branch(branch1);

    list.define(branch2, "foo", Value::Number(3.1));
    let value = list.get(branch2, "foo").unwrap();
    assert_eq!(*value, Value::Number(3.1));
    list.define(branch2, "foo", Value::String("another".to_string()));
    let value = list.get(branch2, "foo").unwrap();
    assert_eq!(*value, Value::String("another".to_string()));
  }

  #[test]
  fn assign_when_variable_define_at_top_level_updates_the_value() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = branch_with(&mut list, branch1, "foo", 2.0);

    list.assign(branch2, "foo", Value::Number(3.1)).unwrap();
    let res = list.get(branch2, "foo").unwrap();
    assert_eq!(*res, Value::Number(3.1));
  }

  #[test]
  fn assign_when_variable_defined_at_perent_level_updates_the_value() {
    let mut list = BranchingScope::empty();
    let branch1 = branch_with(&mut list, 0, "foo", 2.0);
    let branch2 = list.branch(branch1);

    list.assign(branch2, "foo", Value::Number(3.1)).unwrap();
    let res = list.get(branch2, "foo").unwrap();
    assert_eq!(*res, Value::Number(3.1));
  }

  #[test]
  fn assign_when_variable_is_not_defined_returns_none() {
    let mut list = BranchingScope::empty();
    let branch1 = list.branch(0);
    let branch2 = list.branch(branch1);

    let res = list.assign(branch2, "foo", Value::Number(3.1));
    assert!(res.is_none());
  }
}
