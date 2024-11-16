use std::{collections::HashMap, fmt::Display, hash::Hash, rc::Rc};

use anyhow::anyhow;

pub struct TreeWrapper<T> {
    pub body: T,
    pub children: Vec<TreeWrapper<T>>,
}

fn sort_by_prev<K, T, C, F1, F2, F3>(
    elements: &mut Vec<T>,
    ignore_lost: bool,
    id: F1,
    prev: F2,
    ya_ord: F3,
) where
    K: Eq + Hash + Clone,
    T: Clone,
    C: Ord,
    F1: Fn(&T) -> &K,
    F2: Fn(&T) -> &Option<K>,
    F3: Fn(&T) -> &C,
{
    let mut result = vec![];
    {
        let obj_map: HashMap<K, T> = elements
            .iter()
            .map(|e| (id(e).clone(), e.clone()))
            .collect();
        let mut next_map: HashMap<K, T> = HashMap::new();
        let mut head_nodes: Vec<T> = vec![];

        elements.into_iter().for_each(|cwr| {
            if let Some(prev_id) = prev(cwr) {
                if obj_map.contains_key(prev_id) {
                    next_map.insert(prev_id.clone(), cwr.clone());
                } else if !ignore_lost {
                    head_nodes.push(cwr.clone());
                }
            } else {
                head_nodes.push(cwr.clone());
            }
        });

        head_nodes.sort_by(|c1, c2| ya_ord(c1).cmp(ya_ord(c2)));

        for ele in head_nodes {
            result.push(ele.clone());
            let mut p = &ele;

            while let Some(n) = next_map.get(id(p)) {
                result.push(n.clone());
                p = n;
            }
        }
    }

    if !ignore_lost {
        elements.swap_with_slice(&mut result);
    } else {
        elements.clear();
        elements.extend(result);
    }
}

pub fn build_trees<K, T, C, F1, F2, F3, F4>(
    elements: Vec<T>,
    ignore_lost: bool,
    id: F1,
    prev: F2,
    parent: F3,
    ya_ord: F4,
) -> Vec<TreeWrapper<T>>
where
    K: Eq + Hash + Clone + Display,
    C: Ord,
    F1: Fn(&T) -> &K,
    F2: Fn(&T) -> &Option<K>,
    F3: Fn(&T) -> &Option<K>,
    F4: Fn(&T) -> &C,
{
    // parent_map and top_levels are orthogonal.
    // so we could try unwrap them from Rc boxes.
    let mut parent_map: HashMap<K, Vec<Rc<T>>> = HashMap::new();
    let mut top_levels = vec![];

    {
        let ele_map: HashMap<K, Rc<T>> = elements
            .into_iter()
            .map(|e| (id(&e).clone(), Rc::new(e)))
            .collect();

        for ele in ele_map.values() {
            if let Some(pid) = parent(ele) {
                if ele_map.contains_key(pid) {
                    let vec = parent_map
                        .entry(pid.to_owned())
                        .or_insert_with(|| Vec::new());
                    vec.push(ele.to_owned());
                } else if !ignore_lost {
                    top_levels.push(ele.clone());
                }
            } else {
                top_levels.push(ele.clone());
            }
        }

        parent_map.values_mut().for_each(|v| {
            sort_by_prev(
                v,
                ignore_lost,
                |e| id(e.as_ref()),
                |e| prev(e.as_ref()),
                |e| ya_ord(e.as_ref()),
            )
        });
    }

    fn put_in_parent<K, T, F1>(
        parent: Rc<T>,
        parent_map: &mut HashMap<K, Vec<Rc<T>>>,
        id: F1,
    ) -> TreeWrapper<T>
    where
        F1: Fn(&T) -> &K,

        K: Eq + Hash + Clone + Display,
    {
        let ext = |e: Rc<T>| {
            Rc::<T>::try_unwrap(e)
                .map_err(|e| anyhow!("unable to read {}", id(e.as_ref())))
                .unwrap()
        };

        if parent_map.is_empty() {
            return TreeWrapper {
                body: ext(parent),
                children: vec![],
            };
        }

        let children: Vec<TreeWrapper<T>> =
            if let Some(children) = parent_map.remove(&id(parent.as_ref())) {
                children
                    .into_iter()
                    .map(|ele| put_in_parent(ele, parent_map, &id))
                    .collect()
            } else {
                vec![]
            };

        TreeWrapper {
            body: ext(parent),
            children,
        }
    }

    top_levels
        .into_iter()
        .map(|e| put_in_parent(e, &mut parent_map, |e| id(e)))
        .collect()
}

fn put_in_parent<K, T, F1>(
    parent: Rc<T>,
    parent_map: &mut HashMap<K, Vec<Rc<T>>>,
    id: F1,
) -> TreeWrapper<T>
where
    F1: Fn(&T) -> &K,
    K: Eq + Hash + Clone + Display,
{
    let ext = |e: Rc<T>| {
        Rc::<T>::try_unwrap(e)
            .map_err(|e| anyhow!("unable to read {}", id(e.as_ref())))
            .unwrap()
    };

    let mut stack = vec![(parent, Vec::new())];
    let mut result = None;

    while let Some((current, mut children)) = stack.pop() {
        if let Some(child_nodes) = parent_map.remove(id(current.as_ref())) {
            for child in child_nodes.into_iter().rev() {
                stack.push((child, Vec::new()));
            }
        }

        let wrapper = TreeWrapper {
            body: ext(current),
            children: children.into_iter().rev().collect(),
        };

        if stack.is_empty() {
            result = Some(wrapper);
        } else {
            stack.last_mut().unwrap().1.push(wrapper);
        }
    }

    result.unwrap()
}
