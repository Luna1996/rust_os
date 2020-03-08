#[macro_export]
macro_rules! print {
  ($($arg:tt)*) => {
    (((*$crate::ST).ConOut).OutputString)(((*$crate::ST).ConOut), $crate::s16::S16::from(format!($($arg)*)).0.as_ptr());
  };
}
