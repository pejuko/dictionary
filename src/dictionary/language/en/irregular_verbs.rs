use crate::dictionary::language::IrregularVerbType;

pub fn build() -> IrregularVerbType {
    let mut verbs = IrregularVerbType::new();

    verbs.insert("arise".to_string(), vec!["arose".to_string(), "arisen".to_string()]);
    verbs.insert("be".to_string(), vec!["was".to_string(), "were".to_string(), "been".to_string(), "am".to_string(), "are".to_string(), "is".to_string()]);
    verbs.insert("bear".to_string(), vec!["bore".to_string(), "borne".to_string()]);

    verbs
}
