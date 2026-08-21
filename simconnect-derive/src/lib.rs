//! `#[derive(simconnect::DataDefinition)]` — see `simconnect-proto::
//! data_definition` for the trait it implements and `simconnect::client::
//! SimConnect::define_data` for the runtime side. Ported from the prior
//! C# client's reflection-based `DefineDataAsync<T>`
//! (`DataDefinition.cs`/`SimConnect.cs`), as compile-time codegen instead
//! of runtime `FieldInfo` walking, since Rust has no runtime reflection.
//!
//! Generated code always references items through `::simconnect::...`
//! paths (not `::simconnect_proto::...`) because `simconnect_proto` is
//! usually only a *transitive* dependency of whatever crate uses this
//! derive (via `simconnect`) — the extern prelude only contains a crate's
//! *direct* dependencies, so a transitive crate name isn't nameable that
//! way. This means using this derive requires `simconnect` (not just
//! `simconnect-proto`) as a direct dependency, which is already true of
//! every realistic user (that's where the derive itself is re-exported
//! from).

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, LitStr, Type};

/// `__suffix` → SimConnect `Units` string, covering the SDK's documented
/// "Units of Measurement" table. SimConnect unit strings have irregular
/// capitalization/spacing (`"Feet"`, `"Frequency BCD16"`) that isn't
/// mechanically derivable from a lowercase suffix, so each entry is
/// spelled out explicitly. Suffixes are restricted to valid Rust
/// identifier characters (`[a-z0-9_]`, no spaces), so multi-word units
/// get a single run-together alias (e.g. `feetpersecond`).
const UNIT_ALIASES: &[(&str, &str)] = &[
    // Frequency
    ("hz", "Hz"),
    ("khz", "kHz"),
    ("mhz", "MHz"),
    ("bco16", "BCO16"),
    ("frequencybcd16", "Frequency BCD16"),
    // Length
    ("meter", "Meter"),
    ("meters", "Meters"),
    ("kilometer", "Kilometer"),
    ("kilometers", "Kilometers"),
    ("centimeter", "Centimeter"),
    ("centimeters", "Centimeters"),
    ("millimeter", "Millimeter"),
    ("millimeters", "Millimeters"),
    ("foot", "Foot"),
    ("feet", "Feet"),
    ("inch", "Inch"),
    ("inches", "Inches"),
    ("yard", "Yard"),
    ("yards", "Yards"),
    ("mile", "Mile"),
    ("miles", "Miles"),
    ("nauticalmile", "Nautical Mile"),
    ("nauticalmiles", "Nautical Miles"),
    // Area
    ("squarefeet", "Square Feet"),
    ("squaremeters", "Square Meters"),
    // Volume
    ("gallon", "Gallon"),
    ("gallons", "Gallons"),
    ("liter", "Liter"),
    ("liters", "Liters"),
    ("quart", "Quart"),
    ("quarts", "Quarts"),
    // Temperature
    ("celsius", "Celsius"),
    ("rankine", "Rankine"),
    ("kelvin", "Kelvin"),
    ("fahrenheit", "Fahrenheit"),
    // Angle
    ("radian", "Radian"),
    ("radians", "Radians"),
    ("rounds", "Rounds"),
    ("degree", "Degree"),
    ("degrees", "Degrees"),
    ("grad", "Grad"),
    ("grads", "Grads"),
    // Angular velocity
    ("degreespersecond", "Degrees per second"),
    ("radianspersecond", "Radians per second"),
    ("roundsperminute", "Rounds per minute"),
    ("rpm", "RPM"),
    // Speed
    ("knot", "Knot"),
    ("knots", "Knots"),
    ("feetpersecond", "Feet per second"),
    ("feetperminute", "Feet per minute"),
    ("meterspersecond", "Meters per second"),
    ("kilometersperhour", "Kilometers per hour"),
    ("milesperhour", "Miles per hour"),
    ("mach", "Mach"),
    // Force
    ("pound", "Pound"),
    ("pounds", "Pounds"),
    ("newton", "Newton"),
    ("newtons", "Newtons"),
    ("poundforce", "Pound-force"),
    // Weight/mass
    ("kilogram", "Kilogram"),
    ("kilograms", "Kilograms"),
    ("slug", "Slug"),
    ("slugs", "Slugs"),
    ("poundsperhour", "Pounds per hour"),
    ("kilogramsperhour", "Kilograms per hour"),
    ("gallonsperhour", "Gallons per hour"),
    ("literperhour", "Liter per hour"),
    // Pressure
    ("pascal", "Pascal"),
    ("kilopascal", "Kilopascal"),
    ("hectopascal", "Hectopascal"),
    ("atm", "Atm"),
    ("mmhg", "mmHg"),
    ("inhg", "inHg"),
    ("psi", "Psi"),
    ("bar", "Bar"),
    ("bars", "Bars"),
    ("millibar", "Millibar"),
    ("millibars", "Millibars"),
    // Density
    ("kilogramspercubicmeter", "Kilograms per cubic meter"),
    ("slugspercubicfoot", "Slugs per cubic feet"),
    // Electrical
    ("ampere", "Ampere"),
    ("amperes", "Amperes"),
    ("volt", "Volt"),
    ("volts", "Volts"),
    // Time
    ("second", "Second"),
    ("seconds", "Seconds"),
    ("minute", "Minute"),
    ("minutes", "Minutes"),
    ("hour", "Hour"),
    ("hours", "Hours"),
    ("day", "Day"),
    ("days", "Days"),
    ("year", "Year"),
    ("years", "Years"),
    // Miscellaneous
    ("bool", "Bool"),
    ("enum", "Enum"),
    ("number", "Number"),
    ("percent", "Percent"),
    ("percentover100", "Percent Over 100"),
    ("position", "Position"),
    ("position16k", "Position 16k"),
    ("position32k", "Position 32k"),
    ("position128", "Position 128"),
];

