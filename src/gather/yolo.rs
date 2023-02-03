use std::fs;

use super::gather::GatherResponseObject;

pub fn yolo_strings_into_files(gather_result: Vec<GatherResponseObject>) {
    for file in gather_result {
        let file_name = file.fileName;

        let file_data = fs::read_to_string(file_name.clone()).unwrap();
        let mut file_data = file_data.split("\n").collect::<Vec<&str>>();
        for component in file.components {
            for string_literal in component.literals {
                let new_line = "{ strings.ReplacedKey }";
                if string_literal.lineNumber.len() > 1 {
                    continue;
                }
                let line_number = (string_literal.lineNumber[0] - 1) as usize;
                if string_literal.text.trim() != file_data[line_number].trim() {
                    continue;
                }
                file_data[line_number] = new_line;
            }
        }
        let file_data = file_data.join("\n");
        fs::write(file_name, file_data).unwrap();
    }
}
