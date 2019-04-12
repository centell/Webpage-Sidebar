use stdweb::{PromiseFuture};

pub mod browser {
    pub mod storage {
        pub mod sync {
            pub fn get<T>(key: &str) -> PromiseFuture<T> {
                (js! {
                    return browser.storage.sync.get(key);
                }).try_into().unwrap()
            }
        }
    }
}