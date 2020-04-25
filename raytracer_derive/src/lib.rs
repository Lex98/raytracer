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
        impl std::ops::Add for &#name {
            type Output = #name;

            fn add(self, other: &#name) -> Self::Output {
                #name(&self.0 + &other.0)
            }
        }

        impl<'a> std::ops::AddAssign<&'a #name> for #name {
            fn add_assign(&mut self, other: &'a #name) {
                self.0 += &other.0;
            }
        }

        impl std::ops::Div<f64> for &#name {
            type Output = #name;

            fn div(self, divider: f64) -> Self::Output {
                #name(&self.0 / divider)
            }
        }

        impl std::ops::DivAssign<f64> for #name {
            fn div_assign(&mut self, divider: f64) {
                self.0 /= divider;
            }
        }

        impl std::ops::Index<usize> for #name {
            type Output = f64;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl std::ops::IndexMut<usize> for #name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl std::ops::Mul<f64> for &#name {
            type Output = #name;

            fn mul(self, multiplier: f64) -> Self::Output {
                #name(&self.0 * multiplier)
            }
        }

        impl std::ops::MulAssign<f64> for #name {
            fn mul_assign(&mut self, multiplier: f64) {
                self.0 *= multiplier;
            }
        }

        impl<'a> std::ops::Neg for &'a #name {
            type Output = #name;
            fn neg(self) -> Self::Output {
                #name(-&self.0)
            }
        }

        impl std::ops::Sub for &#name {
            type Output = #name;

            fn sub(self, other: &#name) -> Self::Output {
                #name(&self.0 - &other.0)
            }
        }

        impl<'a> std::ops::SubAssign<&'a #name> for #name {
            fn sub_assign(&mut self, other: &'a #name) {
                self.0 -= &other.0;
            }
        }

        impl<'a> IntoIterator for &'a #name {
            type Item = &'a f64;
            type IntoIter = Iter<'a, f64>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a> IntoIterator for &'a mut #name {
            type Item = &'a mut f64;
            type IntoIter = IterMut<'a, f64>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl<'a>  #name {
            pub fn iter(&'a self) -> Iter<'a, f64> {
                self.into_iter()
            }

            pub fn iter_mut(&'a mut self) -> IterMut<'a, f64> {
                self.into_iter()
            }

        }

        impl std::convert::From<[f64; 3]> for #name {
            fn from(base :[f64; 3]) -> #name {
                #name(base.into())
            }
        }
    };
    gen.into()
}
