// trait Functor<T> {
//   fn map<U, F: Fn(T) -> U>(self, f: F) -> Functor<U>;
// }
// trait Applicative<T>: Functor<T> {
//   fn pure(t: T) -> Self;
//   fn apply<U, F: Fn(T) -> U>(self, f: Self) -> Self;
// }

fn pure<T, E>(t: T) -> Result<T, E> {
  Ok(t)
}

fn apply<T, U, E>(f: Result<fn(T) -> U, E>, t: Result<T, E>) -> Result<U, E> {
  match (f, t) {
    (Ok(f), Ok(t)) => Ok(f(t)),
    (Err(e), _) => Err(e),
    (_, Err(e)) => Err(e),
  }
}