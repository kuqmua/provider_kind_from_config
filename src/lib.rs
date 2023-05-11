#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(ProviderKindFromConfig)]
pub fn provider_kind_from_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro_helpers::panic_location::panic_location("ProviderKindFromConfig");
    let ast: syn::DeriveInput =
        syn::parse(input).expect("ProviderKindFromConfig syn::parse(input) failed"); //if need to print ast use syn = { version = "1.0.75", features = ["extra-traits"]} instead of syn="1.0.75"
    let ident: &syn::Ident = &ast.ident;
    let data: syn::Data = ast.data;
    let function_vec_idents: Vec<(syn::Ident, syn::ReturnType)>;
    let trait_handle = quote::quote! {
        pub trait ProviderKindFromConfig<'a> {
            fn is_mongo_initialization_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsMongoInitializationEnabledArxiv
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledBiorxiv
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledGithub
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledHabr
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledMedrxiv
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledReddit
                    + crate::traits::config_fields::GetIsMongoInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_write_error_logs_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledArxiv
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledBiorxiv
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledGithub
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledHabr
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledMedrxiv
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledReddit
                    + crate::traits::config_fields::GetIsMongoWriteErrorLogsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_cleaning_warning_logs_db_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledArxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledBiorxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledGithub
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledHabr
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledMedrxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledReddit
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_cleaning_warning_logs_db_collections_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledArxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledBiorxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledGithub
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledHabr
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledMedrxiv
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledReddit
                    + crate::traits::config_fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_link_parts_randomize_order_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledArxiv
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledBiorxiv
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledGithub
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledHabr
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledMedrxiv
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledReddit
                    + crate::traits::config_fields::GetIsMongoLinkPartsRandomizeOrderEnabledTwitter
                )
            ) -> &'a bool;
            fn is_postgres_initialization_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsPostgresInitializationEnabledArxiv
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledBiorxiv
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledGithub
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledHabr
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledMedrxiv
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledReddit
                    + crate::traits::config_fields::GetIsPostgresInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_write_error_logs_in_local_folder_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledArxiv
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledBiorxiv
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledGithub
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledHabr
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledMedrxiv
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledReddit
                    + crate::traits::config_fields::GetIsWriteErrorLogsInLocalFolderEnabledTwitter
                )
            ) -> &'a bool;
            fn is_cleaning_warning_logs_directory_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledArxiv
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledBiorxiv
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledGithub
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledHabr
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledMedrxiv
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledReddit
                    + crate::traits::config_fields::GetIsCleaningWarningLogsDirectoryEnabledTwitter
                )
            ) -> &'a bool;
            fn check_link(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetCheckLinkArxiv
                    + crate::traits::config_fields::GetCheckLinkBiorxiv
                    + crate::traits::config_fields::GetCheckLinkGithub
                    + crate::traits::config_fields::GetCheckLinkHabr
                    + crate::traits::config_fields::GetCheckLinkMedrxiv
                    + crate::traits::config_fields::GetCheckLinkReddit
                    + crate::traits::config_fields::GetCheckLinkTwitter
                )
            ) -> &'a String;
            fn is_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsEnabledArxiv
                    + crate::traits::config_fields::GetIsEnabledBiorxiv
                    + crate::traits::config_fields::GetIsEnabledGithub
                    + crate::traits::config_fields::GetIsEnabledHabr
                    + crate::traits::config_fields::GetIsEnabledMedrxiv
                    + crate::traits::config_fields::GetIsEnabledReddit
                    + crate::traits::config_fields::GetIsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_dbs_initialization_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsDbsInitializationEnabledArxiv
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledBiorxiv
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledGithub
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledHabr
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledMedrxiv
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledReddit
                    + crate::traits::config_fields::GetIsDbsInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_links_limit_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetIsLinksLimitEnabledArxiv
                    + crate::traits::config_fields::GetIsLinksLimitEnabledBiorxiv
                    + crate::traits::config_fields::GetIsLinksLimitEnabledGithub
                    + crate::traits::config_fields::GetIsLinksLimitEnabledHabr
                    + crate::traits::config_fields::GetIsLinksLimitEnabledMedrxiv
                    + crate::traits::config_fields::GetIsLinksLimitEnabledReddit
                    + crate::traits::config_fields::GetIsLinksLimitEnabledTwitter
                )
            ) -> &'a bool;
            fn links_limit(
                &'a self,
                config : &'a (
                    impl crate::traits::config_fields::GetLinksLimitArxiv
                    + crate::traits::config_fields::GetLinksLimitBiorxiv
                    + crate::traits::config_fields::GetLinksLimitGithub
                    + crate::traits::config_fields::GetLinksLimitHabr
                    + crate::traits::config_fields::GetLinksLimitMedrxiv
                    + crate::traits::config_fields::GetLinksLimitReddit
                    + crate::traits::config_fields::GetLinksLimitTwitter
                )
            ) -> &'a usize;
        }
    };
    let token_stream: proc_macro::TokenStream = trait_handle
        .to_string()
        .parse()
        .expect("cannot parse file into proc_macro::TokenStream");
    let trait_ast: syn::ItemTrait =
        syn::parse(token_stream).expect("cannot parse token_stream from file into syn::ItemTrait");
    let trait_name = trait_ast.ident;
    function_vec_idents = trait_ast
        .items
        .iter()
        .filter_map(|trait_item| match trait_item {
            syn::TraitItem::Method(trait_item_method) => Some((
                trait_item_method.sig.ident.clone(),
                trait_item_method.sig.output.clone(),
            )),
            _ => None,
        })
        .collect();
    let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = data {
        variants
    } else {
        panic!("not a valid data type for this proc macro");
    };
    let mut function_quote_vec_ident = Vec::with_capacity(function_vec_idents.len());
    for (function_name_ident, output) in function_vec_idents {
        // let mut is_str = false;
        // if let syn::ReturnType::Type(_, box_type) = &output {
        //     if let syn::Type::Reference(type_reference) = &**box_type {
        //         if let syn::Type::Path(reference_type_path) = &*type_reference.elem {
        //             for i in &reference_type_path.path.segments {
        //                 if i.ident == "str" {
        //                     is_str = true;
        //                 }
        //             }
        //         }
        //     }
        // }
        let variants_for_quote = variants.iter().map(|variant| {
            use convert_case::Casing;
            let variant_name = &variant.ident;
            let config_field_name = 
            format!(
                "get_{}_{}()",
                function_name_ident
                    .to_string()
                    .to_case(convert_case::Case::Snake)
                    .to_lowercase(),
                variant_name
                    .to_string()
                    .to_case(convert_case::Case::Snake)
                    .to_lowercase()
            )
            .parse::<proc_macro2::TokenStream>()
            .unwrap_or_else(|_| panic!("3"));
            quote::quote! {
                #ident::#variant_name => config.#config_field_name
            }
        });
        let variants_for_config_impl = {
            variants.iter().enumerate().map(|(index, variant)| {
                use convert_case::Casing;
                let variant_name = &variant.ident;
                let config_impl_name = &format!(
                    "Get{}{variant_name}",
                    function_name_ident
                    .to_string()
                    .to_case(convert_case::Case::Pascal),
                );
                match index {
                    0 => format!("impl crate::traits::config_fields::{config_impl_name}")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("1")),
                    _ => format!("+ crate::traits::config_fields::{config_impl_name}")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("2")),
                }
            })
        };
        function_quote_vec_ident.push(quote::quote! {
            fn #function_name_ident(
                &'a self,
                config: &'a (
                    #(#variants_for_config_impl)*
                )
            ) #output {
                match self {
                   #(#variants_for_quote,)*
                }
            }
        });
    }
    let generated = quote::quote! {
        #trait_handle
        impl<'a> #trait_name<'a> for #ident {
            #(#function_quote_vec_ident)*
        }
    };
    // println!("{generated}");
    generated.into()
}
