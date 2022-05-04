pub mod process_transcripts {
    use std::fs;
    use std::path::PathBuf;
    use regex::Regex;
    use ezio::prelude::*;

    pub fn save_processed_scripts() {
        let transcript_dir_pathbuf = PathBuf::from("en-US");
        fs::create_dir_all("out/en-US").unwrap();

        let course_dirs_entry = fs::read_dir(transcript_dir_pathbuf).unwrap();
        for dir in course_dirs_entry {
            let dir_path = dir.unwrap().path();
            // make directory per course
            let course_dir_name = format!("out/{}", dir_path.to_str().unwrap());
            fs::create_dir(&course_dir_name).unwrap();

            let course_dir_entry = fs::read_dir(PathBuf::from(dir_path)).unwrap();
            for file_entry in course_dir_entry {
                let file_path = file_entry.unwrap().path();
                let out_file_path = format!("out/{}", &file_path.to_str().unwrap());
                let re_file_ext = Regex::new(r"^*\.txt$").unwrap();
                if !re_file_ext.is_match(&out_file_path) {
                    continue;
                }
                let mut writer = file::writer(out_file_path);
                // Read lines in a file
                for line in file::reader(file_path) {
                    // remove the lines (e.g. 112, [11:23:58], 11:23:58.132 --> 13:45:5.891) 
                    let re = Regex::new(r"^\d+|^\[\d{2}:\d{2}:\d{2}\]|^$\d{2}:\d{2}:\d{2}\]\.\d{3} --> \d{2}:\d{2}:\d{2}\]\.\d{3}|^\n$").unwrap();
                    if !re.is_match(&line) {
                        writer.write(&line);
                        writer.write("\n");
                    }
                }
            }
        }
    }
}