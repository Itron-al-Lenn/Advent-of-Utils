use crate::AocOption;

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn Solution>;
}

pub trait Solution: Send + Sync + CloneBox {
    fn part1(&self, _input: String) -> AocOption {
        AocOption::None
    }
    fn part2(&self, _input: String) -> AocOption {
        AocOption::None
    }
}

impl<T> CloneBox for T
where
    T: Send + Sync + Clone + 'static,
    T: Solution,
{
    fn clone_box(&self) -> Box<dyn Solution> {
        Box::new(self.clone())
    }
}
