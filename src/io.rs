use actix_web::web::Bytes;
use linked_hash_map::LinkedHashMap;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::cell::RefCell;
use std::fs;

pub type PasteStore = RwLock<LinkedHashMap<String, Bytes>>;

static BUFFER_SIZE: Lazy<usize> = Lazy::new(|| argh::from_env::<crate::BinArgs>().buffer_size);

const DATA_DIR: &str = "data/";

/// Ensures `ENTRIES` is less than the size of `BIN_BUFFER_SIZE`. If it isn't then
/// `ENTRIES.len() - BIN_BUFFER_SIZE` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
fn purge_old(entries: &mut LinkedHashMap<String, Bytes>) {
    if entries.len() > *BUFFER_SIZE {
        let to_remove = entries.len() - *BUFFER_SIZE;

        for _ in 0..to_remove {
            let result = entries.pop_front();
            match result {
                Some(r) => {
                    fs::remove_file(r.0).expect("Removing file failed");
                }
                None => {}
            }
        }
    }
}

/// Generates a 'pronounceable' random ID using gpw
pub fn generate_id() -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));

    KEYGEN.with(|k| k.borrow_mut().next()).unwrap_or_else(|| {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect()
    })
}

/// Stores a paste under the given id
pub fn store_paste(entries: &PasteStore, id: String, content: Bytes) {
    let mut entries = entries.write();

    purge_old(&mut entries);

    let mut fid = id.to_owned();
    fid.insert_str(0, DATA_DIR);

    fs::write(fid, &content).expect("Writing file failed");
    entries.insert(id, content);
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub fn get_paste(entries: &PasteStore, id: &str) -> Option<Bytes> {
    // need to box the guard until owning_ref understands Pin is a stable address
    entries.read().get(id).map(Bytes::clone)
}

// Read pastes from data/ into map
pub fn load_pastes(entries: &PasteStore) {
    let paths = fs::read_dir(DATA_DIR).unwrap();
    for path in paths {
        let p = path.unwrap().path();
        let fid = p.to_str().expect("Failed to ID paste");
        let bytes = fs::read(fid).expect("Failed to read paste");

        let name = p.file_name().expect("Failed to get file name");
        let id = String::from(
            name.to_str()
                .expect("Failed to convert file name to string"),
        );
        let data = Bytes::from(bytes);

        let mut entries = entries.write();
        entries.insert(id, data);
    }
}
