use crate::error::RegistryError;

pub mod fs;
mod manifest;

pub use manifest::*;

pub trait Store {
    fn put_manifest(&self, namespace: &str, reference: &str, m: &Manifest) -> Result<(), RegistryError>;
    fn get_manifest(&self, namespace: &str, reference: &str) -> Result<Manifest, RegistryError>;
    fn list_tags(&self, namespace: &str) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store_puts_and_gets(s: &dyn Store) {
        let m = Manifest::default();
        s.put_manifest("namespace", "reference", &m).unwrap();
        let m2 = s.get_manifest("namespace", "reference").unwrap();
        assert_eq!(m, m2);
    }

    fn store_lists_tags(s: &dyn Store) {
        let m = Manifest::default();
        s.put_manifest("tags", "one", &m).unwrap();
        s.put_manifest("tags", "two", &m).unwrap();
        s.put_manifest("tags", "three", &m).unwrap();
        // Will be lexicographically sorted for fstore
        assert_eq!(s.list_tags("tags"), vec!["one", "three", "two"])
    }

    fn stores_by_digest_and_tag(s: &dyn Store) {
        let mut m = Manifest::default();
        let mut anno = std::collections::HashMap::new();
        anno.insert(String::from("foo"), String::from("bar"));
        m.annotations = Some(anno);
        s.put_manifest("namespace", "tag", &m).unwrap();
        let m2 = s.get_manifest("namespace", &m.digest()).unwrap();
        assert_eq!(m, m2);
    }

    fn allow_overwrite_tag(s: &dyn Store) {
        let m = Manifest::default();
        s.put_manifest("replace", "latest", &m).unwrap();
        s.put_manifest("replace", "latest", &m).unwrap();
    }

    #[test]
    fn fs_store_tests() {
        let test_path = format!("/tmp/blobert-test/{}", uuid::Uuid::new_v4());
        let fstore = fs::Filesystem::new(&test_path).unwrap();
        store_puts_and_gets(&fstore);
        store_lists_tags(&fstore);
        stores_by_digest_and_tag(&fstore);
        allow_overwrite_tag(&fstore);
    }
}
