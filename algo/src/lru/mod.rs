pub mod lru_k;

pub trait Replacer {
    fn evict(&mut self) -> Option<i32>;

    fn record_access(&mut self, frame_id: i32);

    fn set_evictable(&mut self, frame_id: i32, set_evictable: bool);

    fn remove(&mut self, frame_id: i32);

    fn size(&self) -> usize;
}
