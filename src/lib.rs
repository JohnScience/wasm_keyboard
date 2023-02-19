pub use uievents_code;
pub use wasm_keyboard_macros as macros;

use uievents_code::KeyboardEventCode;

#[cfg(any(doc, feature = "keypress"))]
macro_rules! key_handler_ty {
    ($key_id:ident, $state:ident, $f1:ident, $f2:ident, $f3:ident) => {
        KeyHandler<$key_id, $state, $f1, $f2, $f3>
    };
}
#[cfg(not(any(doc, feature = "keypress")))]
macro_rules! key_handler_ty {
    ($key_id:ident, $state:ident, $f1:ident, $f2:ident, $f3:ident) => {
        KeyHandler<$key_id, $state, $f1, $f2>
    };
}

/// A trait for key event handlers, i.e. handlers of [`keydown`], [`keyup`], or
/// the **DEPRECATED** [`keypress`] events.
///
/// Check out the [new_primitive_key_handler!][crate::macros::new_primitive_key_handler!] and
/// [new_simplified_key_handler!][crate::macros::new_simplified_key_handler!] to see
/// how you can conveniently create an instance of type implementing this trait.
///
/// Also check out the [crate::implementors::KeyHandler] struct.
///
/// [`keydown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
/// [`keyup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
/// [`keypress`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
pub trait KeyHandler {
    /// The enum variant corresponding to the [`KeyboardEvent.code`].
    ///
    /// [`KeyboardEvent.code`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    const CODE: KeyboardEventCode;
    /// Handles the [`keydown`] event. Depending on the kind of key handler,
    /// e.g. "primitive" or "simplified", this method may or may not be called
    /// for every [`keydown`] event.
    ///
    /// "Simplified" key handlers are only called
    /// once for each key press, while "primitive" key handlers can fire multiple
    /// times for a single key press (if it's a long-press).
    ///
    /// [`keydown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
    fn handle_keydown(&self);
    /// Handles the [`keyup`] event.
    ///
    /// [`keyup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
    fn handle_keyup(&self);
    /// Handles the **DEPRECATED** [`keypress`] event.
    ///
    /// [`keypress`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
    #[cfg(any(doc, feature = "keypress"))]
    fn handle_keypress(&self);
}

/// Module containing the structures that are default implementors of the traits
/// defined in this crate.
pub mod implementors {
    use uievents_code::KeyboardEventCode;

    /// A default implementor of the [`KeyHandler`] trait.
    ///
    /// # Generic parameters
    ///
    /// - `KEY_ID`: The constant containing `u8` representation of the [`KeyboardEventCode`] enum
    /// variant, which corresponds to the [`KeyboardEvent.code`] of the key that this handler is
    /// for.
    /// - `State`: The type of the state that is maintained by the handler and passed to handler
    /// by reference. Frequently, this type is `()` or some [`Cell<T>`][`std::cell::Cell`].
    /// - `F1`, `F2`, `F3`: The types of the functions or closures that are called when the
    /// corresponding handling method is called for [`keydown`], [`keyup`], and [`keypress`],
    /// respectively.
    ///
    /// Check out the [new_primitive_key_handler!][crate::macros::new_primitive_key_handler!] and
    /// [new_simplified_key_handler!][crate::macros::new_simplified_key_handler!] to see
    /// how you can conveniently create an instance of this type.
    ///
    /// [`KeyboardEvent.code`]: https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/code
    /// [`keydown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
    /// [`keyup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
    /// [`keypress`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
    pub struct KeyHandler<
        const KEY_ID: u8,
        State,
        F1: Fn(&State),
        F2: Fn(&State),
        #[cfg(any(doc, feature = "keypress"))] F3: Fn(&State),
    > {
        handle_keydown_impl: F1,
        handle_keyup_impl: F2,
        #[cfg(any(doc, feature = "keypress"))]
        handle_keypress_impl: F3,
        state: State,
    }

    impl<
            const KEY_ID: u8,
            State,
            F1,
            F2,
            #[cfg(any(doc, feature = "keypress"))] F3: Fn(&State),
        > key_handler_ty!(KEY_ID, State, F1, F2, F3)
    where
        F1: Fn(&State),
        F2: Fn(&State),
    {
        /// Creates a new instance of key handler.
        ///
        /// Frequently, the `state` parameter is `()` or some [`Cell<T>`][`std::cell::Cell`]
        /// and the `handle_*_impl` parameters are closures.
        pub fn new(
            state: State,
            handle_keydown_impl: F1,
            handle_keyup_impl: F2,
            #[cfg(any(doc, feature = "keypress"))] handle_keypress_impl: F3,
        ) -> Self {
            Self {
                state,
                handle_keydown_impl,
                handle_keyup_impl,
                #[cfg(any(doc, feature = "keypress"))]
                handle_keypress_impl,
            }
        }
    }

    impl<
            const KEY_ID: u8,
            State,
            F1: Fn(&State),
            F2: Fn(&State),
            #[cfg(any(doc, feature = "keypress"))] F3: Fn(&State),
        > super::KeyHandler for key_handler_ty!(KEY_ID, State, F1, F2, F3)
    {
        // At the time of writing, Option::unwrap() is unstable in const context
        const CODE: KeyboardEventCode = match KeyboardEventCode::from_repr(KEY_ID) {
            Some(code) => code,
            None => panic!("Invalid key code."),
        };
        fn handle_keydown(&self) {
            let &Self {
                ref handle_keydown_impl,
                ref state,
                ..
            } = self;
            (handle_keydown_impl)(state)
        }
        fn handle_keyup(&self) {
            let &Self {
                ref handle_keyup_impl,
                ref state,
                ..
            } = self;
            (handle_keyup_impl)(state)
        }
        #[cfg(any(doc, feature = "keypress"))]
        fn handle_keypress(&self) {
            let &Self {
                ref handle_keypress_impl,
                ref state,
                ..
            } = self;
            (handle_keypress_impl)(state)
        }
    }
}

/// A trait that is implemented by the types that can handle
/// [`keydown`], [`keyup`], and [`keypress`] events fired
/// by any key on a keyboard.
///
/// [`keydown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
/// [`keyup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
/// [`keypress`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
pub trait KeyboardHandler {
    /// Handles the [`keydown`] event.
    ///
    /// [`keydown`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
    fn handle_keydown(&self, event: &::web_sys::KeyboardEvent);
    /// Handles the [`keyup`] event.
    ///
    /// [`keyup`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
    fn handle_keyup(&self, event: &::web_sys::KeyboardEvent);
    /// Handles the **DEPRECATED** [`keypress`] event.
    ///
    /// [`keypress`]: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
    #[cfg(feature = "keypress")]
    fn handle_keypress(&self, event: &::web_sys::KeyboardEvent);
}
