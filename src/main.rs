use log::{debug, trace};
use nmuidi::nmuidi::Cleaner;
use std::{env, time::Instant};
use std::io::{stdin, stdout, Write};
mod context_menu;
fn main() {
    context_menu::add_context_menu();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        print!("Do you want to delete this folder? [y/N]: ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input.trim().eq_ignore_ascii_case("y") {
            pretty_env_logger::init();
            context_menu::main();
            let mut directory_timings = Vec::new();
            let start_time = Instant::now();
            for dir in args.iter().skip(1) {
                println!("Cleaning {}", dir);
                let start = Instant::now();
                Cleaner::new(dir).clean();
                directory_timings.push((dir.clone(), start.elapsed()));
            }

            let elapsed_time = start_time.elapsed();
            debug!("Total time: {:.2?}", elapsed_time);
            debug!("Directory timings:");
            for (dir, time_spent) in directory_timings {
                debug!("  dir {} took {:.2?}", dir, time_spent);
            }
            trace!("Done.");
        }

    } else {
        println!("No directory specified!");
    }
    trace!("Done.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use jwalk::WalkDir;
    use std::fs;

    #[test]
    fn test_nested() {
        fs::create_dir_all("tmp/nested/dir1").unwrap();
        fs::write("tmp/nested/dir1/file1.txt", "File 1 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2").unwrap();
        fs::write("tmp/nested/dir1/dir2/file2.txt", "File 2 content").unwrap();

        fs::create_dir_all("tmp/nested/dir1/dir2/dir3").unwrap();
        fs::write("tmp/nested/dir1/dir2/dir3/file3.txt", "File 3 content").unwrap();

        Cleaner::new("tmp/nested").clean();

        let num_files = WalkDir::new("tmp/nested")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }

    #[test]
    fn test_dirs() {
        fs::create_dir_all("tmp/dirs/dir1").unwrap();
        fs::create_dir_all("tmp/dirs/dir1a").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2a").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2/dir3").unwrap();
        fs::create_dir_all("tmp/dirs/dir1/dir2/dir3a").unwrap();

        Cleaner::new("tmp/dirs").clean();

        let num_files = WalkDir::new("tmp/dirs")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }

    #[test]
    fn test_files() {
        fs::create_dir_all("tmp/files").unwrap();
        fs::write("tmp/files/file1.txt", "File 1 content").unwrap();
        fs::write("tmp/files/file2.txt", "File 2 content").unwrap();
        fs::write("tmp/files/file3.txt", "File 3 content").unwrap();
        fs::write("tmp/files/file4.txt", "File 4 content").unwrap();
        fs::write("tmp/files/file5.txt", "File 5 content").unwrap();
        fs::write("tmp/files/file6.txt", "File 6 content").unwrap();

        Cleaner::new("tmp/files").clean();

        let num_files = WalkDir::new("tmp/files")
            .skip_hidden(false)
            .into_iter()
            .collect::<Vec<_>>()
            .len();
        assert_eq!(num_files, 1);
    }
}
