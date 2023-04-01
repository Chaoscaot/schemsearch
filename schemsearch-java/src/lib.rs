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

use std::path::PathBuf;
use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::jstring;
use schemsearch_files::Schematic;
use schemsearch_lib::{search, SearchBehavior};

#[no_mangle]
#[allow(unused_variables)]
pub extern "system" fn Java_SchemSearch_search<'local>(mut env: JNIEnv<'local>,
    class: JClass<'local>,
    schematic_path: JString<'local>,
    pattern_path: JString<'local>) -> jstring {
    let schematic_path: String = env.get_string(&schematic_path).expect("Couldn't get java string!").into();
    let pattern_path: String = env.get_string(&pattern_path).expect("Couldn't get java string!").into();
    let schematic = Schematic::load(&PathBuf::from(&schematic_path)).unwrap();
    let pattern = Schematic::load(&PathBuf::from(&pattern_path)).unwrap();

    let matches = search(schematic, &pattern, SearchBehavior {
        ignore_block_data: true,
        ignore_block_entities: true,
        ignore_entities: true,
        ignore_air: false,
        air_as_any: false,
        threshold: 0.0,
    });

    let mut result = String::new();
    for m in matches {
        result.push_str(&format!("{}, {}, {}, {};", m.x, m.y, m.z, m.percent));
    }
    result.remove(result.len() - 1);
    let output = env.new_string(result).expect("Couldn't create java string!");
    output.into_raw()
}