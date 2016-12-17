#[macro_use]
extern crate peel;
use peel::prelude::*;

use std::cell::Cell;

#[test]
fn arenatree_success_create() {
    struct DropTracker<'a>(&'a Cell<u32>);
    impl<'a> Drop for DropTracker<'a> {
        fn drop(&mut self) {
            self.0.set(&self.0.get() + 1);
        }
    }

    let drop_counter = Cell::new(0);
    {
        let mut new_counter = 0;
        let arena = &mut Arena::new();
        macro_rules! new {
            () => {{
                new_counter += 1;
                arena.new_node((new_counter, DropTracker(&drop_counter)))
            }}
        };

        let a = new!();  // 1
        a.append(new!(), arena);  // 2
        a.append(new!(), arena);  // 3
        a.prepend(new!(), arena);  // 4
        let b = new!();  // 5
        b.append(a, arena);
        a.insert_before(new!(), arena);  // 6
        a.insert_before(new!(), arena);  // 7
        a.insert_after(new!(), arena);  // 8
        a.insert_after(new!(), arena);  // 9
        let c = new!();  // 10
        b.append(c, arena);

        assert_eq!(drop_counter.get(), 0);
        arena[c].previous_sibling().unwrap().detach(arena);
        assert_eq!(drop_counter.get(), 0);

        assert_eq!(b.descendants(arena).map(|node| arena[node].data.0).collect::<Vec<_>>(),
                   [5, 6, 7, 1, 4, 2, 3, 9, 10]);
    }

    assert_eq!(drop_counter.get(), 10);
}

#[test]
#[should_panic]
fn arenatree_failure_prepend() {
    let arena = &mut Arena::new();
    let a = arena.new_node(1);
    let b = arena.new_node(2);
    a.prepend(b, arena);
}

#[test]
fn arenatree_success_detach() {
    let arena = &mut Arena::new();
    let a = arena.new_node(1);
    let b = arena.new_node(1);
    a.append(b, arena);
    assert_eq!(b.ancestors(arena).into_iter().count(), 2);
    b.detach(arena);
    assert_eq!(b.ancestors(arena).into_iter().count(), 1);
}
