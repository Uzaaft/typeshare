use joinery::JoinableIterator;

use crate::rust_types::{RustType, RustTypeFormatError, SpecialRustType};
use crate::{
    language::{Language, SupportedLanguage},
    rust_types::{RustEnum, RustEnumVariant, RustField, RustStruct, RustTypeAlias},
};
use std::{collections::HashMap, io::Write};

/// All information needed to generate Typescript type-code
#[derive(Default)]
pub struct TypeScript {
    /// Mappings from Rust type names to Typescript type names
    pub type_mappings: HashMap<String, String>,
    /// Whether or not to exclude the version header that normally appears at the top of generated code.
    /// If you aren't generating a snapshot test, this setting can just be left as a default (false)
    pub no_version_header: bool,
}

impl Language for TypeScript {
    fn type_map(&mut self) -> &HashMap<String, String> {
        &self.type_mappings
    }

    fn format_special_type(
        &mut self,
        special_ty: &SpecialRustType,
        generic_types: &[String],
    ) -> Result<String, RustTypeFormatError> {
        match special_ty {
            SpecialRustType::Vec(rtype) => {
                Ok(format!("{}[]", self.format_type(rtype, generic_types)?))
            }
            SpecialRustType::Array(rtype, len) => {
                let formatted_type = self.format_type(rtype, generic_types)?;
                Ok(format!(
                    "[{}]",
                    std::iter::repeat(&formatted_type)
                        .take(*len)
                        .join_with(", ")
                ))
            }
            // We add optionality above the type formatting level
            SpecialRustType::Option(rtype) => self.format_type(rtype, generic_types),
            SpecialRustType::HashMap(rtype1, rtype2) => Ok(format!(
                "Record<{}, {}>",
                match rtype1.as_ref() {
                    RustType::Simple { id } if generic_types.contains(id) => {
                        return Err(RustTypeFormatError::GenericKeyForbiddenInTS(id.clone()));
                    }
                    _ => self.format_type(rtype1, generic_types)?,
                },
                self.format_type(rtype2, generic_types)?
            )),
            SpecialRustType::Unit => Ok("undefined".into()),
            SpecialRustType::String => Ok("string".into()),
            SpecialRustType::I8
            | SpecialRustType::U8
            | SpecialRustType::I16
            | SpecialRustType::U16
            | SpecialRustType::I32
            | SpecialRustType::U32
            | SpecialRustType::I54
            | SpecialRustType::U53
            | SpecialRustType::F32
            | SpecialRustType::F64 => Ok("number".into()),
            SpecialRustType::Bool => Ok("boolean".into()),
            SpecialRustType::U64
            | SpecialRustType::I64
            | SpecialRustType::ISize
            | SpecialRustType::USize => Ok("bigint".into()),
        }
    }

    fn begin_file(&mut self, w: &mut dyn Write) -> std::io::Result<()> {
        if !self.no_version_header {
            writeln!(w, "/*")?;
            writeln!(w, " Generated by typeshare {}", env!("CARGO_PKG_VERSION"))?;
            writeln!(w, "*/")?;
            writeln!(w)?;
        }
        Ok(())
    }

    fn write_type_alias(&mut self, w: &mut dyn Write, ty: &RustTypeAlias) -> std::io::Result<()> {
        self.write_comments(w, 0, &ty.comments)?;

        let r#type = self
            .format_type(&ty.r#type, ty.generic_types.as_slice())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        writeln!(
            w,
            "export type {}{} = {}{};\n",
            ty.id.renamed,
            (!ty.generic_types.is_empty())
                .then(|| format!("<{}>", ty.generic_types.join(", ")))
                .unwrap_or_default(),
            r#type,
            ty.r#type
                .is_optional()
                .then(|| " | undefined")
                .unwrap_or_default(),
        )?;

        Ok(())
    }

    fn write_struct(&mut self, w: &mut dyn Write, rs: &RustStruct) -> std::io::Result<()> {
        self.write_comments(w, 0, &rs.comments)?;
        writeln!(
            w,
            "export interface {}{} {{",
            rs.id.renamed,
            (!rs.generic_types.is_empty())
                .then(|| format!("<{}>", rs.generic_types.join(", ")))
                .unwrap_or_default()
        )?;

        rs.fields
            .iter()
            .try_for_each(|f| self.write_field(w, f, rs.generic_types.as_slice()))?;

        writeln!(w, "}}\n")
    }

