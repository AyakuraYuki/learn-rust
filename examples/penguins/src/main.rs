fn main() {
    let penguin_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            // 跳过第一行表头，或者空行
            continue;
        }

        let fields: Vec<_> = record.split(',')
            .map(|field| field.trim())
            .collect();
        #[cfg(debug_assertions)]
        {
            eprintln!("debug: {:?} -> {:?}", record, fields)
        }

        let common_name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}: {}cm", common_name, length)
        }
    }
}
