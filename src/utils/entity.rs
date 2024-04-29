pub struct EntityHelper {}

impl EntityHelper {

    pub fn extract_json_entity_id(entity: &serde_json::Value) -> Option<String> {
        entity.get("id")
            .or_else(|| entity.get("client_id"))
            .or_else(|| entity.get("clientId"))
            .or_else(|| entity.get("username"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("serviceId"))
            .or_else(|| entity.get("randomId"))
            .and_then(|id| {
                id.as_str().map(|v| v.to_string())
            })
    }

    pub fn extract_yaml_entity_id(entity: &serde_yaml::Value) -> Option<String> {
        entity.get("id")
            .or_else(|| entity.get("client_id"))
            .or_else(|| entity.get("clientId"))
            .or_else(|| entity.get("username"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("serviceId"))
            .or_else(|| entity.get("randomId"))
            .and_then(|id| {
                id.as_str().map(|v| v.to_string())
            })
    }

    pub fn extract_json_entity_name(entity: &serde_json::Value) -> Option<String> {
        entity.get("name")
            .or_else(|| entity.get("clientName"))
            .or_else(|| entity.get("client_name"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("label"))
            .or_else(|| entity.get("username"))
            .and_then(|id| {
                id.as_str().map(|v| v.to_string())
            })
    }

    pub fn extract_yaml_entity_name(entity: &serde_yaml::Value) -> Option<String> {
        entity.get("name")
            .or_else(|| entity.get("clientName"))
            .or_else(|| entity.get("client_name"))
            .or_else(|| entity.get("otoroshiId"))
            .or_else(|| entity.get("label"))
            .or_else(|| entity.get("username"))
            .and_then(|id| {
                id.as_str().map(|v| v.to_string())
            })
    }
}