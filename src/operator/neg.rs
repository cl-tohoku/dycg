use crate::operator::*;

pub(crate) struct Neg;

impl Neg {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl<'hw> Operator<'hw> for Neg {
    fn name(&self) -> String {
        String::from("Neg")
    }

    fn input_size(&self) -> usize {
        1
    }

    fn perform_shape(&self, inputs: &[&Shape]) -> Result<Shape> {
        Ok(inputs[0].clone())
    }

    fn perform(&self, inputs: &[&Array<'hw>]) -> Result<Array<'hw>> {
        Ok(inputs[0].elementwise_neg_f32())
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
        Ok(vec![-gy])
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::cpu::CpuHardware;
    use crate::make_shape;
    use crate::operator::neg::*;

    #[test]
    fn test_properties() {
        let op = Neg::new();
        assert_eq!(op.name(), "Neg");
        assert_eq!(op.input_size(), 1);
    }

    #[rustfmt::skip]
    #[test]
    fn test_perform_shape() {
        let op = Neg::new();
        
        assert_eq!(op.perform_shape(&[&make_shape![]]), Ok(make_shape![]));
        assert_eq!(op.perform_shape(&[&make_shape![0]]), Ok(make_shape![0]));
        assert_eq!(op.perform_shape(&[&make_shape![3]]), Ok(make_shape![3]));
    }

    #[test]
    fn test_perform_hardware() {
        let hw = RefCell::new(CpuHardware::new());
        let op = Neg::new();

        assert!(ptr::eq(op.perform_hardware(&[&hw]).unwrap(), &hw));
    }

    #[test]
    fn test_perform() {
        let hw = RefCell::new(CpuHardware::new());
        let op = Neg::new();
        let input = Array::scalar_f32(&hw, 42.);
        let expected = Array::scalar_f32(&hw, -42.);
        let observed = op.perform(&[&input]).unwrap();
        assert_eq!(observed.shape(), expected.shape());
        assert_eq!(observed.get_scalar_f32(), expected.get_scalar_f32());
    }
}
