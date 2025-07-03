use crate::ml::traits::MLProcessable;

pub struct PublisherRecord {
    pub id: i64,
    pub normalized: String,
    pub variants: Vec<String>,
    // altri campi eventualmente...
}
impl MLProcessable for PublisherRecord {
    fn id(&self) -> i64 {
        self.id
    }
    fn canonical_key(&self) -> &str {
        &self.normalized
    }
    fn variants(&self) -> Vec<String> {
        self.variants.clone()
    }
    fn set_variants(&mut self, variants: Vec<String>) {
        self.variants = variants;
    }
}
