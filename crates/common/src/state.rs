
use std::sync::Arc;
use tokio::sync::RwLock;

/// A generic shared state structure for concurrent access.
#[derive(Clone)]
pub struct SharedState<T> {
    state: Arc<RwLock<T>>,
}

impl<T> SharedState<T>
where
    T: Default + Clone + Send + Sync + 'static,
{
    /// Creates a new shared state with the default value of the generic type.
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(T::default())),
        }
    }

    /// Updates the state with a new value.
    pub async fn set(&self, value: T) {
        let mut state = self.state.write().await;
        *state = value;
    }

    /// Retrieves the current state value.
    pub async fn get(&self) -> T {
        let state = self.state.read().await;
        state.clone()
    }
}

impl SharedState<u32> {
    /// Increments the counter by 1 (specific to `u32` type).
    pub async fn increment(&self) {
        let mut state = self.state.write().await;
        *state += 1;
    }

    /// Decrements the counter by 1 (specific to `u32` type).
    pub async fn decrement(&self) {
        let mut state = self.state.write().await;
        if *state > 0 {
            *state -= 1;
        }
    }

    /// Resets the counter to 0 (specific to `u32` type).
    pub async fn reset(&self) {
        self.set(0).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Barrier;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_shared_state_generic() {
        let shared: SharedState<String> = SharedState::new();
        shared.set("TestValue".to_string()).await;
        assert_eq!(shared.get().await, "TestValue".to_string());
    }

    #[tokio::test]
    async fn test_shared_state_u32_operations() {
        let counter = SharedState::new();

        counter.increment().await;
        assert_eq!(counter.get().await, 1);

        counter.increment().await;
        assert_eq!(counter.get().await, 2);

        counter.decrement().await;
        assert_eq!(counter.get().await, 1);

        counter.reset().await;
        assert_eq!(counter.get().await, 0);
    }

    #[tokio::test]
    async fn test_concurrent_access() {
        let counter = Arc::new(SharedState::new());
        let barrier = Arc::new(Barrier::new(10));

        let mut tasks = Vec::new();
        for _ in 0..10 {
            let c = counter.clone();
            let b = barrier.clone();
            tasks.push(tokio::spawn(async move {
                b.wait().await; // Synchronize the start
                c.increment().await;
            }));
        }

        for task in tasks {
            task.await.unwrap();
        }

        assert_eq!(counter.get().await, 10);
    }
}
