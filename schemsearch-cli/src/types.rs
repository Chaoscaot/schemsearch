/*
 * Copyright (C) 2023  Chaoscaot
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

#[cfg(feature = "sql")]
use std::io::Cursor;
use std::path::PathBuf;
#[cfg(feature = "sql")]
use futures::executor::block_on;
#[cfg(feature = "sql")]
use schemsearch_files::SpongeSchematic;
#[cfg(feature = "sql")]
use schemsearch_sql::{load_schemdata, SchematicNode};

pub enum SchematicSupplierType {
    PATH(PathSchematicSupplier),
    #[cfg(feature = "sql")]
    SQL(SqlSchematicSupplier),
}

pub trait SchematicSupplier {
    fn get_name(&self) -> String;
}

pub struct PathSchematicSupplier {
    pub path: PathBuf,
}

impl SchematicSupplier for PathSchematicSupplier {
    fn get_name(&self) -> String {
        self.path.file_stem().unwrap().to_str().unwrap().to_string()
    }
}

#[cfg(feature = "sql")]
pub struct SqlSchematicSupplier {
    pub node: SchematicNode,
}

#[cfg(feature = "sql")]
impl SqlSchematicSupplier {
    pub fn get_schematic(&self) -> Result<SpongeSchematic, String> {
        let mut schemdata = block_on(load_schemdata(self.node.id));
        SpongeSchematic::load_data(&mut Cursor::new(schemdata.as_mut_slice()))
    }
}

#[cfg(feature = "sql")]
impl SchematicSupplier for SqlSchematicSupplier {
    fn get_name(&self) -> String {
        format!("{} ({})", self.node.name, self.node.id)
    }
}


