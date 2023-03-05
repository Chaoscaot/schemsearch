#![no_mangle]

use jni::JNIEnv;

use jni::objects::{JClass, JString};

use jni::sys::jstring;
use schemsearch_lib::{search, SearchBehavior};

pub extern "system" fn Java_SchemSearch_search<'local>(mut env: JNIEnv<'local>,
    class: JClass<'local>,
    schematic_path: JString<'local>,
    pattern_path: JString<'local>) -> jstring {
    let schematic_path: String = env.get_string(&schematic_path).expect("Couldn't get java string!").into();
    let pattern_path: String = env.get_string(&pattern_path).expect("Couldn't get java string!").into();
    let file = std::fs::File::open(schematic_path).expect("Failed to open file");
    let schematic = &std::io::Read::bytes(file).map(|b| b.unwrap()).collect();
    let file = std::fs::File::open(pattern_path).expect("Failed to open file");
    let pattern = &std::io::Read::bytes(file).map(|b| b.unwrap()).collect();

    let matches = search(schematic, pattern, SearchBehavior {
        ignore_block_data: true,
        ignore_block_entities: true,
        ignore_entities: true,
    });

    let mut result = String::new();
    for (x, y, z) in matches {
        result.push_str(&format!("{}, {}, {};", x, y, z));
    }
    result.remove(result.len() - 1);
    let output = env.new_string(result).expect("Couldn't create java string!");
    output.into_raw()
}