use anymap::{
    any::{Any, IntoBox},
    Map,
};

/// Context for handlers
#[derive(Debug)]
pub struct Context {
    inner: Map<Any + Send + Sync>,
}

impl Default for Context {
    fn default() -> Self {
        Self { inner: Map::new() }
    }
}

impl Context {
    /// Sets a value to context
    pub fn set<T: IntoBox<Any + Send + Sync>>(&mut self, value: T) {
        self.inner.insert(value);
    }

    /// Returns a reference to value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get<T: IntoBox<Any + Send + Sync>>(&self) -> &T {
        self.inner.get().expect("Value not found in context")
    }

    /// Returns a reference to the value stored in context for the type T, if it exists
    pub fn get_opt<T: IntoBox<Any + Send + Sync>>(&self) -> Option<&T> {
        self.inner.get()
    }

    /// Returns a mutable reference to value from context
    ///
    /// # Panics
    ///
    /// Panics if value not found
    pub fn get_mut<T: IntoBox<Any + Send + Sync>>(&mut self) -> &mut T {
        self.inner.get_mut().expect("Value not found in context")
    }

    /// Returns a mutable reference to the value stored in context for the type T, if it exists
    pub fn get_mut_opt<T: IntoBox<Any + Send + Sync>>(&mut self) -> Option<&mut T> {
        self.inner.get_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context() {
        let mut ctx = Context::default();
        ctx.set(1usize);
        assert_eq!(*ctx.get::<usize>(), 1usize);
        assert_eq!(*ctx.get_opt::<usize>().unwrap(), 1usize);
        *ctx.get_mut::<usize>() = 2;
        assert_eq!(*ctx.get::<usize>(), 2usize);
        *ctx.get_mut_opt::<usize>().unwrap() = 3;
        assert_eq!(*ctx.get::<usize>(), 3usize);
        assert!(ctx.get_opt::<u64>().is_none());
    }

    #[test]
    #[should_panic(expected = "Value not found in context")]
    fn context_get_panics() {
        let ctx = Context::default();
        ctx.get::<usize>();
    }

    #[test]
    #[should_panic(expected = "Value not found in context")]
    fn context_get_mut_panics() {
        let mut ctx = Context::default();
        ctx.get_mut::<usize>();
    }
}
