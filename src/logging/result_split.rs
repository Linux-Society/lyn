pub trait SplitIntoOptions<T, E> {
    /// Splits the result into two options. `(Option<Ok>, Option<Error>)`
    fn split(self) -> (Option<T>, Option<E>);
}

impl<T, E> SplitIntoOptions<T, E> for anyhow::Result<T, E> {
    fn split(self) -> (Option<T>, Option<E>) {
        let mut ok = None;
        let mut err = None;

        match self {
            Ok(ok_case) => {
                ok = Some(ok_case);
            }
            Err(error) => {
                err = Some(error);
            }
        }

        (ok, err)
    }
}
