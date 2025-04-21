#[track_caller]
fn assert_eq<T, U>(a: T, b: U)
where T: PartialEq<U>,
      T: core::fmt::Debug,
      U: core::fmt::Debug,
{
    assert_eq!(a, b);
}

#[test]
#[allow(unused_parens, unused_braces)]
#[closure_it::closure_it]
fn test() {
    assert_eq((0..3).map(it+1).collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq((-3i32..0).map(it.abs()).collect::<Vec<_>>(), [3, 2, 1]);
    assert_eq((1+{it})(2), 3);
    assert_eq((1+[it][0])(2), 3);
    assert_eq(([it])(0), [0]);
    assert_eq((it)(0), 0);
    assert_eq(Some(2).map_or(3, it*2), 4);
    assert_eq(Some(2).map_or(3, it*2), 4);
    assert_eq(Some(2).map_or(3, (it*2)), 4);
    assert_eq(Some(2).map_or(3, ((it*2))), 4);
    assert_eq(Some(2).map_or(3, {(it*2)}), 4);
    assert_eq(Some(2).map_or(3, ({it*2})), 4);
    assert_eq(None::<i32>.map_or(3, it*2), 3);
    assert_eq((0, it, 2).1(2), 2);
    assert_eq(((0), it, (2)).1(2), 2);
    assert_eq(((0), (it), (2)).1(2), 2);
}

#[test]
#[closure_it::closure_it(this)]
fn test_other_name() {
    assert_eq((0..3).map(this+1).collect::<Vec<_>>(), [1, 2, 3]);
    assert_eq((-3i32..0).map(this.abs()).collect::<Vec<_>>(), [3, 2, 1]);
}
