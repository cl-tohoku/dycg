use crate::hardware::cpu::CpuHardware;
use crate::make_shape;
use crate::node::*;

#[test]
fn test_steps() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());
    let lhs = Node::from_scalar(&hw, &g, 1.);
    let rhs = Node::from_scalar(&hw, &g, 2.);
    let ret = lhs + rhs;

    assert_eq!(lhs, Node::new(&g, 0));
    assert_eq!(rhs, Node::new(&g, 1));
    assert_eq!(ret, Node::new(&g, 2));
    assert_eq!(lhs.shape(), make_shape![]);
    assert_eq!(rhs.shape(), make_shape![]);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(lhs.hardware(), &hw));
    assert!(ptr::eq(rhs.hardware(), &hw));
    assert!(ptr::eq(ret.hardware(), &hw));

    {
        let g = g.borrow();
        assert_eq!(g.num_steps(), 3);
        assert_eq!(g.get_step(0).unwrap().operator.name(), "Constant");
        assert_eq!(g.get_step(1).unwrap().operator.name(), "Constant");
        assert_eq!(g.get_step(2).unwrap().operator.name(), "Add");
    }

    let retval = ret.calculate().unwrap();
    assert_eq!(*retval.shape(), make_shape![]);
    assert_eq!(retval.get_scalar_f32(), Ok(3.));
}

#[test]
fn test_neg() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let src = Node::from_scalar(&hw, &g, 42.);
    let dest = -src;

    assert_eq!(src.shape(), make_shape![]);
    assert_eq!(dest.shape(), make_shape![]);
    assert!(ptr::eq(src.hardware(), &hw));
    assert!(ptr::eq(dest.hardware(), &hw));

    assert_eq!(dest.calculate().unwrap().get_scalar_f32(), Ok(-42.));
}

#[test]
fn test_add() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let lhs = Node::from_scalar(&hw, &g, 1.);
    let rhs = Node::from_scalar(&hw, &g, 2.);
    let ret = lhs + rhs;

    assert_eq!(lhs.shape(), make_shape![]);
    assert_eq!(rhs.shape(), make_shape![]);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(lhs.hardware(), &hw));
    assert!(ptr::eq(rhs.hardware(), &hw));
    assert!(ptr::eq(ret.hardware(), &hw));

    assert_eq!(ret.calculate().unwrap().get_scalar_f32(), Ok(3.));
}

#[test]
fn test_sub() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let lhs = Node::from_scalar(&hw, &g, 1.);
    let rhs = Node::from_scalar(&hw, &g, 2.);
    let ret = lhs - rhs;

    assert_eq!(lhs.shape(), make_shape![]);
    assert_eq!(rhs.shape(), make_shape![]);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(lhs.hardware(), &hw));
    assert!(ptr::eq(rhs.hardware(), &hw));
    assert!(ptr::eq(ret.hardware(), &hw));

    assert_eq!(ret.calculate().unwrap().get_scalar_f32(), Ok(-1.));
}

#[test]
fn test_mul() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let lhs = Node::from_scalar(&hw, &g, 1.);
    let rhs = Node::from_scalar(&hw, &g, 2.);
    let ret = lhs * rhs;

    assert_eq!(lhs.shape(), make_shape![]);
    assert_eq!(rhs.shape(), make_shape![]);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(lhs.hardware(), &hw));
    assert!(ptr::eq(rhs.hardware(), &hw));
    assert!(ptr::eq(ret.hardware(), &hw));

    assert_eq!(ret.calculate().unwrap().get_scalar_f32(), Ok(2.));
}

#[test]
fn test_div() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let lhs = Node::from_scalar(&hw, &g, 1.);
    let rhs = Node::from_scalar(&hw, &g, 2.);
    let ret = lhs / rhs;

    assert_eq!(lhs.shape(), make_shape![]);
    assert_eq!(rhs.shape(), make_shape![]);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(lhs.hardware(), &hw));
    assert!(ptr::eq(rhs.hardware(), &hw));
    assert!(ptr::eq(ret.hardware(), &hw));

    assert_eq!(ret.calculate().unwrap().get_scalar_f32(), Ok(0.5));
}

