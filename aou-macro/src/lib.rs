#[proc_macro]
pub fn add_days(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let n = syn::parse_macro_input!(input as syn::LitInt);
    let n_value = n.base10_parse::<u8>().unwrap();

    let mut modules: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut uses: Vec<proc_macro2::TokenStream> = Vec::new();
    let mut map: Vec<proc_macro2::TokenStream> = Vec::new();
    for i in 1..=n_value {
        let day: syn::Ident = quote::format_ident!("day{:02}", i);
        let method: syn::Ident = quote::format_ident!("Day{:02}", i);
        modules.push(quote::quote! {
            mod #day;
        });
        uses.push(quote::quote! {
            use #day::#method;
        });
        map.push(quote::quote! {
            solutions.insert(#i, Box::new(#method));
        })
    }

    let expanded = quote::quote! {
        #(#modules)*

        #(#uses)*

        fn internal_create_solutions() -> std::collections::HashMap<u8, Box<dyn advent_of_utils::Solution>> {
            let mut solutions: std::collections::HashMap<u8, Box<dyn advent_of_utils::Solution>> = std::collections::HashMap::new();
            #(#map)*

            solutions
        }

        #[repr(C)]
        pub struct SolutionsContainer {
            solutions: *mut std::collections::HashMap<u8, Box<dyn advent_of_utils::Solution>>,
        }

        #[no_mangle]
        pub extern "C" fn create_solutions() -> *mut SolutionsContainer {
            let solutions = Box::new(internal_create_solutions());
            let container = Box::new(SolutionsContainer {
                solutions: Box::into_raw(solutions),
            });

            Box::into_raw(container)
        }

        #[no_mangle]
        pub unsafe extern "C" fn destroy_solutions(container: *mut SolutionsContainer) {
            if !container.is_null() {
                let container = Box::from_raw(container);
                if !container.solutions.is_null() {
                    let _ = Box::from_raw(container.solutions);
                }
            }
        }

    };

    proc_macro::TokenStream::from(expanded)
}
