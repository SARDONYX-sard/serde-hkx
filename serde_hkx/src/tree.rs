/// Trait to create a state transition tree
use crate::HavokSort as _;
use havok_serde::HavokClass;
use havok_types::Pointer;
use indexmap::IndexMap;
use std::collections::HashMap;

/// Trait to create ptr dependencies tree
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
    #[inline]
    const fn new(name: String, deps: Vec<usize>) -> Self {
        Self { name, deps }
    }
}

#[allow(clippy::too_many_arguments)]
fn print_node(
    nodes: &IndexMap<usize, Node>,
    idx: usize,
    depth: usize,
    visited: &mut HashMap<usize, usize>,
    last_children: &mut Vec<bool>,
    result: &mut String,
    path: &mut Vec<usize>,
    already_visited: &mut HashMap<usize, bool>,
) {
    if path.contains(&idx) {
        // Cycle detected
        let cycle_start = path.iter().position(|&x| x == idx).unwrap_or_default();
        let cycle_path: Vec<String> = path[cycle_start..]
            .iter()
            .map(|&i| nodes[&i].name.clone())
            .collect();

        result.push_str(&format!(
            "{}\\_Cycle Start(Invalid state transition): {}\n",
            "| ".repeat(depth + 2),
            nodes[&idx].name,
        ));

        for node_name in cycle_path.iter().skip(1) {
            result.push_str(&format!("{}|-- {}\n", "| ".repeat(depth + 2), node_name));
        }

        result.push_str(&format!(
            "{}    \\_Cycle End: {}\n",
            "| ".repeat(depth + 2),
            nodes[&idx].name,
        ));
        return;
    }

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
            #[cfg(feature = "tracing")]
            tracing::error!("Not found key: {idx}");
            return;
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
            "{} (visited {visit_count} times)\n",
            nodes[&idx].name,
        ));
    }

    // Add the current node to the path
    path.push(idx);

    // Print visited tree dependencies
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
                path,
                already_visited,
            );
            last_children.pop();
        }
    }

    // Remove the current node from the path after processing
    path.pop();
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
        let mut path = Vec::new();
        for &key in nodes.keys() {
            if !already_visited.contains_key(&key) {
                print_node(
                    &nodes,
                    key,
                    0,
                    &mut visited,
                    &mut vec![],
                    &mut result,
                    &mut path,
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

#[cfg_attr(miri, ignore)] // Unexplained hang
#[test]
#[cfg_attr(
    feature = "tracing",
    quick_tracing::init(test = "generate_tree_from_xml", stdio = false)
)]
fn should_create_tree() {
    use crate::tests::ClassMap;

    let s = include_str!("../../docs/handson_hex_dump/wisp_skeleton/skeleton.xml");
    let mut classes: ClassMap = crate::from_str(s).unwrap();
    let tree = classes.tree_for_bytes();
    tracing::debug!("tree =\n{tree}");
}
