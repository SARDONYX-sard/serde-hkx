/// Trait to create a state transition tree
use crate::HavokSort as _;
use havok_serde::HavokClass;
use havok_types::Pointer;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Trait to create a state transition tree
pub trait HavokTree {
    /// Tree of the order in which to serialize as binary data.
    fn tree_for_bytes(&mut self) -> String;
    /// Tree of the order in which to serialize as XML.
    fn tree_for_xml(&mut self) -> String;
}

#[derive(Debug)]
struct Node {
    name: String,
    deps: Vec<usize>,
}

impl Node {
    fn new(name: impl Into<String>, deps: Vec<usize>) -> Self {
        Node {
            name: name.into(),
            deps,
        }
    }
}

fn print_node(
    nodes: &IndexMap<usize, Node>,
    idx: usize,
    depth: usize,
    visited: &mut HashMap<usize, usize>,
    last_children: &mut Vec<bool>,
    result: &mut String,
    already_visited: &mut HashMap<usize, bool>,
) {
    let visit_count = visited.entry(idx).or_insert(0);
    *visit_count += 1;

    let is_already_visited = already_visited.entry(idx).or_insert(false);

    if !*is_already_visited {
        *is_already_visited = true;

        for level in 0..depth {
            if level == depth - 1 {
                if *last_children.last().unwrap_or(&false) {
                    result.push_str("`-- ");
                } else {
                    result.push_str("|-- ");
                }
            } else if *last_children.get(level).unwrap_or(&false) {
                result.push_str("    ");
            } else {
                result.push_str("|   ");
            }
        }

        if let Some(node) = nodes.get(&idx) {
            result.push_str(&node.name);
            result.push('\n');
        } else {
            panic!("Not found key: {}", idx);
        }
    } else if *visit_count > 1 {
        for level in 0..depth {
            if level == depth - 1 {
                if *last_children.last().unwrap_or(&false) {
                    result.push_str("`-- ");
                } else {
                    result.push_str("|-- ");
                }
            } else if *last_children.get(level).unwrap_or(&false) {
                result.push_str("    ");
            } else {
                result.push_str("|   ");
            }
        }
        result.push_str(&format!(
            "{} (visited {} times)\n",
            nodes[&idx].name, visit_count
        ));
    }

    // print visited tree dependencies.
    if let Some(node) = nodes.get(&idx) {
        for (i, &dep) in node.deps.iter().enumerate() {
            last_children.push(i == node.deps.len() - 1);
            print_node(
                nodes,
                dep,
                depth + 1,
                visited,
                last_children,
                result,
                already_visited,
            );
            last_children.pop();
        }
    }
}

impl<V> HavokTree for IndexMap<usize, V>
where
    V: HavokClass,
{
    fn tree_for_bytes(&mut self) -> String {
        self.sort_for_bytes();

        let mut nodes: IndexMap<usize, Node> = IndexMap::new();
        for (&index, class) in self.iter() {
            let non_null_deps = class
                .deps_indexes()
                .into_iter()
                .filter(|&index| index != 0)
                .collect();

            nodes.insert(
                index,
                Node::new(
                    format!("{}({})", class.name(), Pointer::new(index)),
                    non_null_deps,
                ),
            );
        }

        let mut visited = HashMap::new();
        let mut result = String::new();
        let mut already_visited = HashMap::new();
        for &key in nodes.keys() {
            if !already_visited.contains_key(&key) {
                print_node(
                    &nodes,
                    key,
                    0,
                    &mut visited,
                    &mut vec![],
                    &mut result,
                    &mut already_visited,
                );
            }
        }

        result
    }

    // FIXME: This is not possible unless sort can be reproduced.
    fn tree_for_xml(&mut self) -> String {
        String::new()
    }
}

#[test]
#[quick_tracing::init]
fn should_create_tree() {
    let mut classes: crate::prelude::ClassMap = crate::from_str(include_str!(
        "../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml"
    ))
    .unwrap();

    let tree = classes.tree_for_bytes();
    tracing::debug!("tree =\n{tree}");
}
