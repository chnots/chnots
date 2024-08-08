use std::{borrow::Cow, collections::HashMap, sync::Arc};

use super::db::chnot::Domain;

pub struct Domains {
    pub managers: Arc<HashMap<String, Arc<Vec<String>>>>,
    pub managed: Arc<HashMap<String, Arc<Vec<String>>>>,
}

impl Domains {
    /// TODO, db
    pub fn new() -> Self {
        let public: Cow<str> = Cow::Owned("public".to_owned());
        let work: Cow<str> = Cow::Owned("work".to_owned());
        let private: Cow<str> = Cow::Owned("private".to_owned());

        let domains = vec![
            Domain {
                manager: vec![work.clone(), private.clone()],
                name: public.clone(),
            },
            Domain {
                manager: vec![private.clone()],
                name: work.clone(),
            },
            Domain {
                manager: vec![],
                name: private.clone(),
            },
        ];

        let original_map: HashMap<String, Domain> = domains
            .into_iter()
            .map(|e| (e.name.to_string(), e))
            .collect();

        let mut managed = HashMap::new();
        let mut managers = HashMap::new();

        for (k, d) in original_map.iter() {
            {
                let m = managed.get_mut(k);
                if m.is_none() {
                    managed.insert(k.to_owned(), vec![k.to_owned()]);
                } else {
                    m.unwrap().push(k.clone());
                }
            }

            for p in &d.manager {
                let d = d.name.clone();
                let m = managed.get_mut(p.as_ref());
                if m.is_none() {
                    managed.insert(p.to_string(), vec![d.to_string()]);
                } else {
                    m.unwrap().push(d.to_string());
                }
            }

            managers.insert(
                k.to_string(),
                d.manager
                    .iter()
                    .filter_map(|e| original_map.get(e.as_ref()).map(|v| v.name.to_string()))
                    .collect(),
            );
        }

        Self {
            managers: Arc::new(
                managers
                    .into_iter()
                    .map(|(k, v)| {
                        let mut v: Vec<String> = v;
                        v.push(k.clone());
                        (k.to_lowercase(), Arc::new(v))
                    })
                    .collect(),
            ),
            managed: Arc::new(
                managed
                    .into_iter()
                    .map(|(k, domain_vec)| {
                        let mut combined = domain_vec
                            .iter()
                            .map(|domain| domain.to_string())
                            .collect::<Vec<String>>();

                        combined.push(k.to_string());

                        (k.to_lowercase(), combined.into())
                    })
                    .collect(),
            ),
        }
    }

    pub fn managed(&self, domain: &str) -> Arc<Vec<String>> {
        self.managed
            .get(domain.to_lowercase().as_str())
            .map_or(Arc::new(vec![]), |e| e.clone())
    }

    pub fn managers(&self, domain: &str) -> Arc<Vec<String>> {
        self.managers
            .get(domain.to_lowercase().as_str())
            .map_or(Arc::new(vec![]), |e| e.clone())
    }
}
