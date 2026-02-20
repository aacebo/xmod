use xtera::Scope;
use xtera_derive::render;

#[test]
fn basic_for_loop() {
    let tpl = render! {
        @for (item of items; track item) {
            "[" {{ item }} "]"
        }
    };

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64]));
    assert_eq!(tpl.render(&s).unwrap(), "[1][2][3]");
}

#[test]
fn empty_iterable() {
    let tpl = render! {
        @for (n of items; track n) { "x" }
    };

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!((Vec::<xval::Value>::new())));
    s.set_template("main", tpl);
    assert_eq!(s.render("main").unwrap(), "");
}

#[test]
fn nested_for_loops() {
    let tpl = render! {
        @for (row of rows; track row) {
            @for (col of row; track col) {
                {{ col }}
            }
            "|"
        }
    };

    let mut s = Scope::new();
    s.set_var("rows", xval::valueof!([[1_i64, 2_i64], [3_i64, 4_i64]]));
    s.set_template("main", tpl);
    assert_eq!(s.render("main").unwrap(), "12|34|");
}

#[test]
fn for_with_if() {
    let tpl = render! {
        @for (n of items; track n) {
            @if (n % 2 == 0) { "even" } @else { "odd" }
        }
    };

    let mut s = Scope::new();
    s.set_var("items", xval::valueof!([1_i64, 2_i64, 3_i64, 4_i64]));
    assert_eq!(tpl.render(&s).unwrap(), "oddevenoddeven");
}
