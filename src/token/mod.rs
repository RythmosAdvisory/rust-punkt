pub use token::word::SentenceWordToken;
pub use token::training::TrainingToken;

macro_rules! impl_flags(
  ($t:ident, $f:ty) => (
    impl WordTokenWithFlags for $t {
      type Flags = $f;

      #[inline]
      fn flags(&self) -> &$f {
        &self.flags
      }

      #[inline]
      fn flags_mut(&mut self) -> &mut $f {
        &mut self.flags
      }
    }
  )
);

mod word;
mod training;
pub mod prelude;
