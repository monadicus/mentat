#[macro_export]
macro_rules! CreateErrorType {
  (@step) => {};
  (@step ($(#[$doc:meta])* new, $fn_name:ident, $($arg_name:ident,)*, $($err_msg:expr,)*, $($suggestion:expr,)*), $(($(#[$docs:meta])* $new_or_from:ident, $fn_names:ident, $($arg_names:ident,)*, $($err_msgs:expr,)*, $($suggestions:expr,)*),)* ) => {
      $(#[$doc])*
      #[track_caller]
      pub fn $fn_name<C>($($arg_name: impl core::fmt::Display,)*) -> super::Result<C> {
        Err(
          $crate::Report::new(Self)
            $(.attach_printable($crate::ErrorMsg::from(format!($err_msg))))*
            $(.attach_printable($crate::Suggestion::from(format!($suggestion))))*
        )?
      }

      CreateErrorType!(@step $(($(#[$docs])* $new_or_from, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
  };
  (@step ($(#[$doc:meta])* from, $fn_name:ident, $($arg_name:ident,)*, $($err_msg:expr,)*, $($suggestion:expr,)*), $(($(#[$docs:meta])* $new_or_from:ident, $fn_names:ident, $($arg_names:ident,)*, $($err_msgs:expr,)*, $($suggestions:expr,)*),)* ) => {
      $(#[$doc])*
      #[track_caller]
      pub fn $fn_name<F: $crate::IntoReport<Ok = C>, C>(into_report: F, $($arg_name: impl core::fmt::Display,)*) -> super::Result<C> {
          use $crate::ResultExt;

          Ok(
            into_report
              .report()
              .change_context(Self)
              $(.attach_printable($crate::ErrorMsg::from(format!($err_msg))))*
              $(.attach_printable($crate::Suggestion::from(format!($suggestion))))*
              ?
          )
      }

      CreateErrorType!(@step $(($(#[$docs])* $new_or_from, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
  };
  ($struct_name:ident $($(#[$docs:meta])* $new_or_from:ident $fn_names:ident { args: ($($arg_names:ident$(,)?)*), error_msgs: [$($err_msgs:expr$(,)?)*], suggestions: [$($suggestions:expr$(,)?)*], })* ) => {
      use $crate::Colorize;

      #[derive(Debug, Default, thiserror::Error)]
      pub struct $struct_name;

      impl core::fmt::Display for $struct_name {
          fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
              let error_ident = stringify!($struct_name);
              let (name, error) = error_ident.split_at(error_ident.len() - 5);
              write!(f, "{} {}:", name.bold().red(), error.bold().red())
          }
      }

      impl $struct_name {
          CreateErrorType!(@step $(($(#[$docs])* $new_or_from, $fn_names, $($arg_names,)*, $($err_msgs,)*, $($suggestions,)*),)*);
      }
  };
}
