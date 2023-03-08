use std::path::Path;
#[cfg(feature = "sql")]
use futures::executor::block_on;
use schemsearch_files::Schematic;
#[cfg(feature = "sql")]
use schemsearch_sql::{load_schemdata, SchematicNode};

pub enum SchematicSupplierType<'local> {
    PATH(Box<PathSchematicSupplier<'local>>),
    #[cfg(feature = "sql")]
    SQL(SqlSchematicSupplier),
}

pub struct PathSchematicSupplier<'local> {
    pub path: &'local Path,
}

impl PathSchematicSupplier<'_> {
    pub fn get_name(&self) -> String {
        self.path.file_name().unwrap().to_str().unwrap().to_string()
    }
}

#[cfg(feature = "sql")]
pub struct SqlSchematicSupplier {
    pub node: SchematicNode,
}

#[cfg(feature = "sql")]
impl SqlSchematicSupplier {
    pub fn get_schematic(&self) -> Result<Schematic, String> {
        let schemdata = block_on(load_schemdata(self.node.id));
        Schematic::load_data(schemdata.as_slice())
    }

    pub fn get_name(&self) -> String {
        format!("{} ({})", self.node.name, self.node.id)
    }
}
