// lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, LitStr, Meta, NestedMeta, Pat};

// 定义 `get` 属性宏
#[proc_macro_attribute]
pub fn get(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析属性宏参数（路径和参数）
    let input_fn = parse_macro_input!(item as ItemFn);
    let attr_args = parse_macro_input!(attr as syn::AttributeArgs);

    let mut path = None;
    let mut params = Vec::new();

    for arg in attr_args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(meta)) if meta.path.is_ident("path") => {
                if let syn::Lit::Str(lit_str) = meta.lit {
                    path = Some(lit_str.value());
                }
            }
            NestedMeta::Meta(Meta::NameValue(meta)) if meta.path.is_ident("params") => {
                if let syn::Lit::Str(lit_str) = meta.lit {
                    // 假设参数是以 `key=value` 的形式传递
                    params = lit_str
                        .value()
                        .split(',')
                        .map(|pair| {
                            let mut parts = pair.split('=');
                            (parts.next().unwrap().to_string(), parts.next().unwrap().to_string())
                        })
                        .collect::<Vec<(String, String)>>();
                }
            }
            _ => {}
        }
    }

    // 生成代码
    let path_str = path.unwrap_or_else(|| "/".to_string());
    let params_tokens = params.iter().map(|(key, value)| {
        let key = LitStr::new(key, proc_macro2::Span::call_site());
        let value = LitStr::new(value, proc_macro2::Span::call_site());
        quote! {
            println!("{}: {}", #key, #value);
        }
    });

    let fn_name = &input_fn.sig.ident;

    // 生成最终代码
    let result = quote! {
        fn #fn_name() {
            // 打印路径
            println!("Requesting path: {}", #path_str);

            // 打印参数
            #(#params_tokens)*

            // 原始函数内容
            #input_fn
        }
    };

    result.into()
}

#[proc_macro_attribute]
pub fn loggable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);

    let func_name = &input.sig.ident;
    let func_block = &input.block;
    let func_inputs = &input.sig.inputs;
    let func_output = &input.sig.output;

    // 解析参数
    let mut log_time = false;
    let mut log_args = false;

    for arg in args {
        if let NestedMeta::Meta(Meta::NameValue(meta)) = arg {
            if let Some(ident) = meta.path.get_ident() {
                if ident == "time" {
                    if let Lit::Bool(lit_bool) = &meta.lit {
                        log_time = lit_bool.value;
                    }
                } else if ident == "args" {
                    if let Lit::Bool(lit_bool) = &meta.lit {
                        log_args = lit_bool.value;
                    }
                }
            }
        }
    }

    // 提取参数名称
    let arg_names: Vec<String> = func_inputs
        .iter()
        .filter_map(|arg| {
            if let syn::FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(ident) = &*pat_type.pat {
                    return Some(ident.ident.to_string());
                }
            }
            None
        })
        .collect();

    // 构造参数打印代码
    let log_args_code = if log_args {
        let arg_names_str = arg_names
            .iter()
            .map(|name| format!("{}={{:?}}", name))
            .collect::<Vec<_>>()
            .join(", ");
        let arg_vars = arg_names
            .iter()
            .map(|name| syn::Ident::new(name, proc_macro2::Span::call_site()));

        quote! {
            println!(concat!("Calling function: ", stringify!(#func_name), " with args: ", #arg_names_str), #(#arg_vars, )*);
        }
    } else {
        quote! {}
    };

    let log_time_code = if log_time {
        quote! {
            let start_time = std::time::Instant::now();
        }
    } else {
        quote! {}
    };

    let log_time_end_code = if log_time {
        quote! {
            let elapsed_time = start_time.elapsed();
            println!("Function {} executed in: {:?}", stringify!(#func_name), elapsed_time);
        }
    } else {
        quote! {}
    };

    // 生成最终代码
    let expanded = quote! {
        fn #func_name(#func_inputs) #func_output {
            #log_args_code
            #log_time_code

            let result = { #func_block };

            #log_time_end_code

            result
        }
    };

    TokenStream::from(expanded)
}
