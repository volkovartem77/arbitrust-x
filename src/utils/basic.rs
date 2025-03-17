use chrono::{Local, DateTime};

pub fn time_now_mcs() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn time_now_ms() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}


pub fn make_arb_id(long_price: f64, short_price: f64) -> String {
    let data = format!("{}{}", long_price, short_price);
    format!("{:x}", md5::compute(data.as_bytes()))
}


pub fn round_up(number: f64, precision: u32) -> f64 {
    let multiplier = 10f64.powi(precision as i32);
    (number * multiplier).ceil() / multiplier
}


pub fn round_down(number: f64, precision: u32) -> f64 {
    let multiplier = 10f64.powi(precision as i32);
    (number * multiplier).floor() / multiplier
}

pub fn round(number: f64, precision: u32) -> f64 {
    let multiplier = 10_f64.powi(precision as i32);
    (number * multiplier).round() / multiplier
}


pub fn datetime_now_mcs() -> String {
    let current_time: DateTime<Local> = Local::now();
    current_time.format("%Y-%m-%d %H:%M:%S%.6f").to_string()
}

// fn connect_to_memory_cache() -> redis::Connection {
//     let client = redis::Client::open(REDIS_SERVER).unwrap();
//     client.get_connection().unwrap()
// }
//
//
// fn set_mem_cache(var: &str, prefix: &str, data: &Value, expiration: Option<usize>) {
//     let mut con = connect_to_memory_cache();
//     let key = format!("{}:{}", var, prefix);
//     let json_data = serde_json::to_string(data).unwrap();
//
//     if let Some(exp) = expiration {
//         let _: () = con.set_ex(&key, json_data, exp).unwrap();
//     } else {
//         let _: () = con.set(&key, json_data).unwrap();
//     }
// }
//
// fn get_mem_cache(var: &str, prefix: &str) -> Option<Value> {
//     let mut con = connect_to_memory_cache();
//     let key = format!("{}:{}", var, prefix);
//     let json_data: String = con.get(&key).unwrap_or_default();
//
//     serde_json::from_str(&json_data).ok()
// }
//
// fn delete_mem_cache(var: &str, prefix: &str) {
//     let mut con = connect_to_memory_cache();
//     let key = format!("{}:{}", var, prefix);
//     let _: () = con.del(&key).unwrap();
// }
//
// pub fn mem_set_history_arb(symbol: &str, value: &[HashMap<String, ValueVariant>]) {
//     let prefix = symbol.to_lowercase();
//     let serialized_history: Vec<Value> = value.iter().map(|variant_map| {
//         let mut json_map = JsonMap::new();
//         for (key, variant) in variant_map {
//             let json_value = match variant {
//                 ValueVariant::StrVal(s) => Value::String(s.clone()),
//                 ValueVariant::FloatVal(f) => Value::Number(serde_json::Number::from_f64(*f).expect("Float value not f64!")),
//                 ValueVariant::IntVal(i) => Value::Number((*i).into()),
//                 ValueVariant::U128Val(u) => json!(*u),
//             };
//             json_map.insert(key.clone(), json_value);
//         }
//         Value::Object(json_map)
//     }).collect();
//
//     let data = Value::Array(serialized_history);
//     set_mem_cache(MEM_HISTORY_ARB, &prefix, &data, None);
// }
//
// pub fn mem_get_history_arb(symbol: &str) -> Vec<HashMap<String, ValueVariant>> {
//     let prefix = symbol.to_lowercase();
//     if let Some(Value::Array(history)) = get_mem_cache(MEM_HISTORY_ARB, &prefix) {
//         history.into_iter().map(|value| {
//             let obj = value.as_object().expect("Expected a JSON object!");
//             obj.iter().map(|(key, val)| {
//                 let value_variant = match val {
//                     Value::String(s) => ValueVariant::StrVal(s.clone()),
//                     Value::Number(num) => {
//                         if let Some(f) = num.as_f64() {
//                             ValueVariant::FloatVal(f)
//                         } else if let Some(i) = num.as_i64() {
//                             ValueVariant::IntVal(i)
//                         } else {
//                             panic!("Invalid number format")
//                         }
//                     },
//                     _ => panic!("Unexpected value type")
//                 };
//                 (key.clone(), value_variant)
//             }).collect()
//         }).collect()
//     } else {
//         Vec::new()
//     }
// }
//
// pub fn mem_add_history_arb(symbol: &str, history: &[HashMap<String, ValueVariant>]) {
//     let mut mem_history_arb = mem_get_history_arb(symbol);
//     mem_history_arb.extend_from_slice(history);
//
//     while mem_history_arb.len() > HISTORY_ARB_LIMIT {
//         mem_history_arb.remove(0);
//     }
//
//     mem_set_history_arb(symbol, &mem_history_arb);
// }
//
// fn mem_set_log(module: &str, lines: &[String]) {
//     let prefix = module.to_lowercase();
//     let data = Value::Array(lines.iter().map(|line| Value::String(line.clone())).collect());
//     set_mem_cache(MEM_LOG, &prefix, &data, None);
// }
//
// fn mem_get_log(module: &str) -> Vec<String> {
//     let prefix = module.to_lowercase();
//     if let Some(Value::Array(lines)) = get_mem_cache(MEM_LOG, &prefix) {
//         lines.iter().map(|line| line.as_str().unwrap().to_string()).collect()
//     } else {
//         Vec::new()
//     }
// }
//
// fn mem_add_to_log(module: &str, line: &str) {
//     let mut log_list = mem_get_log(module);
//     if log_list.len() >= MEM_LOG_LENGTH {
//         log_list.remove(0);
//     }
//     log_list.push(line.to_string());
//     mem_set_log(module, &log_list);
// }
//
// fn log_to_file(msg: &str, module: &str) {
//     let log_file_name = format!("{}.log", module);
//     let current_file_path = Path::new(file!());
//
//     let log_file_path = if let Some(dir) = current_file_path.parent() {
//         dir.join(log_file_name)
//     } else {
//         PathBuf::from(log_file_name)
//     };
//
//     if log_file_path.exists() && log_file_path.metadata().unwrap().len() > LOG_SIZE_MB * 1024 * 1024 {
//         // Handle file size exceeding LOG_SIZE_MB
//     }
//
//     let mut file = fs::OpenOptions::new()
//         .append(true)
//         .create(true)
//         .open(log_file_path)
//         .unwrap();
//     file.write_all(msg.as_bytes()).unwrap();
// }
//
// pub fn log(text: &str, module: &str, mark: &str, to_mem: bool) {
//     let msg = format!("{} {} {}\n", datetime_now_mcs(), mark, text);
//
//     // log_to_file(&msg, module);
//     println!("{} {} {}", datetime_now_mcs(), mark, text);
//
//     if to_mem {
//         mem_add_to_log(module, &msg);
//     }
// }