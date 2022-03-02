use crate::hardware::cpu::CpuHardware;
use crate::node::*;

#[test]
fn test_empty() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);

    let gx = grad(x, &[]);
    assert!(gx.is_empty());
}

#[test]
fn test_self() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);

    let gx = grad(x, &[x]);
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));

    // dx/dx == 1
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));
}

#[test]
fn test_unrelated() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);
    let y = Node::from_scalar(&hw, &g, 42.);

    let gx = grad(y, &[x]);
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));

    // dy/dx == 0 since y is not calculated by x.
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(0.));
}

#[test]
fn test_neg() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);
    let y = -x;

    let gx = grad(y, &[x]);
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));

    // dy/dx == -1
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(-1.));
}

#[test]
fn test_add() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a + b;

    let gx = grad(y, &[a, b]);
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert_eq!(gx[1].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert!(ptr::eq(gx[1].hardware(), &hw));

    // dy/da == 1
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));
    // dy/db == 1
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(1.));
}

#[test]
fn test_sub() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a - b;

    let gx = grad(y, &[a, b]);
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert_eq!(gx[1].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert!(ptr::eq(gx[1].hardware(), &hw));

    // dy/da == 1
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));
    // dy/db == -1
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(-1.));
}

#[test]
fn test_mul() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a * b;

    let gx = grad(y, &[a, b]);
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert_eq!(gx[1].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert!(ptr::eq(gx[1].hardware(), &hw));

    // dy/da == b
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(456.));
    // dy/db == a
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(123.));
}

#[test]
fn test_mul_quadratic() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 123.);
    // This calculation generates a diamond dependency between x and y
    // so that gradient summation x + x is happened during backpropagation.
    let y = x * x;

    let gx = grad(y, &[x]);
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));

    // dy/dx == 2x, internally calculated by x + x.
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(246.));
}

#[test]
fn test_div() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 3.);
    let b = Node::from_scalar(&hw, &g, 2.);
    let y = a / b;

    let gx = grad(y, &[a, b]);
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert_eq!(gx[1].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert!(ptr::eq(gx[1].hardware(), &hw));

    // dy/da == 1/b
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(0.5));
    // dy/db == -a/b^2
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(-0.75));
}

#[test]
fn test_multiple_computation() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 1.);
    let b = Node::from_scalar(&hw, &g, 2.);
    let c = Node::from_scalar(&hw, &g, 3.);
    let y = a + -b * c;

    let gx = grad(y, &[a, b, c]);
    assert_eq!(gx.len(), 3);

    assert_eq!(gx[0].shape(), Shape::new([]));
    assert_eq!(gx[1].shape(), Shape::new([]));
    assert_eq!(gx[2].shape(), Shape::new([]));
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert!(ptr::eq(gx[1].hardware(), &hw));
    assert!(ptr::eq(gx[2].hardware(), &hw));

    // dy/da == 1
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));
    // dy/db == -c
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(-3.));
    // dy/dc = -b
    assert_eq!(gx[2].calculate().unwrap().get_scalar_f32(), Ok(-2.));
}

#[test]
fn test_higher_order_gradients() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 5.);
    let y = x * x * x;

    let gx1 = grad(y, &[x])[0];
    let gx2 = grad(gx1, &[x])[0];
    let gx3 = grad(gx2, &[x])[0];
    let gx4 = grad(gx3, &[x])[0];

    assert_eq!(gx1.shape(), Shape::new([]));
    assert_eq!(gx2.shape(), Shape::new([]));
    assert_eq!(gx3.shape(), Shape::new([]));
    assert_eq!(gx4.shape(), Shape::new([]));
    assert!(ptr::eq(gx1.hardware(), &hw));
    assert!(ptr::eq(gx2.hardware(), &hw));
    assert!(ptr::eq(gx3.hardware(), &hw));
    assert!(ptr::eq(gx4.hardware(), &hw));

    // y' == dy/dx == 3x^2
    assert_eq!(gx1.calculate().unwrap().get_scalar_f32(), Ok(75.));
    // y'' == 6x
    assert_eq!(gx2.calculate().unwrap().get_scalar_f32(), Ok(30.));
    // y''' == 6
    assert_eq!(gx3.calculate().unwrap().get_scalar_f32(), Ok(6.));
    // y'''' == 0
    assert_eq!(gx4.calculate().unwrap().get_scalar_f32(), Ok(0.));
}

#[test]
fn test_gradient_of_multiple_variables() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 2.);
    let b = Node::from_scalar(&hw, &g, 3.);
    let y = a * a * b;

    let y_a = grad(y, &[a])[0];
    let y_b = grad(y, &[b])[0];

    let y_aa = grad(y_a, &[a])[0];
    let y_ab = grad(y_a, &[b])[0];
    let y_ba = grad(y_b, &[a])[0];
    let y_bb = grad(y_b, &[b])[0];

    let y_aaa = grad(y_aa, &[a])[0];
    let y_aab = grad(y_aa, &[b])[0];
    let y_aba = grad(y_ab, &[a])[0];
    let y_abb = grad(y_ab, &[b])[0];
    let y_baa = grad(y_ba, &[a])[0];
    let y_bab = grad(y_ba, &[b])[0];
    let y_bba = grad(y_bb, &[a])[0];
    let y_bbb = grad(y_bb, &[b])[0];

    assert_eq!(y_a.calculate().unwrap().get_scalar_f32(), Ok(12.)); // 2ab
    assert_eq!(y_b.calculate().unwrap().get_scalar_f32(), Ok(4.)); // a^2

    assert_eq!(y_aa.calculate().unwrap().get_scalar_f32(), Ok(6.)); // 2b
    assert_eq!(y_ab.calculate().unwrap().get_scalar_f32(), Ok(4.)); // 2a
    assert_eq!(y_ba.calculate().unwrap().get_scalar_f32(), Ok(4.)); // 2a
    assert_eq!(y_bb.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0

    assert_eq!(y_aaa.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0
    assert_eq!(y_aab.calculate().unwrap().get_scalar_f32(), Ok(2.)); // 2
    assert_eq!(y_aba.calculate().unwrap().get_scalar_f32(), Ok(2.)); // 2
    assert_eq!(y_abb.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0
    assert_eq!(y_baa.calculate().unwrap().get_scalar_f32(), Ok(2.)); // 2
    assert_eq!(y_bab.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0
    assert_eq!(y_bba.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0
    assert_eq!(y_bbb.calculate().unwrap().get_scalar_f32(), Ok(0.)); // 0
}

#[test]
#[should_panic]
fn test_different_graph() {
    let hw = RefCell::new(CpuHardware::new());
    let g1 = RefCell::new(Graph::new());
    let g2 = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g1, 2.);
    let y = Node::from_scalar(&hw, &g2, 3.);
    let _gx = grad(y, &[x])[0];
}
