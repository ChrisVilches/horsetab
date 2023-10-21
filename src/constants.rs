// TODO: It's no longer just about click lengths, but also has a "reset". So it's more like
//       "sequence instructions" or something like that. Update name.
#[derive(Clone)]
pub enum MouseClickKind {
  Short,
  Long,
  Reset,
}
