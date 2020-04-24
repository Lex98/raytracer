extern crate proc_macro;
extern crate syn;
#[macro_use] 
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Base3Ops)]
pub fn derive_base3_ops(tokens: TokenStream) -> TokenStream {
    base3_ops_impl(tokens)
}

fn base3_ops_impl(tokens: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(tokens).unwrap();
    let name = &ast.ident;

    // Generate the `impl`
    let gen = quote! {
        impl ops::Add for &#name {
            type Output = #name;

            fn add(self, other: &#name) -> Self::Output {
                #name(&self.0 + &other.0)
            }
        }

        impl<'a> ops::AddAssign<&'a #name> for #name {
            fn add_assign(&mut self, other: &'a #name) {
                self.0 += &other.0;
            }
        }

        impl ops::Div<f64> for &#name {
            type Output = #name;

            fn div(self, divider: f64) -> Self::Output {
                #name(&self.0 / divider)
            }
        }

        impl ops::DivAssign<f64> for #name {
            fn div_assign(&mut self, divider: f64) {
                self.0 /= divider;
            }
        }

        impl ops::Index<usize> for #name {
            type Output = f64;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl ops::IndexMut<usize> for #name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl ops::Mul<f64> for &#name {
            type Output = #name;

            fn mul(self, multiplier: f64) -> Self::Output {
                #name(&self.0 * multiplier)
            }
        }

        impl ops::MulAssign<f64> for #name {
            fn mul_assign(&mut self, multiplier: f64) {
                self.0 *= multiplier;
            }
        }

        impl<'a> ops::Neg for &'a #name {
            type Output = #name;
            fn neg(self) -> Self::Output {
                #name(-&self.0)
            }
        }

        impl ops::Sub for &#name {
            type Output = #name;

            fn sub(self, other: &#name) -> Self::Output {
                #name(&self.0 - &other.0)
            }
        }

        impl<'a> ops::SubAssign<&'a #name> for #name {
            fn sub_assign(&mut self, other: &'a #name) {
                self.0 -= &other.0;
            }
        }
    };
    gen.into()
}

