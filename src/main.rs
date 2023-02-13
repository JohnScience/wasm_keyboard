pub use uievents_code;

use uievents_code::KeyboardEventCode;

pub trait KeyHandler {
    const CODE: KeyboardEventCode;
    fn handle_keydown(&mut self);
    fn handle_keypress(&mut self);
    fn handle_keyup(&mut self);
}

pub mod default_implementor {
    use uievents_code::KeyboardEventCode;

    pub struct KeyHandler<const KEY_ID: u8, State, F1, F2, F3>
    where
        F1: Fn(&mut State) -> (),
        F2: Fn(&mut State) -> (),
        F3: Fn(&mut State) -> ()
    {
        handle_keydown_impl: F1,
        handle_keypress_impl: F2,
        handle_keyup_impl: F3,
        state: State,
    }

    impl<const KEY_ID: u8, State, F1, F2, F3> KeyHandler<KEY_ID, State, F1, F2, F3>
    where
        F1: Fn(&mut State) -> (),
        F2: Fn(&mut State) -> (),
        F3: Fn(&mut State) -> ()
    {
        pub fn new(state: State, handle_keydown_impl: F1, handle_keypress_impl: F2, handle_keyup_impl: F3) -> Self {
            Self {
                state,
                handle_keydown_impl,
                handle_keypress_impl,
                handle_keyup_impl,
            }
        }
    }

    impl<const KEY_ID: u8, State, F1, F2, F3> super::KeyHandler for KeyHandler<KEY_ID, State, F1, F2, F3>
    where
        F1: Fn(&mut State) -> (),
        F2: Fn(&mut State) -> (),
        F3: Fn(&mut State) -> ()
    {
        // At the time of writing, Option::unwrap() is unstable in const context
        const CODE: KeyboardEventCode = match KeyboardEventCode::from_repr(KEY_ID) {
            Some(code) => code,
            None => panic!("Invalid key code."),
        };
        fn handle_keydown(&mut self) {
            let &mut Self {
                ref handle_keydown_impl,
                ref mut state,
                ..
            } = self;
            (handle_keydown_impl)(state)
        }
        fn handle_keypress(&mut self) {
            let &mut Self {
                ref handle_keypress_impl,
                ref mut state,
                ..
            } = self;
            (handle_keypress_impl)(state)
        }
        fn handle_keyup(&mut self) {
            let &mut Self {
                ref handle_keyup_impl,
                ref mut state,
                ..
            } = self;
            (handle_keyup_impl)(state)
        }
    }
}

fn main() {

}