#[proc_macro_derive(DataDefinition, attributes(simconnect))]
pub fn derive_data_definition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

struct FieldAttr {
    name: Option<String>,
    units: Option<String>,
    epsilon: Option<f32>,
}

fn parse_simconnect_attr(field: &Field) -> syn::Result<FieldAttr> {
    let mut out = FieldAttr {
        name: None,
        units: None,
        epsilon: None,
    };
    for attr in &field.attrs {
        if !attr.path().is_ident("simconnect") {
            continue;
        }
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("name") {
                let lit: LitStr = meta.value()?.parse()?;
                out.name = Some(lit.value());
            } else if meta.path.is_ident("units") {
                let lit: LitStr = meta.value()?.parse()?;
                out.units = Some(lit.value());
            } else if meta.path.is_ident("epsilon") {
                let lit: syn::Lit = meta.value()?.parse()?;
                out.epsilon = Some(match lit {
                    syn::Lit::Float(f) => f.base10_parse()?,
                    syn::Lit::Int(i) => i.base10_parse::<i64>()? as f32,
                    other => {
                        return Err(syn::Error::new_spanned(other, "epsilon must be a number"))
                    }
                });
            } else {
                return Err(
                    meta.error("unsupported #[simconnect(...)] key (expected name/units/epsilon)")
                );
            }
            Ok(())
        })?;
    }
    Ok(out)
}

fn last_segment_name(ty: &Type) -> syn::Result<String> {
    if let Type::Path(p) = ty {
        if let Some(seg) = p.path.segments.last() {
            return Ok(seg.ident.to_string());
        }
    }
    Err(syn::Error::new_spanned(
        ty,
        "unsupported field type for #[derive(DataDefinition)]",
    ))
}

/// Result of peeling trailing `__segment`s off a field name (see
/// [`parse_field_name`]).
struct ParsedFieldName {
    /// What's left after peeling off any recognized `__index`/`__unit`
    /// segments — the part `default_datum_name` uppercases and
    /// space-joins.
    base: String,
    /// A trailing all-digits `__123` segment, e.g. `com_active_frequency
    /// __1__mhz` -> `Some("1")` — SimConnect's `:index` convention
    /// (`"COM ACTIVE FREQUENCY:1"`) has no legal Rust-identifier spelling
    /// (identifiers can only contain `[a-zA-Z0-9_]`, no `:` or any other
    /// punctuation — `__` is about the only usable delimiter, which is why
    /// it's overloaded for both this and the unit suffix below), so it's
    /// spliced onto the default name with a colon instead.
    index: Option<String>,
    /// A trailing unit-alias `__mhz` segment's canonical units string.
    units: Option<&'static str>,
    /// The first trailing segment encountered that classified as neither
    /// of the above, if peeling stopped because of it (i.e. *nothing* was
    /// recognized) — passed back verbatim so the caller can decide
    /// whether it's an error (no explicit `units` given) or harmless (an
    /// explicit `units` attribute already resolved it).
    unrecognized: Option<String>,
}

