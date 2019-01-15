// http://jadpole.github.io/rust/builder-macro

#[macro_export]
macro_rules! builder {
    ( $src_name:ident => $dest_name:ident {
        $( $attr_name:ident : $attr_type:ty = $attr_default:expr ),*
    })
    => {
        pub struct $dest_name {
            $( $attr_name : $attr_type ),*
        }

        pub struct $src_name {
            $( pub $attr_name : $attr_type ),*
        }

        impl $src_name {
            pub fn new() -> $src_name {
                $src_name {
                    $( $attr_name : $attr_default ),*
                }
            }

            pub fn build(&self) -> $dest_name {


                $dest_name {
                    $( $attr_name : self.$attr_name ),*
                }
            }
        }
    }
}