#[test]
fn test_fill_scalar() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());
    let ret = Node::fill(&hw, &g, make_shape![], 123.);
    assert_eq!(ret.shape(), make_shape![]);
    assert!(ptr::eq(ret.hardware(), &hw));
    assert_eq!(ret.calculate().unwrap().get_scalar_f32(), Ok(123.));
}

#[test]
fn test_fill_0() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());
    let ret = Node::fill(&hw, &g, make_shape![0], 123.);
    assert_eq!(ret.shape(), make_shape![0]);
    assert!(ptr::eq(ret.hardware(), &hw));
    assert_eq!(ret.calculate().unwrap().get_values_f32(), vec![]);
}

#[test]
fn test_fill_n() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());
    let ret = Node::fill(&hw, &g, make_shape![3], 123.);
    assert_eq!(ret.shape(), make_shape![3]);
    assert!(ptr::eq(ret.hardware(), &hw));
    assert_eq!(
        ret.calculate().unwrap().get_values_f32(),
        vec![123., 123., 123.]
    );
}

#[test]
fn test_multiple_computation() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 1.);
    let b = Node::from_scalar(&hw, &g, 2.);
    let c = Node::from_scalar(&hw, &g, 3.);
    let y = a + -b * c;

    assert_eq!(a.shape(), make_shape![]);
    assert_eq!(b.shape(), make_shape![]);
    assert_eq!(c.shape(), make_shape![]);
    assert_eq!(y.shape(), make_shape![]);
    assert!(ptr::eq(a.hardware(), &hw));
    assert!(ptr::eq(b.hardware(), &hw));
    assert!(ptr::eq(c.hardware(), &hw));
    assert!(ptr::eq(y.hardware(), &hw));

    assert_eq!(y.calculate().unwrap().get_scalar_f32(), Ok(-5.));
}

#[test]
fn test_grad_self() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);

    let gx = grad(x, &[x]).unwrap();
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));
}

#[test]
fn test_grad_unrelated() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);
    let y = Node::from_scalar(&hw, &g, 42.);

    let gx = grad(y, &[x]).unwrap();
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(0.));
}

#[test]
fn test_grad_neg() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let x = Node::from_scalar(&hw, &g, 42.);
    let y = -x;

    let gx = grad(y, &[x]).unwrap();
    assert_eq!(gx.len(), 1);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(-1.));
}

#[test]
fn test_grad_add() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a + b;

    let gx = grad(y, &[a, b]).unwrap();
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));

    assert_eq!(gx[1].shape(), make_shape![]);
    assert!(ptr::eq(gx[1].hardware(), &hw));
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(1.));
}

#[test]
fn test_grad_sub() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a - b;

    let gx = grad(y, &[a, b]).unwrap();
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(1.));

    assert_eq!(gx[1].shape(), make_shape![]);
    assert!(ptr::eq(gx[1].hardware(), &hw));
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(-1.));
}

#[test]
fn test_grad_mul() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 123.);
    let b = Node::from_scalar(&hw, &g, 456.);
    let y = a * b;

    let gx = grad(y, &[a, b]).unwrap();
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(456.));

    assert_eq!(gx[1].shape(), make_shape![]);
    assert!(ptr::eq(gx[1].hardware(), &hw));
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(123.));
}

#[test]
fn test_grad_div() {
    let hw = RefCell::new(CpuHardware::new());
    let g = RefCell::new(Graph::new());

    let a = Node::from_scalar(&hw, &g, 3.);
    let b = Node::from_scalar(&hw, &g, 2.);
    let y = a / b;

    let gx = grad(y, &[a, b]).unwrap();
    assert_eq!(gx.len(), 2);

    assert_eq!(gx[0].shape(), make_shape![]);
    assert!(ptr::eq(gx[0].hardware(), &hw));
    // dy/da == 1/b
    assert_eq!(gx[0].calculate().unwrap().get_scalar_f32(), Ok(0.5));

    assert_eq!(gx[1].shape(), make_shape![]);
    assert!(ptr::eq(gx[1].hardware(), &hw));
    // dy/db == -a/b^2
    assert_eq!(gx[1].calculate().unwrap().get_scalar_f32(), Ok(-0.75));
}
