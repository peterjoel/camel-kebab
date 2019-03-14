macro_rules! impl_str_ext {
    (trait: $trait: ident,
    case: $case: ident,
    as: $as_name: ident,
    as_unchecked: $as_unchecked_name: ident,
    is: $is_name: ident) => {
        trait $trait: $crate::CaseExt {
            fn $as_name(&self) -> Option<$case<'_>>;
            fn $as_unchecked_name(&self) -> $case<'_>;
            fn $is_name(&self) -> bool;
        }

        impl $trait for str {
            #[inline]
            fn $as_name(&self) -> Option<$case<'_>> {
                <Self as $crate::CaseExt>::as_case::<$case>(self)
            }

            #[inline]
            fn $as_unchecked_name(&self) -> $case<'_> {
                <Self as $crate::CaseExt>::as_case_unchecked::<$case>(self)
            }

            #[inline]
            fn $is_name(&self) -> bool {
                <Self as $crate::CaseExt>::is_case::<$case>(self)
            }
        }
    };
}

macro_rules! impl_from {
    ($($from: ident),+ => $to: ident) => {
        $(
            impl<'a> std::convert::From<$from<'a>> for $to<'a> {
                fn from(other: $from) -> $to {
                    use $crate::internals::Case;
                    $to::from_cased_words(other.to_cased_words())
                }
            }
        )+
    }
}

macro_rules! impl_eq {
    ($case: ident) => {
        impl<'a> std::cmp::PartialEq for $case<'a> {
            fn eq(&self, other: &$case<'a>) -> bool {
                $crate::internals::display_eq(&self.to_string(), other)
            }
        }

        impl<'a> std::cmp::Eq for $case<'a> {}
    };
}