/// Peels up to one `__<digits>` (index) and one `__<unit alias>` segment
/// off the end of `ident`, in either order (each classified by content,
/// not position — `foo__1__mhz` and `foo__mhz__1` are equivalent).
/// Stops at the first trailing segment that's neither (or a repeat of one
/// already found), leaving it and everything before it as `base`.
fn parse_field_name(ident: &str) -> ParsedFieldName {
    let mut rest = ident;
    let mut index = None;
    let mut units = None;
    let mut unrecognized = None;
    while let Some(pos) = rest.rfind("__") {
        let (head, seg) = (&rest[..pos], &rest[pos + 2..]);
        if index.is_none() && !seg.is_empty() && seg.bytes().all(|b| b.is_ascii_digit()) {
            index = Some(seg.to_string());
            rest = head;
            continue;
        }
        if units.is_none() {
            if let Some((_, u)) = UNIT_ALIASES
                .iter()
                .find(|(alias, _)| alias.eq_ignore_ascii_case(seg))
            {
                units = Some(*u);
                rest = head;
                continue;
            }
        }
        unrecognized = Some(seg.to_string());
        break;
    }
    // Only surface `unrecognized` when peeling found nothing at all —
    // once at least one segment is recognized, whatever's left (including
    // an unrecognized-looking trailing bit) is just part of the base name.
    if index.is_some() || units.is_some() {
        unrecognized = None;
    }
    ParsedFieldName {
        base: rest.to_string(),
        index,
        units,
        unrecognized,
    }
}

fn default_datum_name(base: &str, index: Option<&str>) -> String {
    let name = base.to_uppercase().replace('_', " ");
    match index {
        Some(i) => format!("{name}:{i}"),
        None => name,
    }
}

