// Copyright (C) 2023-2024 Rowan Hart
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::disallowed_names)]
#![deny(missing_docs)]

use getters2::Getters;

#[derive(Getters)]
#[getters(deref, clone, mutable)]
struct FooNamed {
    bar: i32,
}

#[test]
fn test_struct_named() {
    let mut foo = FooNamed { bar: 42 };
    assert_eq!(foo.bar_ref(), &42);
    assert_eq!(foo.bar_deref(), 42);
    assert_eq!(foo.bar_clone(), 42);
    *foo.bar_mut() = 43;
    assert_eq!(foo.bar_ref(), &43);
    assert_eq!(foo.bar_deref(), 43);
    assert_eq!(foo.bar_clone(), 43);
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
struct FooNewtype(i32);

#[test]
fn test_struct_newtype() {
    let mut foo = FooNewtype(42);
    assert_eq!(foo.first_ref(), &42);
    assert_eq!(foo.first_deref(), 42);
    assert_eq!(foo.first_clone(), 42);
    *foo.first_mut() = 43;
    assert_eq!(foo.first_ref(), &43);
    assert_eq!(foo.first_deref(), 43);
    assert_eq!(foo.first_clone(), 43);
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
struct FooTuple(i32, i32, i32, i32, i32, i32);

#[test]
fn test_struct_tuple() {
    let mut foo = FooTuple(42, 43, 44, 45, 46, 47);
    assert_eq!(foo.first_ref(), &42);
    assert_eq!(foo.first_deref(), 42);
    assert_eq!(foo.first_clone(), 42);
    assert_eq!(foo.second_ref(), &43);
    assert_eq!(foo.second_deref(), 43);
    assert_eq!(foo.second_clone(), 43);
    assert_eq!(foo.third_ref(), &44);
    assert_eq!(foo.third_deref(), 44);
    assert_eq!(foo.third_clone(), 44);
    assert_eq!(foo.fourth_ref(), &45);
    assert_eq!(foo.fourth_deref(), 45);
    assert_eq!(foo.fourth_clone(), 45);
    assert_eq!(foo.fifth_ref(), &46);
    assert_eq!(foo.fifth_deref(), 46);
    assert_eq!(foo.fifth_clone(), 46);
    assert_eq!(foo.last_ref(), &47);
    assert_eq!(foo.last_deref(), 47);
    assert_eq!(foo.last_clone(), 47);

    *foo.first_mut() = 43;
    *foo.second_mut() = 44;
    *foo.third_mut() = 45;
    *foo.fourth_mut() = 46;
    *foo.fifth_mut() = 47;
    *foo.last_mut() = 48;

    assert_eq!(foo.first_ref(), &43);
    assert_eq!(foo.first_deref(), 43);
    assert_eq!(foo.first_clone(), 43);
    assert_eq!(foo.second_ref(), &44);
    assert_eq!(foo.second_deref(), 44);
    assert_eq!(foo.second_clone(), 44);
    assert_eq!(foo.third_ref(), &45);
    assert_eq!(foo.third_deref(), 45);
    assert_eq!(foo.third_clone(), 45);
    assert_eq!(foo.fourth_ref(), &46);
    assert_eq!(foo.fourth_deref(), 46);
    assert_eq!(foo.fourth_clone(), 46);
    assert_eq!(foo.fifth_ref(), &47);
    assert_eq!(foo.fifth_deref(), 47);
    assert_eq!(foo.fifth_clone(), 47);
    assert_eq!(foo.last_ref(), &48);
    assert_eq!(foo.last_deref(), 48);
    assert_eq!(foo.last_clone(), 48);
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
enum BarNamed {
    Foo { bar: i32, baz: i32 },
    Bar { bar: i32, baz: i32 },
}

#[test]
fn test_enum_named() {
    let mut foo = BarNamed::Foo { bar: 42, baz: 43 };
    assert_eq!(foo.foo_bar_ref(), Some(&42));
    assert_eq!(foo.foo_bar_deref(), Some(42));
    assert_eq!(foo.foo_bar_clone(), Some(42));
    assert_eq!(foo.foo_baz_ref(), Some(&43));
    assert_eq!(foo.foo_baz_deref(), Some(43));
    assert_eq!(foo.foo_baz_clone(), Some(43));

    let Some(r) = foo.foo_bar_mut() else {
        panic!("Expected Some");
    };
    *r = 44;

    let Some(r) = foo.foo_baz_mut() else {
        panic!("Expected Some");
    };
    *r = 45;

    assert_eq!(foo.foo_bar_ref(), Some(&44));
    assert_eq!(foo.foo_bar_deref(), Some(44));
    assert_eq!(foo.foo_bar_clone(), Some(44));
    assert_eq!(foo.foo_baz_ref(), Some(&45));
    assert_eq!(foo.foo_baz_deref(), Some(45));
    assert_eq!(foo.foo_baz_clone(), Some(45));

    let mut bar = BarNamed::Bar { bar: 42, baz: 43 };
    assert_eq!(bar.bar_bar_ref(), Some(&42));
    assert_eq!(bar.bar_bar_deref(), Some(42));
    assert_eq!(bar.bar_bar_clone(), Some(42));
    assert_eq!(bar.bar_baz_ref(), Some(&43));
    assert_eq!(bar.bar_baz_deref(), Some(43));
    assert_eq!(bar.bar_baz_clone(), Some(43));

    let Some(r) = bar.bar_bar_mut() else {
        panic!("Expected Some");
    };
    *r = 44;

    let Some(r) = bar.bar_baz_mut() else {
        panic!("Expected Some");
    };
    *r = 45;

    assert_eq!(bar.bar_bar_ref(), Some(&44));
    assert_eq!(bar.bar_bar_deref(), Some(44));
    assert_eq!(bar.bar_bar_clone(), Some(44));
    assert_eq!(bar.bar_baz_ref(), Some(&45));
    assert_eq!(bar.bar_baz_deref(), Some(45));
    assert_eq!(bar.bar_baz_clone(), Some(45));
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
enum BarTuple {
    Foo(i32, i32),
    Bar(i32, i32),
}

#[test]
fn test_enum_tuple() {
    let mut foo = BarTuple::Foo(42, 43);
    assert_eq!(foo.foo_first_ref(), Some(&42));
    assert_eq!(foo.foo_first_deref(), Some(42));
    assert_eq!(foo.foo_first_clone(), Some(42));
    assert_eq!(foo.foo_last_ref(), Some(&43));
    assert_eq!(foo.foo_last_deref(), Some(43));
    assert_eq!(foo.foo_last_clone(), Some(43));

    let Some(r) = foo.foo_first_mut() else {
        panic!("Expected Some");
    };
    *r = 44;

    let Some(r) = foo.foo_last_mut() else {
        panic!("Expected Some");
    };
    *r = 45;

    assert_eq!(foo.foo_first_ref(), Some(&44));
    assert_eq!(foo.foo_first_deref(), Some(44));
    assert_eq!(foo.foo_first_clone(), Some(44));
    assert_eq!(foo.foo_last_ref(), Some(&45));
    assert_eq!(foo.foo_last_deref(), Some(45));
    assert_eq!(foo.foo_last_clone(), Some(45));

    let mut bar = BarTuple::Bar(42, 43);
    assert_eq!(bar.bar_first_ref(), Some(&42));
    assert_eq!(bar.bar_first_deref(), Some(42));
    assert_eq!(bar.bar_first_clone(), Some(42));
    assert_eq!(bar.bar_last_ref(), Some(&43));
    assert_eq!(bar.bar_last_deref(), Some(43));
    assert_eq!(bar.bar_last_clone(), Some(43));

    let Some(r) = bar.bar_first_mut() else {
        panic!("Expected Some");
    };
    *r = 44;

    let Some(r) = bar.bar_last_mut() else {
        panic!("Expected Some");
    };
    *r = 45;

    assert_eq!(bar.bar_first_ref(), Some(&44));
    assert_eq!(bar.bar_first_deref(), Some(44));
    assert_eq!(bar.bar_first_clone(), Some(44));
    assert_eq!(bar.bar_last_ref(), Some(&45));
    assert_eq!(bar.bar_last_deref(), Some(45));
    assert_eq!(bar.bar_last_clone(), Some(45));
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
enum BarUnit {
    Foo(i32),
    Bar(i32),
}

#[test]
fn test_enum_unit() {
    let mut foo = BarUnit::Foo(42);
    assert_eq!(foo.foo_first_ref(), Some(&42));
    assert_eq!(foo.foo_first_deref(), Some(42));
    assert_eq!(foo.foo_first_clone(), Some(42));

    let Some(r) = foo.foo_first_mut() else {
        panic!("Expected Some");
    };
    *r = 43;

    assert_eq!(foo.foo_first_ref(), Some(&43));
    assert_eq!(foo.foo_first_deref(), Some(43));
    assert_eq!(foo.foo_first_clone(), Some(43));

    let mut bar = BarUnit::Bar(42);
    assert_eq!(bar.bar_first_ref(), Some(&42));
    assert_eq!(bar.bar_first_deref(), Some(42));
    assert_eq!(bar.bar_first_clone(), Some(42));

    let Some(r) = bar.bar_first_mut() else {
        panic!("Expected Some");
    };
    *r = 43;

    assert_eq!(bar.bar_first_ref(), Some(&43));
    assert_eq!(bar.bar_first_deref(), Some(43));
    assert_eq!(bar.bar_first_clone(), Some(43));
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
enum BarUnitWithDiscriminant {
    Foo(i32),
    Baz,
}

#[test]
fn test_enum_unit_with_discriminant() {
    let mut foo = BarUnitWithDiscriminant::Foo(42);
    assert_eq!(foo.foo_first_ref(), Some(&42));
    assert_eq!(foo.foo_first_deref(), Some(42));
    assert_eq!(foo.foo_first_clone(), Some(42));

    let Some(r) = foo.foo_first_mut() else {
        panic!("Expected Some");
    };
    *r = 43;

    assert_eq!(foo.foo_first_ref(), Some(&43));
    assert_eq!(foo.foo_first_deref(), Some(43));
    assert_eq!(foo.foo_first_clone(), Some(43));
    let _baz = BarUnitWithDiscriminant::Baz;
}

#[derive(Getters)]
#[getters(deref, clone, mutable)]
pub struct Skip {
    #[getters(skip, skip_deref, skip_clone, skip_mutable)]
    _foo: i32,
}

#[test]
fn test_skip() {
    // NOTE: THis is where we'd put our methods...if we had any!
    // let mut foo = Skip { foo: 42 };
    // assert_eq!(foo.foo_ref(), &42);
    // assert_eq!(foo.foo_deref(), 42);
    // assert_eq!(foo.foo_clone(), 42);
    // *foo.foo_mut() = 43;
    // assert_eq!(foo.foo_ref(), &43);
    // assert_eq!(foo.foo_deref(), 43);
    // assert_eq!(foo.foo_clone(), 43);
}
