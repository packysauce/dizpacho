#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::GenericArgument;
use syn::{
    parenthesized, parse2, Binding, Error, Expr, ExprCall, FnArg, Generics, ImplItemType, ItemImpl,
    Path, PathArguments, Result, Signature, Token, Type, WhereClause,
};

// (could) looks like #[dizpacho(std::convert::From<Self>::default<'a>)]
#[derive(Debug)]
struct DizpachoAttrs {
    impl_gen: Option<Generics>,
    path: Path,
    target: Option<Type>,
    where_clause: Option<WhereClause>,
}

impl Parse for DizpachoAttrs {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        // dizpacho( <we are now here> )
        let impl_gen = content.parse().ok();
        let path = content.parse()?;
        let target = content
            .parse::<Token![for]>()
            .and_then(|_| content.parse::<Type>())
            .ok();
        let where_clause = content.parse::<WhereClause>().ok();
        Ok(Self {
            impl_gen,
            path,
            target,
            where_clause,
        })
    }
}

fn dizpacho_impl(_attr: TokenStream, mut imp: ItemImpl) -> syn::Result<TokenStream2> {
    let self_ty: Type = *imp.self_ty.clone();

    // find all the methods with delegate attributes
    // remove the attribute and convert it to a delegate path
    let mut links = Vec::new();
    for item in &mut imp.items {
        // only handle methods with a delegate attribute
        // it's a method...
        if let syn::ImplItem::Method(item) = item {
            // partition the attributes into delegate and other attributes
            let (delegates, not) = item
                .attrs
                .iter()
                .cloned()
                .partition::<Vec<_>, _>(|attr| attr.path.is_ident("dizpacho"));
            let delegates: Vec<DizpachoAttrs> = delegates
                .into_iter()
                .map(|a| syn::parse::<DizpachoAttrs>(a.tokens.into()))
                .collect::<Result<Vec<_>>>()?;
            item.attrs = not;
            links.push((item.sig.clone(), delegates))
        }
    }

    let mut output = imp.to_token_stream();

    for (sig, delegates) in links {
        for attrs in delegates {
            let ret = dizpacho_method(Some(self_ty.clone()), attrs, sig.clone())?;

            ret.to_tokens(&mut output)
        }
    }
    Ok(output)
}

/// Handles fn-like things (so method or associated function) within an impl
/// ex:
/// ```rust
/// use dizpacho::dizpacho;
///
/// pub struct MyStruct(usize);
///
/// #[dizpacho]
/// impl MyStruct {
/// #[dizpacho(std::ops::Deref<Target=usize>::deref)]
/// fn make_thingy(&self) -> &usize { &self.0 }
/// # }
/// ```
fn dizpacho_method(
    self_ty: Option<Type>,
    attrs: DizpachoAttrs,
    mut sig: Signature,
) -> Result<TokenStream2> {
    let DizpachoAttrs {
        impl_gen,
        mut path,
        target,
        where_clause,
    } = attrs;
    let target = target.or_else(|| self_ty.clone());
    let self_ty = self_ty.ok_or_else(|| {
        syn::Error::new_spanned(
            sig.to_token_stream(),
            "The impl containing this function also needs to the dizpacho trait",
        )
    })?;
    let method_name = sig.ident.clone();
    // copy the delegate's fn sig but replace the name and generics

    // last segment is the function segment
    let fn_segment = path.segments.pop().unwrap().into_tuple().0;

    let mut generics: Vec<GenericArgument> = vec![];
    let mut assoc_types: Vec<ImplItemType> = vec![];

    // time to mangle the trait's generics!
    // steal the bindings for the impl items
    // and leave only the single names, we'll need them for the impl keyword
    let mut trait_segment = path.segments.pop().unwrap().into_value();
    sig.ident = fn_segment.ident;
    if let PathArguments::AngleBracketed(ref mut bracketed) = &mut trait_segment.arguments {
        let mut local_self = None;
        while let Some(mut pair) = bracketed.args.pop() {
            let arg = pair.value_mut();
            match arg {
                GenericArgument::Type(ref mut ty) => {
                    // We don't actually have Self right here, so let's be helpful and do a sneak
                    if *ty == syn::parse(quote!(Self).into())? || ty == &self_ty {
                        let new_type: GenericArgument = parse2(quote!(#self_ty))?;
                        local_self = Some(new_type.clone());
                        *ty = self_ty.clone();
                    } else {
                        generics.push(parse2(quote!(#ty))?);
                    }
                }
                GenericArgument::Binding(Binding {
                    ref ident, ref ty, ..
                }) => {
                    assoc_types.push(parse2(quote!(type #ident = #ty;))?);
                }
                GenericArgument::Lifetime(_) => generics.push(arg.clone()),
                // ignore the rest
                _ => {}
            }
        }
        if generics.is_empty() {
            if let Some(s) = local_self {
                bracketed.args.push(s);
            } else {
                trait_segment.arguments = PathArguments::None;
            }
            //dbg!(&trait_segment);
        } else {
            bracketed.args = generics.iter().cloned().collect();
        }
    }
    path.segments.push(trait_segment);
    let trait_ty = &path;
    //dbg!(&trait_ty);

    let call_args = sig
        .inputs
        .iter_mut()
        .map::<Result<Expr>, _>(|arg| match arg {
            FnArg::Receiver(_rec) => parse2(quote!(self)),
            FnArg::Typed(ref mut arg) => {
                if *arg.ty == syn::parse(quote!(Self).into())? {
                    *arg.ty = self_ty.clone();
                }
                let ident = arg.pat.to_token_stream();
                parse2(ident)
            }
        })
        .collect::<Result<Punctuated<_, Comma>>>()?;

    let toks = quote::quote!(<#self_ty>::#method_name(#call_args));

    let the_call: ExprCall = syn::parse(quote_spanned!(toks.span()=> #toks).into())?;

    let assoc_types = assoc_types
        .into_iter()
        .map(|t| t.to_token_stream())
        .collect::<TokenStream2>();

    let tokens = quote! {
        impl #impl_gen #trait_ty for #target #where_clause {
            #assoc_types

            #sig {
                #the_call
            }
        }
    }
    .into();
    //dbg!(&tokens);
    let item: ItemImpl = syn::parse(tokens)?;
    Ok(item.to_token_stream())
}

#[proc_macro_attribute]
pub fn dizpacho(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = &item;
    let result = match syn::parse::<ItemImpl>(item.clone()) {
        Ok(imp) => dizpacho_impl(attr, imp),
        Err(e) => {
            return Error::new(
                e.span(),
                "unsupported item! impl blocks and impl fns both need the attribute",
            )
            .to_compile_error()
            .into()
        }
    };
    match result {
        Ok(thing) => thing.to_token_stream().into(),
        Err(e) => e.to_compile_error().into(),
    }
}