enum UnitsMode {
    Forbidden,
    Required(&'static str),
    Free,
}

struct TypeMapping {
    data_type: TokenStream2,
    encode_stmt: TokenStream2,
    decode_expr: TokenStream2,
    units_mode: UnitsMode,
}

fn map_type(type_name: &str, ident: &syn::Ident) -> syn::Result<TypeMapping> {
    macro_rules! scalar {
        ($data_type:ident, $write:ident, $read:ident) => {
            TypeMapping {
                data_type: quote!(::simconnect::proto::enums::DataType::$data_type),
                encode_stmt: quote!(buf.extend_from_slice(&self.#ident.$write().to_le_bytes());),
                decode_expr: quote!(r.$read()?),
                units_mode: UnitsMode::Free,
            }
        };
    }
    macro_rules! structural {
        ($data_type:ident, $path:path, fallible) => {
            TypeMapping {
                data_type: quote!(::simconnect::proto::enums::DataType::$data_type),
                encode_stmt: quote!(self.#ident.write_le(&mut buf)?;),
                decode_expr: quote!($path::read_le(&mut r)?),
                units_mode: UnitsMode::Forbidden,
            }
        };
        ($data_type:ident, $path:path, infallible) => {
            TypeMapping {
                data_type: quote!(::simconnect::proto::enums::DataType::$data_type),
                encode_stmt: quote!(self.#ident.write_le(&mut buf);),
                decode_expr: quote!($path::read_le(&mut r)?),
                units_mode: UnitsMode::Forbidden,
            }
        };
    }
    macro_rules! bcd {
        ($path:path, $units:literal) => {
            TypeMapping {
                data_type: quote!(::simconnect::proto::enums::DataType::Int32),
                encode_stmt: quote!(self.#ident.write_le(&mut buf);),
                decode_expr: quote!($path::read_le(&mut r)?),
                units_mode: UnitsMode::Required($units),
            }
        };
    }

    Ok(match type_name {
        "bool" => TypeMapping {
            data_type: quote!(::simconnect::proto::enums::DataType::Int32),
            encode_stmt: quote!(buf.extend_from_slice(&(self.#ident as u32).to_le_bytes());),
            decode_expr: quote!(r.bool32()?),
            units_mode: UnitsMode::Free,
        },
        "i32" => scalar!(Int32, clone, i32),
        "i64" => scalar!(Int64, clone, i64),
        "f32" => scalar!(Float32, clone, f32),
        "f64" => scalar!(Float64, clone, f64),
        "String8" => structural!(String8, ::simconnect::proto::strings::String8, fallible),
        "String32" => structural!(String32, ::simconnect::proto::strings::String32, fallible),
        "String64" => structural!(String64, ::simconnect::proto::strings::String64, fallible),
        "String128" => structural!(String128, ::simconnect::proto::strings::String128, fallible),
        "String256" => structural!(String256, ::simconnect::proto::strings::String256, fallible),
        "String260" => structural!(String260, ::simconnect::proto::strings::String260, fallible),
        "Waypoint" => structural!(Waypoint, ::simconnect::proto::data::Waypoint, infallible),
        "LatLonAlt" => structural!(LatLonAlt, ::simconnect::proto::data::LatLonAlt, infallible),
        "Xyz" => structural!(Xyz, ::simconnect::proto::data::Xyz, infallible),
        "MarkerState" => structural!(
            MarkerState,
            ::simconnect::proto::data::MarkerState,
            fallible
        ),
        "Bco16" => bcd!(::simconnect::proto::bcd::Bco16, "BCO16"),
        "FrequencyBcd16" => bcd!(::simconnect::proto::bcd::FrequencyBcd16, "Frequency BCD16"),
        other => {
            return Err(syn::Error::new_spanned(
                ident,
                format!(
                    "unsupported field type `{other}` for #[derive(DataDefinition)]; \
                     supported types: bool, i32, i64, f32, f64, String8..String260, \
                     Waypoint, LatLonAlt, Xyz, MarkerState, Bco16, FrequencyBcd16"
                ),
            ))
        }
    })
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input,
                    "#[derive(DataDefinition)] requires a struct with named fields",
                ))
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input,
                "#[derive(DataDefinition)] can only be derived for structs",
            ))
        }
    };

    let mut schema_entries = Vec::new();
    let mut decode_fields = Vec::new();
    let mut encode_stmts = Vec::new();

    for field in fields {
        let ident = field.ident.as_ref().expect("named field");
        let attr = parse_simconnect_attr(field)?;
        let parsed = parse_field_name(&ident.to_string());

        let type_name = last_segment_name(&field.ty)?;
        let mapping = map_type(&type_name, ident)?;

        let units = match (&attr.units, parsed.units) {
            (Some(u), _) => Some(u.clone()),
            (None, Some(u)) => Some(u.to_string()),
            (None, None) => {
                if type_name == "bool" {
                    Some("Bool".to_string())
                } else {
                    None
                }
            }
        };
        if attr.units.is_none() && units.is_none() {
            if let Some(suffix) = &parsed.unrecognized {
                return Err(syn::Error::new_spanned(
                    ident,
                    format!(
                        "unrecognized `__{suffix}` suffix; add #[simconnect(units = \"...\")] \
                         explicitly, or use a recognized unit suffix ({}) or a numeric `__index` \
                         suffix",
                        UNIT_ALIASES
                            .iter()
                            .map(|(a, _)| format!("__{a}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ));
            }
        }

        match mapping.units_mode {
            UnitsMode::Forbidden if units.is_some() => {
                return Err(syn::Error::new_spanned(
                    ident,
                    format!(
                        "field type `{type_name}` doesn't take a units override — it has no \
                         unit conversion, remove #[simconnect(units = ...)] (and any `__unit` \
                         suffix) from this field"
                    ),
                ))
            }
            UnitsMode::Required(expected) if units.as_deref() != Some(expected) => {
                return Err(syn::Error::new_spanned(
                    ident,
                    format!(
                        "field type `{type_name}` requires #[simconnect(units = \"{expected}\")]"
                    ),
                ))
            }
            _ => {}
        }

        let datum_name = attr
            .name
            .clone()
            .unwrap_or_else(|| default_datum_name(&parsed.base, parsed.index.as_deref()));
        let epsilon = attr.epsilon.unwrap_or(0.0);
        let units_tokens = match &units {
            Some(u) => quote!(::core::option::Option::Some(#u)),
            None => quote!(::core::option::Option::None),
        };
        let data_type = &mapping.data_type;

        schema_entries.push(quote! {
            ::simconnect::data_definition::FieldSpec {
                datum_name: #datum_name,
                units_name: #units_tokens,
                epsilon: #epsilon,
                data_type: #data_type,
            }
        });

        let decode_expr = &mapping.decode_expr;
        decode_fields.push(quote!(#ident: #decode_expr));

        let encode_stmt = &mapping.encode_stmt;
        encode_stmts.push(encode_stmt.clone());
    }

    Ok(quote! {
        #[automatically_derived]
        impl ::simconnect::DataDefinition for #struct_name {
            const SCHEMA: &'static [::simconnect::data_definition::FieldSpec] = &[
                #(#schema_entries),*
            ];

            fn decode(
                data: &[u8],
            ) -> ::core::result::Result<Self, ::simconnect::proto::codec::TooShort> {
                let mut r = ::simconnect::proto::codec::PacketReader::new(data);
                ::core::result::Result::Ok(Self {
                    #(#decode_fields,)*
                })
            }

            fn encode(
                &self,
            ) -> ::core::result::Result<
                ::std::vec::Vec<u8>,
                ::simconnect::proto::strings::FixedStringError,
            > {
                let mut buf = ::std::vec::Vec::new();
                #(#encode_stmts)*
                ::core::result::Result::Ok(buf)
            }
        }
    })
}
