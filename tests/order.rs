use lull_spec::{Order, ReadOrderIdInnerRef, ReadOrderInner, ReadOrderInnerRef};

#[test]
fn order_my_struct() {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct MyOrderStruct {
        id: usize,
    }

    impl ReadOrderIdInnerRef<usize> for MyOrderStruct {
        fn read_order_id_inner_ref(&self) -> &usize {
            &self.id
        }
    }

    let order = Order::new(MyOrderStruct { id: 1000usize });
    assert_eq!(
        order.read_order_inner_ref(),
        &MyOrderStruct { id: 1000usize }
    );
    assert_eq!(order.read_order_inner(), MyOrderStruct { id: 1000usize });
}
