use std::path::Path;
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
    let schematic = Schematic::load(Path::new(&schematic_path)).unwrap();
    let pattern = Schematic::load(Path::new(&pattern_path)).unwrap();

    let matches = search(&schematic, &pattern, SearchBehavior {
        ignore_block_data: true,
        ignore_block_entities: true,
        ignore_entities: true,
        ignore_air: false,
        air_as_any: false,
        threshold: 0.0,
    });

    let mut result = String::new();
    for (x, y, z, p) in matches {
        result.push_str(&format!("{}, {}, {}, {};", x, y, z, p));
    }
    result.remove(result.len() - 1);
    let output = env.new_string(result).expect("Couldn't create java string!");
    output.into_raw()
}