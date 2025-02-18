use crate::utils::storages::Storage;

#[test]
fn storage () {
    Storage::create_storage_if_not_exists();

    assert!(true);
}
#[test]
fn create_folder () {
    Storage::create_folder("New Folder");

    assert!(true);
}