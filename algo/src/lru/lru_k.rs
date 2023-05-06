use rustflake::Snowflake;

use super::Replacer;

/// LRUKReplacer implements the Replacer trait
/// with the LRU-K algorithm for cache eviction. It is used to manage
/// and track the usage of elements, evicting the least recently used
/// element that satisfies the K-th access condition.
///
/// # Parameters
/// - `replacer_size`: The maximum number of elements that the replacer can hold.
/// - `curr_size`: The current number of elements in the replacer.
/// - `frames`: A vector of Frame structs that holds the managed elements.
/// - `current_timestamp`: A counter representing the current timestamp.
/// - `k`: The K-th access condition parameter for the LRU-K algorithm.
///
/// # Trait Implementations
/// The following methods are implemented for the Replacer trait:
///
/// - `evict`: Evicts an element based on the LRU-K algorithm, returning
///   its frame ID if successful or None if the replacer is empty.
/// - `record_access`: Updates an element's access information when it is
///   used, identified by its frame ID.
/// - `set_evictable`: Sets the evictable state of an element, identified
///   by its frame ID, to the given boolean value.
/// - `remove`: Removes an element from the replacer, identified by its
///   frame ID.
/// - `size`: Returns the current number of elements in the replacer.
///
/// # Example
/// ```
/// # use algo::lru::{lru_k::LRUKReplacer, Replacer};
///
/// let mut lru_k_replacer = LRUKReplacer::new(3, 2);
/// lru_k_replacer.record_access(1);
/// lru_k_replacer.record_access(2);
/// lru_k_replacer.record_access(3);
/// lru_k_replacer.record_access(1);
/// lru_k_replacer.set_evictable(1, true);
/// lru_k_replacer.set_evictable(2, true);
/// lru_k_replacer.set_evictable(3, true);
/// let frame_id = lru_k_replacer.evict();
/// assert_eq!(frame_id, Some(2));
/// ```
pub struct LRUKReplacer {
    replacer_size: usize,
    curr_size: usize,
    frames: Vec<Frame>,
    snowflake: Snowflake,
    k: usize,
}

#[derive(PartialEq, Eq)]
pub struct Frame {
    frame_id: i32,
    accesses: Vec<i64>,
    evictable: bool,
}

impl LRUKReplacer {
    pub fn new(replacer_size: usize, k: usize) -> LRUKReplacer {
        LRUKReplacer {
            replacer_size,
            k,
            curr_size: 0,
            frames: Vec::with_capacity(replacer_size),
            snowflake: Snowflake::default(),
        }
    }

    fn find_mut(&mut self, frame_id: i32) -> Option<(usize, &mut Frame)> {
        self.frames
            .iter()
            .position(|frame| frame.frame_id == frame_id)
            .and_then(|index| Some((index, &mut self.frames[index])))
    }

    fn find(&self, frame_id: i32) -> Option<(usize, &Frame)> {
        self.frames
            .iter()
            .position(|frame| frame.frame_id == frame_id)
            .and_then(|index| Some((index, &self.frames[index])))
    }
}

impl Default for LRUKReplacer {
    fn default() -> Self {
        LRUKReplacer::new(10, 2)
    }
}

impl Replacer for LRUKReplacer {
    /// Find the frame with largest backward k-distance and evict that
    /// frame. Only frames that are marked as 'evictable' are candidates for
    /// eviction.
    ///
    /// A frame with less than k historical references is given +inf as its
    /// backward k-distance. If multiple frames have inf backward k-distance, then
    /// evict the frame with the earliest timestamp overall.
    ///
    /// Successful eviction of a frame should decrement the size of replacer and
    /// remove the frame's access history.
    ///
    /// # Return
    ///
    /// * An `Option<i32>` that contains the id of frame that is evicted successfully or `None`.
    fn evict(&mut self) -> Option<i32> {
        let current_timestamp = self.snowflake.generate();
        let (mut distance, mut result) = (0, None);
        for frame in self.frames.iter().rev() {
            if !frame.evictable {
                continue;
            }
            if frame.accesses.len() < self.k {
                distance = std::i64::MAX;
                result = Some(frame.frame_id);
            } else if current_timestamp - frame.accesses[frame.accesses.len() - self.k] > distance {
                distance = current_timestamp - frame.accesses[self.k - 1];
                result = Some(frame.frame_id);
            }
        }
        result.and_then(|frame_id| {
            self.remove(frame_id);
            Some(frame_id)
        })
    }

    /// Record the event that the given frame id is accessed at current
    /// timestamp. Create a new entry for access history if frame id has not been seen before.
    ///
    /// If frame id is invalid (ie. larger than replacer_size_) the process is aborted
    /// # Arguments
    ///
    /// * `frame_id` - id of frame that received a new access.
    fn record_access(&mut self, frame_id: i32) {
        let current_timestamp = self.snowflake.generate();
        let frame: &mut Frame = match self.find_mut(frame_id) {
            Some((_, frame)) => frame,
            None => {
                assert!(
                    self.frames.len() < self.replacer_size,
                    "frame size exceeds the limit"
                );
                self.frames.push(Frame {
                    frame_id,
                    accesses: vec![],
                    evictable: false,
                });
                let index = self.frames.len() - 1;
                &mut self.frames[index]
            }
        };
        frame.accesses.push(current_timestamp)
    }

