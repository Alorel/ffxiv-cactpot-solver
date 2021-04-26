#[derive(Debug)]
pub enum FillFailure {
    ValueAlreadyContained,
    PositionAlreadyFilled,
}
