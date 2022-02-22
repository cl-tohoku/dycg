use crate::operator::*;

pub(crate) struct Sub;

impl Sub {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'hw> Operator<'hw> for Sub {
    fn name(&self) -> String {
        String::from("Sub")
    }

    fn input_size(&self) -> usize {
        2
    }

    fn perform_shape(&self, inputs: &[&Shape]) -> Result<Shape> {
        inputs[0].elementwise(inputs[1])
    }

    fn perform(&self, inputs: &[&Array<'hw>]) -> Result<Array<'hw>> {
        inputs[0].elementwise_sub_f32(inputs[1])
    }

    fn gradient<'op: 'g, 'g>(
        &self,
        _x: &[Node<'hw, 'op, 'g>],
        _y: Node<'hw, 'op, 'g>,
        gy: Node<'hw, 'op, 'g>,
    ) -> Result<Vec<Node<'hw, 'op, 'g>>>
    where
        'hw: 'op,
    {
        Ok(vec![gy, -gy])
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::CpuHardware;
    use crate::make_shape;
    use crate::operator::sub::*;

    #[test]
    fn test_properties() {
        let op = Sub::new();
        assert_eq!(op.name(), "Sub");
        assert_eq!(op.input_size(), 2);
    }

    #[rustfmt::skip]
    #[test]
    fn test_perform_shape() {
        let op = Sub::new();
        assert_eq!(op.perform_shape(&[&make_shape![], &make_shape![]]), Ok(make_shape![]));
        assert_eq!(op.perform_shape(&[&make_shape![0], &make_shape![0]]), Ok(make_shape![0]));
        assert_eq!(op.perform_shape(&[&make_shape![3], &make_shape![3]]), Ok(make_shape![3]));
    }

    #[rustfmt::skip]
    #[test]
    fn test_perform_shape_invalid() {
        let op = Sub::new();
        assert!(op.perform_shape(&[&make_shape![], &make_shape![0]]).is_err());
        assert!(op.perform_shape(&[&make_shape![], &make_shape![3]]).is_err());
        assert!(op.perform_shape(&[&make_shape![0], &make_shape![]]).is_err());
        assert!(op.perform_shape(&[&make_shape![0], &make_shape![3]]).is_err());
        assert!(op.perform_shape(&[&make_shape![3], &make_shape![]]).is_err());
        assert!(op.perform_shape(&[&make_shape![3], &make_shape![0]]).is_err());
    }

    #[test]
    fn test_perform_hardware() {
        let hw1 = RefCell::new(CpuHardware::new());
        let hw2 = RefCell::new(CpuHardware::new());
        let op = Sub::new();

        assert!(ptr::eq(op.perform_hardware(&[&hw1, &hw1]).unwrap(), &hw1));
        assert!(ptr::eq(op.perform_hardware(&[&hw2, &hw2]).unwrap(), &hw2));
        assert!(op.perform_hardware(&[&hw1, &hw2]).is_err());
    }

    #[test]
    fn test_perform() {
        let hw = RefCell::new(CpuHardware::new());
        let op = Sub::new();
        let lhs = Array::scalar_f32(&hw, 1.);
        let rhs = Array::scalar_f32(&hw, 2.);
        let expected = Array::scalar_f32(&hw, -1.);
        let observed = op.perform(&[&lhs, &rhs]).unwrap();
        assert_eq!(observed.shape(), expected.shape());
        assert_eq!(observed.get_scalar_f32(), expected.get_scalar_f32());
    }
}
