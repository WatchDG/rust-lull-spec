use lull_spec::{OrderId, ReadOrderId, ReadOrderIdInner, ReadOrderIdInnerRef, WriteOrderIdInner};

#[test]
fn read_order_id_usize() {
    let order_id = OrderId::new(1000usize);
    assert_eq!(order_id.read_order_id_inner_ref(), &1000usize);
    assert_eq!(order_id.read_order_id_inner(), 1000usize);
    assert_eq!(order_id.read_order_id(), OrderId::new(1000usize));
}

#[test]
fn read_order_id_string() {
    let order_id = OrderId::new(String::from("abc"));
    assert_eq!(order_id.read_order_id_inner_ref(), &String::from("abc"));
    assert_eq!(order_id.read_order_id_inner(), String::from("abc"));
    assert_eq!(order_id.read_order_id(), OrderId::new(String::from("abc")));
}

#[test]
fn read_order_id_my_struct() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MyStruct {
        id: String,
    }

    let order_id = OrderId::new(MyStruct {
        id: String::from("abc"),
    });
    assert_eq!(
        order_id.read_order_id_inner_ref(),
        &MyStruct {
            id: String::from("abc")
        }
    );
    assert_eq!(
        order_id.read_order_id_inner(),
        MyStruct {
            id: String::from("abc")
        }
    );
    assert_eq!(
        order_id.read_order_id(),
        OrderId::new(MyStruct {
            id: String::from("abc")
        })
    );
}

#[test]
fn write_order_id_usize() {
    let mut order_id = OrderId::new(1000usize);
    assert_eq!(order_id.read_order_id_inner(), 1000usize);
    order_id
        .write_order_id_inner(2000usize)
        .write_order_id_inner(3000usize);
    assert_eq!(order_id.read_order_id_inner(), 3000usize);
}
