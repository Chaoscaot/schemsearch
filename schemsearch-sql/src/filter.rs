#[derive(Default, Debug, Clone)]
pub struct SchematicFilter {
    pub user_id: Option<Vec<u32>>,
    pub name: Option<Vec<String>>,
}

impl SchematicFilter {
    pub fn new() -> SchematicFilter {
        SchematicFilter {
            user_id: None,
            name: None,
        }
    }

    pub fn user_id(mut self, user_id: Vec<&u32>) -> SchematicFilter {
        self.user_id = Some(user_id.into_iter().map(|id| *id).collect());
        self
    }

    pub fn name(mut self, name: Vec<&String>) -> SchematicFilter {
        self.name = Some(name.into_iter().map(|name| name.to_string()).collect());
        self
    }

    pub fn build(self) -> String {
        if self.user_id.is_none() && self.name.is_none() {
            return String::new();
        }
        let mut query = Vec::new();
        if let Some(user_id) = self.user_id {
            query.push(user_id.into_iter().map(|id| format!("SN.NodeOwner = {}", id)).collect::<Vec<String>>().join(" OR "));
        }
        if let Some(name) = self.name {
            query.push(name.into_iter().map(|name| format!("SN.NodeName LIKE '%{}%'", name)).collect::<Vec<String>>().join(" OR "));
        }
        format!("AND ({})", query.join(") AND ("))
    }
}