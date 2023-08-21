use espresso_store::Store;
use starbase::system;

#[system]
pub fn load_store(resources: ResourcesMut) -> SystemResult {
    resources.set(Store::load()?);
}
