pub use codex_protocol::context::ContextualUserFragment;
pub use codex_protocol::context::matches_marked_text;

pub trait FragmentRegistration: Sync {
    fn matches_text(&self, text: &str) -> bool;
}

pub struct FragmentRegistrationProxy<T> {
    _marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> FragmentRegistrationProxy<T> {
    pub const fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> Default for FragmentRegistrationProxy<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ContextualUserFragment> FragmentRegistration for FragmentRegistrationProxy<T> {
    fn matches_text(&self, text: &str) -> bool {
        T::matches_text(text)
    }
}