    /// Toggle whether a frame is evictable or non-evictable. This function
    /// also controls replacer's size. Note that size is equal to number of
    /// evictable entries.
    ///
    /// If a frame was previously evictable and is to be set to non-evictable, then
    /// size should decrement. If a frame was previously non-evictable and is to be
    /// set to evictable, then size should increment.
    ///
    /// If frame id is invalid, throw an exception or abort the process.
    ///
    /// For other scenarios, this function should terminate without modifying
    /// anything.
    ///
    /// # Arguments
    ///
    /// * `frame_id` - id of frame whose 'evictable' status will be modified
    /// * `set_evictable` - whether the given frame is evictable or not
    fn set_evictable(&mut self, frame_id: i32, set_evictable: bool) {
        let (_, mut frame) = self
            .find_mut(frame_id)
            .unwrap_or_else(|| panic!("frame_id {} is invalid", frame_id));
        let evictable = frame.evictable;
        frame.evictable = set_evictable;
        if set_evictable && !evictable {
            self.curr_size += 1;
        } else if !set_evictable && evictable {
            self.curr_size -= 1;
        }
    }

    /// Remove an evictable frame from replacer, along with its access
    /// history. This function should also decrement replacer's size if removal is
    /// successful.
    ///
    /// Note that this is different from evicting a frame, which always remove the
    /// frame with largest backward k-distance. This function removes specified
    /// frame id, no matter what its backward k-distance is.
    ///
    /// If Remove is called on a non-evictable frame, throw an exception or abort
    /// the process.
    ///
    /// If specified frame is not found, directly return from this function.
    ///
    /// # Arguments
    ///
    /// * `frame_id` - id of frame to be removed
    fn remove(&mut self, frame_id: i32) {
        if let Some((index, _)) = self.find(frame_id) {
            assert!(
                self.frames[index].evictable,
                "frame_id {} is non-evictable",
                frame_id
            );
            self.frames.remove(index);
            self.curr_size -= 1;
        }
    }

    /// Return replacer's size, which tracks the number of evictable frames.
    ///
    /// # Return
    ///
    /// * `usize`
    fn size(&self) -> usize {
        self.curr_size
    }
}

#[cfg(test)]
mod tests {
    use crate::lru::{lru_k::LRUKReplacer, Replacer};

    #[test]
    fn sample_test() {
        let mut lru_replacer = LRUKReplacer::new(7, 2);

        // Scenario: add six elements to the replacer. We have [1,2,3,4,5]. Frame 6 is non-evictable.
        lru_replacer.record_access(1);
        lru_replacer.record_access(2);
        lru_replacer.record_access(3);
        lru_replacer.record_access(4);
        lru_replacer.record_access(5);
        lru_replacer.record_access(6);
        lru_replacer.set_evictable(1, true);
        lru_replacer.set_evictable(2, true);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        lru_replacer.set_evictable(5, true);
        lru_replacer.set_evictable(6, false);
        assert_eq!(5, lru_replacer.size());

        // Scenario: Insert access history for frame 1. Now frame 1 has two access histories.
        // All other frames have max backward k-dist. The order of eviction is [2,3,4,5,1].
        lru_replacer.record_access(1);

        // Scenario: Evict three pages from the replacer. Elements with max k-distance should be
        // popped first based on LRU.
        let value = lru_replacer.evict();
        assert_eq!(Some(2), value);
        let value = lru_replacer.evict();
        assert_eq!(Some(3), value);
        let value = lru_replacer.evict();
        assert_eq!(Some(4), value);
        assert_eq!(lru_replacer.size(), 2);

        // Scenario: Now replacer has frames [5,1]. Insert new frames 3, 4, and update access
        // history for 5. We should end with [3,1,5,4]
        lru_replacer.record_access(3);
        lru_replacer.record_access(4);
        lru_replacer.record_access(5);
        lru_replacer.record_access(4);
        lru_replacer.set_evictable(3, true);
        lru_replacer.set_evictable(4, true);
        assert_eq!(4, lru_replacer.size());

        // Scenario: continue looking for victims. We expect 3 to be evicted next.
        let value = lru_replacer.evict();
        assert_eq!(Some(3), value);
        assert_eq!(3, lru_replacer.size());

        // Set 6 to be evictable. 6 Should be evicted next since it has max backward k-dist.
        lru_replacer.set_evictable(6, true);
        assert_eq!(4, lru_replacer.size());
        let value = lru_replacer.evict();
        assert_eq!(Some(6), value);
        assert_eq!(3, lru_replacer.size());

        // Now we have [1,5,4]. Continue looking for victims.
        lru_replacer.set_evictable(1, false);
        assert_eq!(2, lru_replacer.size());
        let value = lru_replacer.evict();
        assert_eq!(Some(5), value);
        assert_eq!(1, lru_replacer.size());

        // Update access history for 1. Now we have [4,1]. Next victim is 4.
        lru_replacer.record_access(1);
        lru_replacer.record_access(1);
        lru_replacer.set_evictable(1, true);
        assert_eq!(2, lru_replacer.size());
        let value = lru_replacer.evict();
        assert_eq!(Some(4), value);

        assert_eq!(1, lru_replacer.size());
        let value = lru_replacer.evict();
        assert_eq!(Some(1), value);
        assert_eq!(0, lru_replacer.size());

        // These operations should not modify size
        assert_eq!(None, lru_replacer.evict());
        assert_eq!(0, lru_replacer.size());
        lru_replacer.remove(1);
        assert_eq!(0, lru_replacer.size());
    }
}
