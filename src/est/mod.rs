pub struct Bounds<T> {
    pub lower_bound: T,
    pub upper_bound: T,
}

pub enum OptionalClosedIntvlEstimate <T> {
    None,
    Exact(T),
    ClosedIntvl(Bounds<T>)
}