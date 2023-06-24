use proc_macro::{TokenStream};
use proc_macro2::TokenStream as TokenStream2;

use quote::{quote, ToTokens};
use syn::{Fields, Meta, MetaNameValue, Expr, ExprLit, Lit, MetaList, DeriveInput, Data, DataStruct};

/// Parse struct fields into an iterator over the
/// doc comments of fields in the order of definition.
fn parse_doc_comments_from_fields(fields: &Fields) -> impl Iterator<Item = String> + '_ {
    fields
        .iter()
        .map(|field| {

        let mut doc_comments = vec![];

        // Every individual doc comment is an attr.
        field
            .attrs
            .iter()
            .for_each(|attr| {
                if let Meta::NameValue(MetaNameValue { path, value, .. }) = &attr.meta {
                    path
                    .segments
                    .iter()
                    .for_each(|segment| {
                        if segment.ident == "doc" {
                            if let Expr::Lit(ExprLit {
                                 lit: Lit::Str(lit_str),
                             .. }) = value {
                                let mut raw_token = lit_str.token().to_string();
                                if let Some(stripped) = raw_token.strip_prefix('\"') {
                                    raw_token = stripped.to_string();
                                }
                                if let Some(stripped) = raw_token.strip_suffix('\"') {
                                    raw_token = stripped.to_string();
                                }
                                // Collect every line of doc-comment.
                                doc_comments.push(raw_token.trim().to_string());
                            }
                        }
                    });
                }
            });

        if doc_comments.is_empty() {
            return "No doc comment found".to_string()
        }
        return doc_comments.join(" ");
    })
}

/// Parse fields for the widgets to generate from the `#[control]` field attributes.
fn parse_widgets_from_fields(fields: &Fields) -> impl Iterator<Item = TokenStream2> + '_ {

    fields
    .iter()
    .flat_map(|field| {
        let name = field.ident.clone().unwrap();
        field
        .attrs
        .iter()
        .filter_map(move |attr| {
            if let Meta::List(MetaList { path, tokens, .. }) = &attr.meta {
                if path.into_token_stream().to_string() == "control" {
                    let mut token_iter = tokens.clone().into_iter();
                    if let Some(proc_macro2::TokenTree::Ident(ident)) = token_iter.next() {
                        if ident == "slider" {
                            let proc_macro2::TokenTree::Group(group) =
                                token_iter.next().expect("slider to be provided a InclusiveRange prop") else {
                                panic!("slider expects an InclusiveRange prop.");
                            };
                            let stream = group.stream();
                            return Some(quote!(
                                ::eframe::egui::Slider::new(&mut self.#name, #stream)
                            ))
                        }
                        else if ident == "textbox" {
                            return Some(quote!(
                                ::eframe::egui::TextEdit::singleline(&mut self.#name).hint_text("")
                            ))
                        }
                        return None;
                    }
                }
            }
            None
        })
    })
}

/// Expand the parsed struct into a [eframe::egui::Grid] of three columns
/// where the first column is the struct field name, the second column
/// is the interactive form control, and the third field is the description
/// of the field extracted from the doc comment.
pub fn expand(input: DeriveInput) -> TokenStream {
    let Data::Struct(DataStruct { fields, .. }) = &input.data else {
        panic!("expected a struct");
    };

    let struct_name = &input.ident;
    let grid_id = format!("{}__control-panel", input.ident);

    let field_names = fields.iter().map(|field| &field.ident);
    let field_docs = parse_doc_comments_from_fields(fields);
    let field_widgets = parse_widgets_from_fields(fields);

    let setting_heading = "Setting";
    let value_heading = "Value";
    let description_heading = "Description";

    let expanded = quote! {

        impl #struct_name {
            fn ui(&mut self, ui: &mut ::eframe::egui::Ui) -> ::eframe::egui::Response {

                ::eframe::egui::Grid::new(#grid_id)
                .num_columns(3)
                .spacing([40., 20.])
                .striped(true)
                .show(ui, |ui| {
                    ui.heading(#setting_heading);
                    ui.heading(#value_heading);
                    ui.heading(#description_heading);
                    ui.end_row();
                    #(
                        {
                            ui.label(stringify!(#field_names));
                            ui.add(#field_widgets);
                            ui.vertical(|ui| ui.label(#field_docs));
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
