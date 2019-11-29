
use crate::v8::MaybeBool;

impl MaybeBool {
    pub  fn is_nothing(self) ->bool {
          unsafe {
              cpp!([self as "v8::Maybe<bool>"] -> bool as "bool" {
                 return self.IsNothing();
              })
          }
  
      }
  
      fn is_just(self) ->bool {
          unsafe {
              return cpp!([self as "v8::Maybe<bool>"] -> bool as "bool" {
                 return self.IsJust();
              })
          }
      }
  
   pub   fn to_checked(self) -> bool {
          unsafe {
              cpp!([self as "v8::Maybe<bool>"] -> bool as "bool" {
                 return self.ToChecked();
              })
          }
      }
  }
  