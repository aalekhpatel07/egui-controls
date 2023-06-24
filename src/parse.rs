use proc_macro2;

use proc_macro::{TokenStream, Span};
use proc_macro2::{TokenStream as TokenStream2};

use syn::{*, token::Struct, parse::Parse, parse::ParseStream};
use quote::{quote, ToTokens};

fn parse_doc_comments_from_fields(fields: &Fields) -> impl Iterator<Item = String> + '_ {
    fields
    .iter()
    .map(|field| {
        field
            .attrs
            .iter()
            .find_map(|attr| {
                if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
                    if path.segments.iter().any(|segment| segment.ident == "doc") {
                        // Some segment has "doc" so assume this is field has a doc comment.
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = value {
                            let mut raw_token = lit_str.token().to_string();
                            if let Some(stripped) = raw_token.strip_prefix('\"') {
                                raw_token = stripped.to_string();
                            }
                            if let Some(stripped) = raw_token.strip_suffix('\"') {
                                raw_token = stripped.to_string();
                            }
                            return Some(raw_token.trim().to_string());
                        }
                    }
                }
                None
            })
            .unwrap_or("No doc comment found".to_string())
    })
}

fn parse_widgets_from_fields(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {

    let field_type = fields.iter().map(|field| &field.ty);

    fields.iter().zip(field_type.into_iter())
        .map(|(field, ftype)| {
            let name = field.ident.clone().unwrap();
            match ftype {
                Type::Path(type_path)  => {
                    let full_type = type_path.clone().into_token_stream().to_string();
                    match full_type.as_str() {
                        | "u8"
                        | "u16"
                        | "u32"
                        | "u64"
                        | "u128"
                        | "usize"
                        | "i8"
                        | "i16"
                        | "i32"
                        | "i64"
                        | "i128"
                        | "isize"
                        => {
                            // FIXME: Change the hardcoded ranges to be configurable
                            //        via #[control(slider(min=0, max=50))]
                            quote!(
                                eframe::egui::Slider::new(&mut state.#name, 0..=100)
                            )
                        },
                        "f32" | "f64" => {
                            // FIXME: Change the hardcoded ranges to be configurable
                            //        via #[control(slider(min=0., max=2.0))]
                            quote!(
                                eframe::egui::Slider::new(&mut state.#name, 0.0..=1.0)
                            )
                        },
                        _ => {
                            // FIXME: How should we support other types?
                            // For example, datetimes could be converted to a date picker.
                            // Maybe allow a file upload marker as well.
                            quote!(
                                eframe::egui::TextEdit::singleline(&mut state.#name).hint_text("Unsupported type. Will treat it as string.")
                            )
                        }
                    }
                },
                _ => {
                    // We probably don't care about non-path types
                    // but might wanna handle the case.
                    quote!(
                        eframe::egui::TextEdit::singleline(&mut state.#name).hint_text("Unsupported type. Will treat it as string.")
                    )
                }
            }
        })
}


pub fn expand(input: DeriveInput) -> TokenStream {

    let Data::Struct(DataStruct { fields, .. }) = &input.data else {
        panic!("expected a struct");
    };

    let struct_name = &input.ident;
    let grid_id = format!("{}__control-panel", input.ident);

    let field_name = fields.iter().map(|field| &field.ident);
    let field_doc = parse_doc_comments_from_fields(fields);
    let field_widget = parse_widgets_from_fields(fields);

    let expanded = quote! {

        impl ::eframe::egui::Widget for #struct_name {
            fn ui(self, ui: &mut ::eframe::egui::Ui) -> ::eframe::egui::Response {

                let id = ui.id();
                let state = ui.memory_mut(|mem| {
                    mem.data
                    .get_persisted_mut_or_insert_with(id, || std::sync::Arc::new(std::sync::Mutex::new(#struct_name::default())))
                    .clone()
                });

                let mut state = 
                    state
                    .lock()
                    .unwrap();

                egui::Grid::new(#grid_id)
                .num_columns(3)
                .spacing([40., 20.])
                .striped(true)
                .show(ui, |ui| {
                    ui.heading("Setting");
                    ui.heading("Value");
                    ui.heading("Description");
                    
                    ui.end_row();
                    
                    #(
                        {
                            ui.label(stringify!(#field_name));
                            ui.add(#field_widget);
                            ui.vertical(|ui| {
                                ui.label(#field_doc);
                            });
                            ui.end_row();
                        }
                    )*
                })
                .response
            }
        }
    };
    expanded.into()
}