    fn write_enum(&mut self, w: &mut dyn Write, e: &RustEnum) -> std::io::Result<()> {
        self.write_comments(w, 0, &e.shared().comments)?;

        let generic_parameters = (!e.shared().generic_types.is_empty())
            .then(|| format!("<{}>", e.shared().generic_types.join(", ")))
            .unwrap_or_default();

        match e {
            RustEnum::Unit(shared) => {
                write!(
                    w,
                    "export enum {}{} {{",
                    shared.id.renamed, generic_parameters
                )?;

                self.write_enum_variants(w, e)?;

                writeln!(w, "\n}}\n")
            }
            RustEnum::Algebraic { shared, .. } => {
                write!(
                    w,
                    "export type {}{} = ",
                    shared.id.renamed, generic_parameters
                )?;

                self.write_enum_variants(w, e)?;

                write!(w, ";")?;
                writeln!(w)?;
                writeln!(w)
            }
        }
    }
}

impl TypeScript {
    fn write_enum_variants(&mut self, w: &mut dyn Write, e: &RustEnum) -> std::io::Result<()> {
        match e {
            // Write all the unit variants out (there can only be unit variants in
            // this case)
            RustEnum::Unit(shared) => shared.variants.iter().try_for_each(|v| match v {
                RustEnumVariant::Unit(shared) => {
                    writeln!(w)?;
                    self.write_comments(w, 1, &shared.comments)?;
                    write!(w, "\t{} = {:?},", shared.id.original, &shared.id.renamed)
                }
                _ => unreachable!(),
            }),

            // Write all the algebraic variants out (all three variant types are possible
            // here)
            RustEnum::Algebraic {
                tag_key,
                content_key,
                shared,
            } => shared.variants.iter().try_for_each(|v| {
                writeln!(w)?;
                self.write_comments(w, 1, &v.shared().comments)?;
                match v {
                    RustEnumVariant::Unit(shared) => write!(
                        w,
                        "\t| {{ {}: {:?}, {}?: undefined }}",
                        tag_key, shared.id.renamed, content_key
                    ),
                    RustEnumVariant::Tuple { ty, shared } => {
                        let r#type = self
                            .format_type(ty, e.shared().generic_types.as_slice())
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                        write!(
                            w,
                            "\t| {{ {}: {:?}, {}{}: {} }}",
                            tag_key,
                            shared.id.renamed,
                            content_key,
                            ty.is_optional().then(|| "?").unwrap_or_default(),
                            r#type
                        )
                    }
                    RustEnumVariant::AnonymousStruct { fields, shared } => {
                        writeln!(
                            w,
                            "\t| {{ {}: {:?}, {}: {{",
                            tag_key, shared.id.renamed, content_key
                        )?;

                        fields.iter().try_for_each(|f| {
                            self.write_field(w, f, e.shared().generic_types.as_slice())
                        })?;

                        write!(w, "}}")?;
                        write!(w, "}}")
                    }
                }
            }),
        }
    }

    fn write_field(
        &mut self,
        w: &mut dyn Write,
        field: &RustField,
        generic_types: &[String],
    ) -> std::io::Result<()> {
        self.write_comments(w, 1, &field.comments)?;
        let ts_ty = self
            .format_type(&field.ty, generic_types)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        let optional = field.ty.is_optional() || field.has_default;
        let double_optional = field.ty.is_double_optional();
        let is_readonly = field
            .decorators
            .get(&SupportedLanguage::TypeScript)
            .filter(|v| v.contains(&"readonly".to_string()))
            .is_some();
        writeln!(
            w,
            "\t{}{}{}: {}{};",
            is_readonly.then(|| "readonly ").unwrap_or_default(),
            typescript_property_aware_rename(&field.id.renamed),
            optional.then(|| "?").unwrap_or_default(),
            ts_ty,
            double_optional.then(|| " | null").unwrap_or_default()
        )?;

        Ok(())
    }

    fn write_comments(
        &mut self,
        w: &mut dyn Write,
        indent: usize,
        comments: &[String],
    ) -> std::io::Result<()> {
        // Only attempt to write a comment if there are some, otherwise we're Ok()
        if !comments.is_empty() {
            let comment: String = {
                let tab_indent = "\t".repeat(indent);
                // If there's only one comment then keep it on the same line, otherwise we'll make a nice multi-line comment
                if comments.len() == 1 {
                    format!("{}/** {} */", tab_indent, comments.first().unwrap())
                } else {
                    let joined_comments = comments.join(&format!("\n{} * ", tab_indent));
                    format!(
                        "{tab}/**
{tab} * {comment}
{tab} */",
                        tab = tab_indent,
                        comment = joined_comments
                    )
                }
            };
            writeln!(w, "{}", comment)?;
        }
        Ok(())
    }
}

fn typescript_property_aware_rename(name: &str) -> String {
    if name.chars().any(|c| c == '-') {
        return format!("{:?}", name);
    }
    name.to_string()
}
