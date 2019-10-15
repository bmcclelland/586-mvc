extern crate proc_macro;
use proc_macro::*;

fn parse_ident(ts: TokenStream) -> String {
    let ident: syn::Ident = syn::parse(ts).unwrap();
    return format!("{}", ident).into();
}

fn types_for(mod_name: &str) -> (String,String) {
    let pascal = pascal_case(mod_name);
    return (
        format!("{}Action", pascal), 
        format!("{}Params", pascal)
    );
}

fn pascal_case(s: &str) -> String {
    let mut t : String = "".into();
    let mut iter = s.chars();

    let mut caps = true;
    while let Some(c) = iter.next() {
        if c == '_' {
            caps = true;
        }
        else if caps {
            let d: String = c.to_uppercase().collect();
            t.push_str(&d);
            caps = false;
        }
        else {
            t.push(c);
        }
    }

    return t;
}

#[proc_macro]
pub fn action_types(ts: TokenStream) -> TokenStream {
    let mod_name = parse_ident(ts);
    let (action, params) = types_for(&mod_name);
    let out = format!( "pub struct {}(pub {});", action, params);
    out.parse().unwrap()
}

#[proc_macro]
pub fn action_func(ts: TokenStream) -> TokenStream {
    let mod_name = parse_ident(ts);
    let (action, params) = types_for(&mod_name);
    let out = format!(
        "pub fn {name}(request: &Request) -> Option<Box<dyn Action>> {{\
            let json: {params} = try_json(request)?;\
            return Some(Box::new({action}(json)));\
            }}",
        name=mod_name, action=action, params=params
    );
    out.parse().unwrap()
}
