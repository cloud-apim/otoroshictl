pub struct EntityHelper {}

impl EntityHelper {
    pub fn extract_json_entity_id(entity: &serde_json::Value) -> Option<String> {
        entity
            .get("id")
            .or_else(|| entity.get("client_id"))
            .or_else(|| entity.get("clientId"))
            .or_else(|| entity.get("username"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("serviceId"))
            .or_else(|| entity.get("randomId"))
            .and_then(|id| id.as_str().map(|v| v.to_string()))
    }

    pub fn extract_yaml_entity_id(entity: &serde_yaml::Value) -> Option<String> {
        entity
            .get("id")
            .or_else(|| entity.get("client_id"))
            .or_else(|| entity.get("clientId"))
            .or_else(|| entity.get("username"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("serviceId"))
            .or_else(|| entity.get("randomId"))
            .and_then(|id| id.as_str().map(|v| v.to_string()))
    }

    pub fn extract_json_entity_name(entity: &serde_json::Value) -> Option<String> {
        entity
            .get("name")
            .or_else(|| entity.get("clientName"))
            .or_else(|| entity.get("client_name"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("label"))
            .or_else(|| entity.get("username"))
            .and_then(|id| id.as_str().map(|v| v.to_string()))
    }

    pub fn extract_yaml_entity_name(entity: &serde_yaml::Value) -> Option<String> {
        entity
            .get("name")
            .or_else(|| entity.get("clientName"))
            .or_else(|| entity.get("client_name"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("label"))
            .or_else(|| entity.get("username"))
            .and_then(|id| id.as_str().map(|v| v.to_string()))
    }

    /// Name of a kubernetes style entity: `metadata.name` when present, the name found in `spec` otherwise
    pub fn extract_yaml_kube_entity_name(entity: &serde_yaml::Value) -> Option<String> {
        entity
            .get("metadata")
            .and_then(|metadata| metadata.get("name"))
            .and_then(|name| name.as_str().map(|v| v.to_string()))
            .or_else(|| entity.get("spec").and_then(Self::extract_yaml_entity_name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yaml(content: &str) -> serde_yaml::Value {
        serde_yaml::from_str(content).unwrap()
    }

    #[test]
    fn test_kube_entity_name_from_metadata() {
        let entity = yaml("kind: Route\nmetadata:\n  name: from-meta\nspec:\n  name: from-spec\n");
        assert_eq!(
            EntityHelper::extract_yaml_kube_entity_name(&entity),
            Some("from-meta".to_string())
        );
    }

    #[test]
    fn test_kube_entity_name_from_spec_name_without_metadata() {
        let entity = yaml("kind: Route\nspec:\n  id: r1\n  name: from-spec\n");
        assert_eq!(
            EntityHelper::extract_yaml_kube_entity_name(&entity),
            Some("from-spec".to_string())
        );
    }

    #[test]
    fn test_kube_entity_name_from_spec_client_name_with_empty_metadata() {
        let entity =
            yaml("kind: ApiKey\nmetadata: {}\nspec:\n  clientId: k1\n  clientName: my-key\n");
        assert_eq!(
            EntityHelper::extract_yaml_kube_entity_name(&entity),
            Some("my-key".to_string())
        );
    }

    #[test]
    fn test_kube_entity_name_from_spec_when_metadata_name_is_not_a_string() {
        let entity = yaml("kind: Route\nmetadata:\n  name: ~\nspec:\n  name: from-spec\n");
        assert_eq!(
            EntityHelper::extract_yaml_kube_entity_name(&entity),
            Some("from-spec".to_string())
        );
    }

    #[test]
    fn test_kube_entity_name_missing_everywhere() {
        let entity = yaml("kind: Route\nmetadata: {}\nspec:\n  id: r1\n");
        assert_eq!(EntityHelper::extract_yaml_kube_entity_name(&entity), None);
    }
}
