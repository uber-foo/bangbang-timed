use bangbang_timed::prelude::*;
use core::time::Duration;
use std::sync::{Arc, Mutex};

#[test]
fn constrains_min_off() {
    let faux_clock = Arc::new(Mutex::new(0 as u32));
    let faux_clock_inner = Arc::clone(&faux_clock);
    let now = move || faux_clock_inner.lock().unwrap().clone();
    let faux_ten_milliseconds = Duration::from_millis(10);

    let mut on_off =
        TimeConstrainedOnOff::new(true, None, None, None, Some(faux_ten_milliseconds), &now);

    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);
}

#[test]
fn constrains_min_on() {
    let faux_clock = Arc::new(Mutex::new(0 as u32));
    let faux_clock_inner = Arc::clone(&faux_clock);
    let now = move || faux_clock_inner.lock().unwrap().clone();
    let faux_ten_milliseconds = Duration::from_millis(10);

    let mut on_off =
        TimeConstrainedOnOff::new(true, None, None, Some(faux_ten_milliseconds), None, &now);

    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);
}

#[test]
fn constrains_min_on_and_off() {
    let faux_clock = Arc::new(Mutex::new(0 as u32));
    let faux_clock_inner = Arc::clone(&faux_clock);
    let now = move || faux_clock_inner.lock().unwrap().clone();
    let faux_ten_milliseconds = Duration::from_millis(10);

    let mut on_off = TimeConstrainedOnOff::new(
        true,
        None,
        None,
        Some(faux_ten_milliseconds),
        Some(faux_ten_milliseconds),
        &now,
    );

    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 9;

    assert!(on_off.bang().is_err());
    assert_eq!(on_off.is_on(), false);
    assert_eq!(on_off.is_off(), true);

    *faux_clock.lock().unwrap() = now() + 1;

    assert!(on_off.bang().is_ok());
    assert_eq!(on_off.is_on(), true);
    assert_eq!(on_off.is_off(), false);
}

#[test]
fn calls_handlers_after_constraint_met() {
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

    let faux_clock = Arc::new(Mutex::new(0 as u32));
    let faux_clock_inner = Arc::clone(&faux_clock);
    let now = move || faux_clock_inner.lock().unwrap().clone();
    let faux_ten_milliseconds = Duration::from_millis(10);

    {
        let on_off = TimeConstrainedOnOff::new(
            true,
            Some(&mut handle_on),
            Some(&mut handle_off),
            Some(faux_ten_milliseconds),
            Some(faux_ten_milliseconds),
            &now,
        );

        assert_eq!(on_off.is_on(), true);
        assert_eq!(on_off.is_off(), false);

        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, false);
        assert_eq!(*called_off_handler, false);
    }

    {
        let mut on_off = TimeConstrainedOnOff::new(
            true,
            Some(&mut handle_on),
            Some(&mut handle_off),
            Some(faux_ten_milliseconds),
            Some(faux_ten_milliseconds),
            &now,
        );

        assert!(on_off.bang().is_err());
        assert_eq!(on_off.is_on(), true);
        assert_eq!(on_off.is_off(), false);

        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, false);
        assert_eq!(*called_off_handler, false);
    }

    {
        let mut on_off = TimeConstrainedOnOff::new(
            true,
            Some(&mut handle_on),
            Some(&mut handle_off),
            Some(faux_ten_milliseconds),
            Some(faux_ten_milliseconds),
            &now,
        );

        *faux_clock.lock().unwrap() = now() + 9;

        assert!(on_off.bang().is_err());
        assert_eq!(on_off.is_on(), true);
        assert_eq!(on_off.is_off(), false);

        *faux_clock.lock().unwrap() = now() + 1;

        assert!(on_off.bang().is_ok());
        assert_eq!(on_off.is_on(), false);
        assert_eq!(on_off.is_off(), true);

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
            Some(faux_ten_milliseconds),
            Some(faux_ten_milliseconds),
            &now,
        );

        *faux_clock.lock().unwrap() = now() + 9;

        assert!(on_off.bang().is_err());
        assert_eq!(on_off.is_on(), false);
        assert_eq!(on_off.is_off(), true);

        *faux_clock.lock().unwrap() = now() + 1;

        assert!(on_off.bang().is_ok());
        assert_eq!(on_off.is_on(), true);
        assert_eq!(on_off.is_off(), false);

        let called_on_handler = called_on_handler.lock().unwrap();
        let called_off_handler = called_off_handler.lock().unwrap();
        assert_eq!(*called_on_handler, true);
        assert_eq!(*called_off_handler, false);
    }
}
