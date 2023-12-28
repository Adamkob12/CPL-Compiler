mod cla;
mod token;

use std::fs::{read_to_string, write, File};
use std::path::Path;
use walkdir::WalkDir;

const INPUT_DIRECTORY: &str = "input";
const OUTPUT_DIRECTORY: &str = "output";

const INPUT_FILE_EXTENSION: &str = "ou";
const OUTPUT_FILE_EXTENSION: &str = "tok";

fn main() {

    let input_dir = Path::new(INPUT_DIRECTORY);
    let output_dir = Path::new(OUTPUT_DIRECTORY);

    // Iterate over all of the files in the input folder
    for input_file in WalkDir::new(input_dir).into_iter() {
        if input_file.is_err() {
            eprintln!("Error: {}", input_file.unwrap_err());
            continue;
        }

        // Extract the file
        let input_file = input_file.unwrap();

        // Skip directories
        if !input_file.metadata().unwrap().is_file() {
            continue;
        }

        let input_file_path = input_file.path();

        let output_file_path = output_dir.join(
            input_file_path
                .strip_prefix(INPUT_DIRECTORY)
                .unwrap()
                .with_extension(OUTPUT_FILE_EXTENSION),
        );

        let file_extension = input_file_path.extension();

        if file_extension.is_none() {
            eprintln!(
                "Error: File {:?} has no extension, expected <.{}> extension",
                input_file.path(),
                INPUT_FILE_EXTENSION
            );
            continue;
        }

        let file_extension = file_extension.unwrap();

        if file_extension == INPUT_FILE_EXTENSION {
            let input_as_string = read_to_string(input_file_path)
                .expect("Couldn't parse the input file into a string");

            File::create(&output_file_path)
                .expect("Couldn't create the output file")
                .set_len(0)
                .expect("Couldn't truncate the output file");
            let lexed = compile(input_as_string);
            write(output_file_path, lexed).expect("Couldn't write to the output file");
        }
    }
}

fn compile(source_code: String) -> String {
    // This function is "fake". It is not a real compiler.
    // This represents "the rest of the compiler" for the Lexer.
    let mut lexer = cla::Lexer::new(source_code.as_str());
    let mut output = String::new();

    while let Some((lexeme, token)) = lexer.get_next_token() {
        output.push_str(lexeme.0);
        output.push_str(format!(": {}\n", token).as_str());
    }

    return output;
}
