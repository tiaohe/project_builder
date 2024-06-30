use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse::Parse, parse::ParseStream, ItemFn, LitStr, Result};

#[derive(Debug)]
enum Aop {
    Pre,
    Post,
    Around,
    All,
}

impl Aop {
    fn from_str(s: &str) -> Option<Aop> {
        match s {
            "Pre" => Some(Aop::Pre),
            "Post" => Some(Aop::Post),
            "Around" => Some(Aop::Around),
            "All" => Some(Aop::All),
            _ => None,
        }
    }
}

struct AopArgs {
    mode: String,
}

impl Parse for AopArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mode: LitStr = input.parse()?;
        Ok(AopArgs {
            mode: mode.value(),
        })
    }
}

#[proc_macro_attribute]
pub fn elapsed(attr: TokenStream, func: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AopArgs);
    let func = parse_macro_input!(func as ItemFn);

    let aop_mode = Aop::from_str(&args.mode).unwrap_or(Aop::All);

    let func_vis = &func.vis;
    let func_block = &func.block;
    let func_decl = func.sig;
    let func_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let pre_execution = quote! {
        println!("Starting execution of {}", stringify!(#func_name));
        start = std::time::Instant::now();
    };

    let post_execution = quote! {
        println!("Finished execution of {}", stringify!(#func_name));
        println!("Time cost: {:?}", start.elapsed());
    };

    let around_execution_start = quote! {
        println!("Around start of {}", stringify!(#func_name));
    };

    let around_execution_end = quote! {
        println!("Around end of {}", stringify!(#func_name));
    };

    let caller = match aop_mode {
        Aop::Pre => quote! {
            #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
                let mut start = std::time::Instant::now();
                #pre_execution
                #func_block
            }
        },
        Aop::Post => quote! {
            #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
                let start = std::time::Instant::now();
                let result = (|| #func_block)();
                #post_execution
                result
            }
        },
        Aop::Around => quote! {
            #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
                #around_execution_start
                let result = (|| #func_block)();
                #around_execution_end
                result
            }
        },
        Aop::All => quote! {
            #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
                let mut start = std::time::Instant::now();
                #pre_execution
                #around_execution_start
                let result = (|| #func_block)();
                #around_execution_end
                #post_execution
                result
            }
        },
    };

    caller.into()
}