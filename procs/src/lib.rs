extern crate proc_macro;
use proc_macro::*;

fn parse_ident(ts: TokenStream) -> String {
    let ident: syn::Ident = syn::parse(ts).unwrap();
    format!("{}", ident)
}

fn types_for(mod_name: &str) -> (String,String) {
    let s = pascal_case(mod_name);
    (format!("{}Action", s), format!("{}Params", s))
}

fn pascal_case(s: &str) -> String {
    let mut t = String::default();
    let mut caps = true;

    for c in s.chars() {
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

    t
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
