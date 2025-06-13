impl MLNameLearner {
    /// Applica un feedback di falso positivo: riduce la confidenza o rimuove il pattern tra due nomi
    pub fn apply_false_positive(&mut self, name1: &str, name2: &str) {
        self.learned_patterns.retain(|pattern| {
            !(pattern.base_form == name1 && pattern.variant_form == name2)
                && !(pattern.base_form == name2 && pattern.variant_form == name1)
        });
        // Puoi anche abbassare solo la confidenza invece di rimuovere del tutto
    }

    /// Applica un feedback di falso negativo: aggiunge o rafforza il pattern tra due nomi
    pub fn apply_false_negative(&mut self, name1: &str, name2: &str) {
        // Usa già la logica incrementale esistente
        let _ = self.add_observed_variant(name1, name2, 1.0);
    }
}