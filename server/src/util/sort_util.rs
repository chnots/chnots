use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    
};

pub struct TreeWrapper<T> {
    pub body: T,
    pub children: Vec<TreeWrapper<T>>,
}

impl<T> From<T> for TreeWrapper<T> {
    fn from(value: T) -> Self {
        Self {
            body: value,
            children: vec![],
        }
    }
}

pub fn sort_by_prev<K, T, C, F1, F2, F3>(
    elements: &mut Vec<T>,
    ignore_lost: bool,
    id: F1,
    prev: F2,
    ya_ord: F3,
) where
    K: Eq + Hash + Clone,
    C: Ord,
    F1: Fn(&T) -> &K,
    F2: Fn(&T) -> &Option<K>,
    F3: Fn(&T) -> &C,
{
    let mut sorted = vec![];
    {
        let ele_map: HashMap<K, &T> = elements.iter().map(|e| (id(e).to_owned(), e)).collect();
        let mut next_map: HashMap<K, &T> = HashMap::new();
        let mut head_nodes: Vec<&T> = vec![];

        elements.iter().for_each(|ele| {
            if let Some(prev_id) = prev(ele) {
                // This element has previous node
                if ele_map.contains_key(prev_id) {
                    // We could find the previous node in the original vec.
                    next_map.insert(prev_id.clone(), ele);
                } else if !ignore_lost {
                    // Even the previous node, if the previous node is lost, we sort by another sort method.
                    head_nodes.push(ele);
                }
            } else {
                head_nodes.push(ele);
            }
        });

        head_nodes.sort_by(|c1, c2| ya_ord(c1).cmp(ya_ord(c2)));

        for ele in head_nodes {
            sorted.push(ele);
            let mut p = &ele;

            while let Some(n) = next_map.get(id(p)) {
                sorted.push(n);
                p = n;
            }
        }
    }

    let mut new_index_map = HashMap::new();
    sorted.iter().enumerate().for_each(|(index, ele)| {
        new_index_map.insert(id(ele).to_owned(), index);
    });

    // We need only retain elements in the new index map.
    elements.retain(|e| new_index_map.contains_key(id(e)));

    elements.sort_by_cached_key(|e| *new_index_map.get(id(e)).unwrap_or(&0));
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
    let mut filled: HashMap<K, TreeWrapper<T>> = Default::default();
    let mut ele_vec: Vec<(TreeWrapper<T>, Vec<K>)> = Default::default();
    {
        let mut parent_to_subs = HashMap::new();

        for ele in elements.iter() {
            let id = id(ele);
            let parent = parent(ele);
            if let Some(parent) = parent {
                parent_to_subs
                    .entry(parent.to_owned())
                    .or_insert(vec![])
                    .push(id.to_owned());
            }
        }

        for ele in elements {
            let key = id(&ele).to_owned();
            if let Some(subs) = parent_to_subs.remove(&key) {
                ele_vec.push((TreeWrapper::from(ele), subs));
            } else {
                filled.insert(key, TreeWrapper::from(ele));
            }
        }
    }

    let mut old_size = 0;
    while old_size != ele_vec.len() || ele_vec.is_empty() {
        old_size = ele_vec.len();
        for (wrapper, vec) in ele_vec.iter_mut() {
            vec.retain(|e| {
                if filled.contains_key(e) {
                    if let Some(ele) = filled.remove(e) {
                        wrapper.children.push(ele);
                    }
                    false
                } else {
                    true
                }
            });
        }
        let (fulled, others) = ele_vec.into_iter().partition(|(_, subs)| subs.is_empty());

        ele_vec = others;

        filled.extend(fulled.into_iter().map(|(mut wrapper, _)| {
            sort_by_prev(
                &mut wrapper.children,
                ignore_lost,
                |t| id(&t.body),
                |t| prev(&t.body),
                |t| ya_ord(&t.body),
            );
            (id(&wrapper.body).to_owned(), wrapper)
        }));
    }

    if !ele_vec.is_empty() {
        tracing::error!("There are some single nodes in the vec");
    }

    let mut result = filled.into_values().collect();
    sort_by_prev(
        &mut result,
        ignore_lost,
        |t| id(&t.body),
        |t| prev(&t.body),
        |t| ya_ord(&t.body),
    );

    result
}
