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
        impl<T: ops::Add<T, Output = T> + Copy> std::ops::Add for &#name<T> {
            type Output = #name<T>;

            fn add(self, other: &#name<T>) -> Self::Output {
                #name(&self.0 + &other.0)
            }
        }

        impl<T: ops::Add<T, Output = T> + Copy> std::ops::Add for #name<T> {
            type Output = #name<T>;

            fn add(self, other: #name<T>) -> Self::Output {
                #name(&self.0 + &other.0)
            }
        }

        impl<'a, T: ops::AddAssign<T> + Copy> std::ops::AddAssign<&'a #name<T>> for #name<T> {
            fn add_assign(&mut self, other: &'a #name<T>) {
                self.0 += &other.0;
            }
        }

        impl<T: ops::Div<T, Output = T> + Copy> std::ops::Div<T> for &#name<T> {
            type Output = #name<T>;

            fn div(self, divider: T) -> Self::Output {
                #name(&self.0 / divider)
            }
        }

        impl<T: ops::Div<T, Output = T> + Copy> std::ops::Div<T> for #name<T> {
            type Output = #name<T>;

            fn div(self, divider: T) -> Self::Output {
                #name(&self.0 / divider)
            }
        }

        impl<T: ops::DivAssign<T> + Copy> std::ops::DivAssign<T> for #name<T> {
            fn div_assign(&mut self, divider: T) {
                self.0 /= divider;
            }
        }

        impl<T> std::ops::Index<usize> for #name<T> {
            type Output = T;

            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }

        impl<T> std::ops::IndexMut<usize> for #name<T> {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }

        impl<T: ops::Mul<T, Output = T> + Copy> std::ops::Mul<T> for &#name<T> {
            type Output = #name<T>;

            fn mul(self, multiplier: T) -> Self::Output {
                #name(&self.0 * multiplier)
            }
        }

        impl<T: ops::Mul<T, Output = T> + Copy> std::ops::Mul<T> for #name<T> {
            type Output = #name<T>;

            fn mul(self, multiplier: T) -> Self::Output {
                #name(&self.0 * multiplier)
            }
        }

        impl<T: ops::MulAssign<T> + Copy> std::ops::MulAssign<T> for #name<T> {
            fn mul_assign(&mut self, multiplier: T) {
                self.0 *= multiplier;
            }
        }

        impl<'a, T: ops::Neg<Output = T> + Copy> std::ops::Neg for &'a #name<T> {
            type Output = #name<T>;
            fn neg(self) -> Self::Output {
                #name(-&self.0)
            }
        }

        impl<'a, T: ops::Neg<Output = T> + Copy> std::ops::Neg for #name<T> {
            type Output = #name<T>;
            fn neg(self) -> Self::Output {
                #name(-&self.0)
            }
        }

        impl<T: ops::Sub<T, Output = T> + Copy> std::ops::Sub for &#name<T> {
            type Output = #name<T>;

            fn sub(self, other: &#name<T>) -> Self::Output {
                #name(&self.0 - &other.0)
            }
        }

        impl<T: ops::Sub<T, Output = T> + Copy> std::ops::Sub for #name<T> {
            type Output = #name<T>;

            fn sub(self, other: #name<T>) -> Self::Output {
                #name(&self.0 - &other.0)
            }
        }

        impl<'a, T: ops::SubAssign<T> + Copy> std::ops::SubAssign<&'a #name<T>> for #name<T> {
            fn sub_assign(&mut self, other: &'a #name<T>) {
                self.0 -= &other.0;
            }
        }

        impl<'a, T> IntoIterator for &'a #name<T> {
            type Item = &'a T;
            type IntoIter = Iter<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut #name<T> {
            type Item = &'a mut T;
            type IntoIter = IterMut<'a, T>;

            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl<'a, T>  #name<T> {
            pub fn iter(&'a self) -> Iter<'a, T> {
                self.into_iter()
            }

            pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
                self.into_iter()
            }
        }

        impl<T> std::convert::From<[T; 3]> for #name<T> {
            fn from(base :[T; 3]) -> #name<T> {
                #name(base.into())
            }
        }

        impl<T: SampleUniform + Copy> #name<T> {
            pub fn random(min: T, max: T) -> Self {
                #name(Base3::random(min, max).into())
            }
        }
    };
    gen.into()
}
