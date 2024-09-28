use std::collections::HashMap;

use chrono::DateTime;

use crate::{
    model::v1::dto::NestedChnot,
    utils::sort_util::{build_trees, TreeWrapper},
};

use super::dto::{ChnotRing, ChnotWithRelation};

pub fn construct_rings(elements: Vec<ChnotWithRelation>, ignore_lost: bool) -> Vec<ChnotRing> {
    let mut rings = HashMap::new();

    for ele in elements {
        rings
            .entry(ele.rind_id.clone())
            .or_insert_with(Vec::new)
            .push(ele);
    }

    rings
        .into_iter()
        .map(|(ring_id, vec)| build_one_chnot_ring(vec, ignore_lost, ring_id))
        .collect()
}

fn build_one_chnot_ring(
    elements: Vec<ChnotWithRelation>,
    ignore_lost: bool,
    ring_id: String,
) -> ChnotRing {
    let mut init_time = DateTime::from_timestamp(i64::MAX, 0)
        .unwrap()
        .fixed_offset();
    let mut update_time = DateTime::from_timestamp(0, 0).unwrap().fixed_offset();

    for cwr in elements.iter() {
        if cwr.init_time < init_time {
            init_time = cwr.init_time.clone();
        }

        if cwr.insert_time > update_time {
            update_time = cwr.insert_time.clone();
        }
    }

    fn to_nest(tree: TreeWrapper<ChnotWithRelation>) -> NestedChnot {
        NestedChnot {
            chnot: tree.body.chnot,
            children: tree.children.into_iter().map(|e| to_nest(e)).collect(),
        }
    }

    let trees = build_trees(
        elements,
        ignore_lost,
        |e| &e.id,
        |e: &ChnotWithRelation| &e.prev_id,
        |e| &e.parent_id,
        |e| &e.insert_time,
    )
    .into_iter()
    .map(|t| to_nest(t))
    .collect();

    ChnotRing {
        chnots: trees,
        ring_id: ring_id.to_string(),
        r#type: super::db::chnot::ChnotType::MarkdownWithToent,
        init_time,
        update_time,
    }
}
