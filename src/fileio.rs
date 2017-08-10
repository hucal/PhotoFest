use std;
use std::io;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};

// TODO convert to a config struct
static ROOT: &str = "test/files";

// TODO convert to metadata reader
pub fn read_title(id: &str) -> String {
    // Create a copy of id and return it as a String
    String::from(id)
}

// This function returns a File to the image's thumbnail.
// Right now, it's just the image file itself.
//
// TODO create thumbnail if nonexistant
// TODO use function that returns thumbnail filepath 
// pub fn get_thumbnail_path
pub fn get_thumbnail_file(id: String) -> io::Result<File> {
    get_download_file(id)
}

pub fn get_download_file(id: String) -> io::Result<File> {
    let mut p = PathBuf::from(ROOT);
    p.push(&id);
    p.push(&id);
    File::open(p.as_path())
}


// TODO convert to an iterator that returns a single image id at a time
pub fn list_images() -> Vec<String> {
    let dir = Path::new(ROOT); 
    let mut imgs = Vec::new();

    // TODO log io errors
    // TODO look for thumbnail path
    // TODO mark as bad dir if more than one found, or none found

    // Closure to simulate catch { } syntax like in Rust RFC 0243
    // This closure will append as many valid filenames to the vector until
    // an error is encountered. In this case, it will return early.
    //
    // Any expression appended by a question mark has type Result
    // If the Result is an Err, the closure returns early,
    // otherwise the Result is unwrapped in favor of its "Ok" content.
    let _ = (|| -> io::Result<()> {
        // Iterate through all entries of the directory
        for entry in read_dir(dir)? {
            // Get entry's path
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // Create an error struct with a custom error message
                let err = |m| io::Error::new(io::ErrorKind::Other, m);

                // Convert path's basename to String 
                // Does this by converting path to basename, basename to str,
                // and str to String. Returns early in case of any errors.
                let basename: &std::ffi::OsStr =
                    path.file_name().ok_or(err("No basename!"))?;
                let basename_str: &str =
                    basename.to_str().ok_or(err("Couldn't convert path to str"))?;
                imgs.push(String::from(basename_str))
            }
        };
        // Indicate succesful iteration
        Ok(())
    })();
    imgs
}
