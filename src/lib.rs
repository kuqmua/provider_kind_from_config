#![deny(
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used,
    clippy::float_arithmetic
)]
#![allow(clippy::too_many_arguments)]

#[proc_macro_derive(ProviderKindFromConfig)]
pub fn derive_provider_kind_from_config(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
                    impl crate::traits::fields::GetIsMongoInitializationEnabledArxiv
                    + crate::traits::fields::GetIsMongoInitializationEnabledBiorxiv
                    + crate::traits::fields::GetIsMongoInitializationEnabledGithub
                    + crate::traits::fields::GetIsMongoInitializationEnabledHabr
                    + crate::traits::fields::GetIsMongoInitializationEnabledMedrxiv
                    + crate::traits::fields::GetIsMongoInitializationEnabledReddit
                    + crate::traits::fields::GetIsMongoInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_write_error_logs_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsMongoWriteErrorLogsEnabledArxiv
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledBiorxiv
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledGithub
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledHabr
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledMedrxiv
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledReddit
                    + crate::traits::fields::GetIsMongoWriteErrorLogsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_cleaning_warning_logs_db_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledArxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledBiorxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledGithub
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledHabr
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledMedrxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledReddit
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_cleaning_warning_logs_db_collections_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledArxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledBiorxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledGithub
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledHabr
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledMedrxiv
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledReddit
                    + crate::traits::fields::GetIsMongoCleaningWarningLogsDbCollectionsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_mongo_link_parts_randomize_order_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledArxiv
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledBiorxiv
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledGithub
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledHabr
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledMedrxiv
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledReddit
                    + crate::traits::fields::GetIsMongoLinkPartsRandomizeOrderEnabledTwitter
                )
            ) -> &'a bool;
            fn is_postgres_initialization_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsPostgresInitializationEnabledArxiv
                    + crate::traits::fields::GetIsPostgresInitializationEnabledBiorxiv
                    + crate::traits::fields::GetIsPostgresInitializationEnabledGithub
                    + crate::traits::fields::GetIsPostgresInitializationEnabledHabr
                    + crate::traits::fields::GetIsPostgresInitializationEnabledMedrxiv
                    + crate::traits::fields::GetIsPostgresInitializationEnabledReddit
                    + crate::traits::fields::GetIsPostgresInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_write_error_logs_in_local_folder_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledArxiv
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledBiorxiv
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledGithub
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledHabr
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledMedrxiv
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledReddit
                    + crate::traits::fields::GetIsWriteErrorLogsInLocalFolderEnabledTwitter
                )
            ) -> &'a bool;
            fn is_cleaning_warning_logs_directory_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledArxiv
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledBiorxiv
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledGithub
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledHabr
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledMedrxiv
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledReddit
                    + crate::traits::fields::GetIsCleaningWarningLogsDirectoryEnabledTwitter
                )
            ) -> &'a bool;
            fn check_link(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetCheckLinkArxiv
                    + crate::traits::fields::GetCheckLinkBiorxiv
                    + crate::traits::fields::GetCheckLinkGithub
                    + crate::traits::fields::GetCheckLinkHabr
                    + crate::traits::fields::GetCheckLinkMedrxiv
                    + crate::traits::fields::GetCheckLinkReddit
                    + crate::traits::fields::GetCheckLinkTwitter
                )
            ) -> &'a String;
            fn is_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsEnabledArxiv
                    + crate::traits::fields::GetIsEnabledBiorxiv
                    + crate::traits::fields::GetIsEnabledGithub
                    + crate::traits::fields::GetIsEnabledHabr
                    + crate::traits::fields::GetIsEnabledMedrxiv
                    + crate::traits::fields::GetIsEnabledReddit
                    + crate::traits::fields::GetIsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_dbs_initialization_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsDbsInitializationEnabledArxiv
                    + crate::traits::fields::GetIsDbsInitializationEnabledBiorxiv
                    + crate::traits::fields::GetIsDbsInitializationEnabledGithub
                    + crate::traits::fields::GetIsDbsInitializationEnabledHabr
                    + crate::traits::fields::GetIsDbsInitializationEnabledMedrxiv
                    + crate::traits::fields::GetIsDbsInitializationEnabledReddit
                    + crate::traits::fields::GetIsDbsInitializationEnabledTwitter
                )
            ) -> &'a bool;
            fn is_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsPrintsEnabledArxiv
                    + crate::traits::fields::GetIsPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsPrintsEnabledGithub
                    + crate::traits::fields::GetIsPrintsEnabledHabr
                    + crate::traits::fields::GetIsPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsPrintsEnabledReddit
                    + crate::traits::fields::GetIsPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_warning_high_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsWarningHighPrintsEnabledArxiv
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledGithub
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledHabr
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledReddit
                    + crate::traits::fields::GetIsWarningHighPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_warning_low_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsWarningLowPrintsEnabledArxiv
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledGithub
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledHabr
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledReddit
                    + crate::traits::fields::GetIsWarningLowPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_success_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsSuccessPrintsEnabledArxiv
                    + crate::traits::fields::GetIsSuccessPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsSuccessPrintsEnabledGithub
                    + crate::traits::fields::GetIsSuccessPrintsEnabledHabr
                    + crate::traits::fields::GetIsSuccessPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsSuccessPrintsEnabledReddit
                    + crate::traits::fields::GetIsSuccessPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_partial_success_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsPartialSuccessPrintsEnabledArxiv
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledGithub
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledHabr
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledReddit
                    + crate::traits::fields::GetIsPartialSuccessPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_error_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsErrorPrintsEnabledArxiv
                    + crate::traits::fields::GetIsErrorPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsErrorPrintsEnabledGithub
                    + crate::traits::fields::GetIsErrorPrintsEnabledHabr
                    + crate::traits::fields::GetIsErrorPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsErrorPrintsEnabledReddit
                    + crate::traits::fields::GetIsErrorPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_time_measurement_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsTimeMeasurementPrintsEnabledArxiv
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledGithub
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledHabr
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledReddit
                    + crate::traits::fields::GetIsTimeMeasurementPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_info_prints_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsInfoPrintsEnabledArxiv
                    + crate::traits::fields::GetIsInfoPrintsEnabledBiorxiv
                    + crate::traits::fields::GetIsInfoPrintsEnabledGithub
                    + crate::traits::fields::GetIsInfoPrintsEnabledHabr
                    + crate::traits::fields::GetIsInfoPrintsEnabledMedrxiv
                    + crate::traits::fields::GetIsInfoPrintsEnabledReddit
                    + crate::traits::fields::GetIsInfoPrintsEnabledTwitter
                )
            ) -> &'a bool;
            fn is_links_limit_enabled(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetIsLinksLimitEnabledArxiv
                    + crate::traits::fields::GetIsLinksLimitEnabledBiorxiv
                    + crate::traits::fields::GetIsLinksLimitEnabledGithub
                    + crate::traits::fields::GetIsLinksLimitEnabledHabr
                    + crate::traits::fields::GetIsLinksLimitEnabledMedrxiv
                    + crate::traits::fields::GetIsLinksLimitEnabledReddit
                    + crate::traits::fields::GetIsLinksLimitEnabledTwitter
                )
            ) -> &'a bool;
            fn links_limit(
                &'a self,
                config : &'a (
                    impl crate::traits::fields::GetLinksLimitArxiv
                    + crate::traits::fields::GetLinksLimitBiorxiv
                    + crate::traits::fields::GetLinksLimitGithub
                    + crate::traits::fields::GetLinksLimitHabr
                    + crate::traits::fields::GetLinksLimitMedrxiv
                    + crate::traits::fields::GetLinksLimitReddit
                    + crate::traits::fields::GetLinksLimitTwitter
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
        let mut is_str = false;
        if let syn::ReturnType::Type(_, box_type) = &output {
            if let syn::Type::Reference(type_reference) = &**box_type {
                if let syn::Type::Path(reference_type_path) = &*type_reference.elem {
                    for i in &reference_type_path.path.segments {
                        if i.ident == "str" {
                            is_str = true;
                        }
                    }
                }
            }
        }
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
                    0 => format!("impl crate::traits::fields::{config_impl_name}")
                    .parse::<proc_macro2::TokenStream>()
                    .unwrap_or_else(|_| panic!("1")),
                    _ => format!("+ crate::traits::fields::{config_impl_name}")
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
