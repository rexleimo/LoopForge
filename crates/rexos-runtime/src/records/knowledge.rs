#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct KnowledgeEntityRecord {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) entity_type: String,
    pub(crate) properties: serde_json::Map<String, serde_json::Value>,
    pub(crate) created_at: i64,
    pub(crate) updated_at: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct KnowledgeRelationRecord {
    pub(crate) id: String,
    pub(crate) source: String,
    pub(crate) relation: String,
    pub(crate) target: String,
    pub(crate) properties: serde_json::Map<String, serde_json::Value>,
    pub(crate) created_at: i64,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct KnowledgeAddEntityToolArgs {
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) name: String,
    pub(crate) entity_type: String,
    #[serde(default)]
    pub(crate) properties: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct KnowledgeAddRelationToolArgs {
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) source: String,
    pub(crate) relation: String,
    pub(crate) target: String,
    #[serde(default)]
    pub(crate) properties: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, serde::Deserialize)]
pub(crate) struct KnowledgeQueryToolArgs {
    pub(crate) query: String,
}
