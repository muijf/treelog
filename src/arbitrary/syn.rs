//! Rust AST visualization using syn.

use crate::tree::Tree;
use std::path::Path;

impl Tree {
    /// Builds a tree from a Rust source file's AST.
    ///
    /// Requires the `syn` feature.
    ///
    /// Parses the Rust file and converts its AST into a tree structure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use treelog::Tree;
    ///
    /// let tree = Tree::from_syn_file("src/lib.rs").unwrap();
    /// ```
    #[cfg(feature = "arbitrary-syn")]
    pub fn from_syn_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let ast = syn::parse_file(&content)?;
        Ok(Self::from_syn_file_ast(&ast))
    }

    /// Builds a tree from a syn::File AST.
    ///
    /// Requires the `syn` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use syn::parse_file;
    ///
    /// let code = "fn main() {}";
    /// let ast = parse_file(code).unwrap();
    /// let tree = Tree::from_syn_file_ast(&ast);
    /// ```
    #[cfg(feature = "arbitrary-syn")]
    pub fn from_syn_file_ast(file: &syn::File) -> Self {
        let mut children = Vec::new();

        for item in &file.items {
            children.push(Self::from_syn_item(item));
        }

        if children.is_empty() {
            Tree::new_leaf("file (empty)".to_string())
        } else {
            Tree::Node("file".to_string(), children)
        }
    }

    /// Builds a tree from a syn::Item.
    ///
    /// Requires the `syn` feature.
    ///
    /// # Examples
    ///
    /// ```
    /// use treelog::Tree;
    /// use syn::parse_quote;
    ///
    /// let item: syn::Item = parse_quote! {
    ///     fn hello() {}
    /// };
    /// let tree = Tree::from_syn_item(&item);
    /// ```
    #[cfg(feature = "arbitrary-syn")]
    pub fn from_syn_item(item: &syn::Item) -> Self {
        match item {
            syn::Item::Const(item) => {
                let label = format!("const {}", item.ident);
                Tree::new_leaf(label)
            }
            syn::Item::Enum(item) => {
                let label = format!("enum {}", item.ident);
                let mut children = Vec::new();
                for variant in &item.variants {
                    let variant_label = format!("variant {}", variant.ident);
                    children.push(Tree::new_leaf(variant_label));
                }
                if children.is_empty() {
                    Tree::new_leaf(label)
                } else {
                    Tree::Node(label, children)
                }
            }
            syn::Item::ExternCrate(item) => Tree::new_leaf(format!("extern crate {}", item.ident)),
            syn::Item::Fn(item) => {
                let label = format!("fn {}", item.sig.ident);
                let mut children = Vec::new();

                // Add parameters
                if !item.sig.inputs.is_empty() {
                    let params: Vec<String> = item
                        .sig
                        .inputs
                        .iter()
                        .map(|input| match input {
                            syn::FnArg::Receiver(_) => "self".to_string(),
                            syn::FnArg::Typed(typed) => {
                                if let syn::Pat::Ident(ident) = &*typed.pat {
                                    ident.ident.to_string()
                                } else {
                                    "param".to_string()
                                }
                            }
                        })
                        .collect();
                    children.push(Tree::new_leaf(format!("params: {}", params.join(", "))));
                }

                if children.is_empty() {
                    Tree::new_leaf(label)
                } else {
                    Tree::Node(label, children)
                }
            }
            syn::Item::Impl(item) => {
                let label = if let Some((_, trait_path, _)) = &item.trait_ {
                    format!("impl {:?} for {:?}", trait_path, item.self_ty)
                } else {
                    format!("impl {:?}", item.self_ty)
                };
                let mut children = Vec::new();
                for impl_item in &item.items {
                    match impl_item {
                        syn::ImplItem::Fn(method) => {
                            children.push(Tree::new_leaf(format!("method {}", method.sig.ident)));
                        }
                        syn::ImplItem::Const(const_item) => {
                            children.push(Tree::new_leaf(format!("const {}", const_item.ident)));
                        }
                        _ => {
                            children.push(Tree::new_leaf("item".to_string()));
                        }
                    }
                }
                if children.is_empty() {
                    Tree::new_leaf(label)
                } else {
                    Tree::Node(label, children)
                }
            }
            syn::Item::Mod(item) => {
                let label = format!("mod {}", item.ident);
                Tree::new_leaf(label)
            }
            syn::Item::Static(item) => Tree::new_leaf(format!("static {}", item.ident)),
            syn::Item::Struct(item) => {
                let label = format!("struct {}", item.ident);
                let mut children = Vec::new();
                if let syn::Fields::Named(fields) = &item.fields {
                    for field in &fields.named {
                        let field_name = field
                            .ident
                            .as_ref()
                            .map(|i| i.to_string())
                            .unwrap_or_else(|| "unnamed".to_string());
                        children.push(Tree::new_leaf(field_name));
                    }
                }
                if children.is_empty() {
                    Tree::new_leaf(label)
                } else {
                    Tree::Node(label, children)
                }
            }
            syn::Item::Trait(item) => {
                let label = format!("trait {}", item.ident);
                let mut children = Vec::new();
                for item in &item.items {
                    if let syn::TraitItem::Fn(method) = item {
                        children.push(Tree::new_leaf(format!("method {}", method.sig.ident)));
                    }
                }
                if children.is_empty() {
                    Tree::new_leaf(label)
                } else {
                    Tree::Node(label, children)
                }
            }
            syn::Item::Type(item) => Tree::new_leaf(format!("type {}", item.ident)),
            syn::Item::Use(item) => Tree::new_leaf(format!("use {:?}", item)),
            _ => Tree::new_leaf(format!("item: {:?}", std::any::type_name_of_val(item))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "arbitrary-syn")]
    #[test]
    fn test_from_syn_file_ast() {
        let code = "fn main() {}";
        let ast = syn::parse_file(code).unwrap();
        let tree = Tree::from_syn_file_ast(&ast);
        assert!(tree.is_node());
    }

    #[cfg(feature = "arbitrary-syn")]
    #[test]
    fn test_from_syn_item() {
        let item: syn::Item = syn::parse_quote! {
            struct Test {
                field: i32,
            }
        };
        let tree = Tree::from_syn_item(&item);
        assert!(tree.is_node());
    }
}
