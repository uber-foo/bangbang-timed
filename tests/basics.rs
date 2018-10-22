use bangbang_timed::prelude::*;

#[test]
fn new_has_no_side_effect() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let called_on_handler = Arc::new(Mutex::new(false));
    let called_on_inner_handler = Arc::clone(&called_on_handler);
    let mut handle_on = move || {
        *called_on_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    let called_off_handler = Arc::new(Mutex::new(false));
    let called_off_inner_handler = Arc::clone(&called_off_handler);
    let mut handle_off = move || {
        *called_off_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    let now = || 0;

    let _on_off = TimeConstrainedOnOff::new(
        false,
        Some(&mut handle_on),
        Some(&mut handle_off),
        None,
        None,
        &now,
    );
    let called_on_handler = called_on_handler.lock().unwrap();
    let called_off_handler = called_off_handler.lock().unwrap();
    assert_eq!(*called_on_handler, false);
    assert_eq!(*called_off_handler, false);
}

#[test]
fn toggles_on_off_on_off() {
    let now = || 0;

    let mut on_off = TimeConstrainedOnOff::new(true, None, None, None, None, &now);

    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);
}

#[test]
fn toggles_off_on_off_on() {
    let now = || 0;

    let mut on_off = TimeConstrainedOnOff::new(false, None, None, None, None, &now);

    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);
}

#[test]
fn calls_handlers() {
    use std::sync::Arc;
    use std::sync::Mutex;

    let called_on_handler = Arc::new(Mutex::new(false));
    let called_on_inner_handler = Arc::clone(&called_on_handler);
    let mut handle_on = move || {
        *called_on_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    let called_off_handler = Arc::new(Mutex::new(false));
    let called_off_inner_handler = Arc::clone(&called_off_handler);
    let mut handle_off = move || {
        *called_off_inner_handler.lock().unwrap() = true;
        Ok(())
    };

    let now = || 0;

    {
        let mut on_off = TimeConstrainedOnOff::new(
            false,
            Some(&mut handle_on),
            Some(&mut handle_off),
            None,
            None,
            &now,
        );

        assert!(on_off.bang().is_ok());
        let mut called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, true);
        assert_eq!(*called_off_handler, false);
        *called_on_handler = false;
    }

    {
        let mut on_off = TimeConstrainedOnOff::new(
            true,
            Some(&mut handle_on),
            Some(&mut handle_off),
            None,
            None,
            &now,
        );

        assert!(on_off.bang().is_ok());
        let called_on_handler = called_on_handler.lock().unwrap();
        let mut called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, false);
        assert_eq!(*called_off_handler, true);
        *called_off_handler = false;
    }

    {
        let mut on_off = TimeConstrainedOnOff::new(
            false,
            Some(&mut handle_on),
            Some(&mut handle_off),
            None,
            None,
            &now,
        );

        assert!(on_off.bang().is_ok());
        assert!(on_off.bang().is_ok());
        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, true);
        assert_eq!(*called_off_handler, true);
    }
}
