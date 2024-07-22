use std::collections::HashMap;
use log::warn;
use memoize::memoize;
use gosub_css3::stylesheet::CssValue;

use crate::syntax::CssSyntax;
use crate::syntax_matcher::CssSyntaxTree;

/// A CSS property definition including its type and initial value and optional expanded values if it's a shorthand property
#[derive(Debug, Clone)]
pub struct PropertyDefinition {
    /// Name of the property (ie: color, background etc)
    name: String,
    /// List of expanded (computed) properties if this property is a shorthand property
    computed: Vec<String>,
    /// Syntax tree of the property. This is a tree that describes the valid values for this property.
    syntax: CssSyntaxTree,
    /// True when the property inherits from parent nodes if not set
    inherited: bool,
    /// Initial value of the property, if any
    initial_value: Option<CssValue>,
    /// URL to MDN documentation for this property
    mdn_url: String
}

#[derive(Debug, Clone)]
pub struct SyntaxDefinition {
    name: String,
    syntax: CssSyntaxTree,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    /// Name of the function
    name: String,
    /// Compiled syntax tree
    syntax: CssSyntaxTree,
    /// URL to MDN documentation for this function
    mdn_url: String
}

impl PropertyDefinition {
    pub fn name(self) -> String {
        self.name.clone()
    }

    pub fn expanded_properties(self) -> Vec<String> {
        self.computed.clone()
    }

    pub fn syntax(self) -> CssSyntaxTree {
        self.syntax
    }

    pub fn inherited(self) -> bool {
        self.inherited
    }

    pub fn has_initial_value(self) -> bool {
        self.initial_value.is_some()
    }

    pub fn initial_value(self) -> CssValue {
        self.initial_value.clone().unwrap_or(CssValue::None)
    }

    /// Matches a list of values against the current definition
    pub fn matches(self, value: &CssValue) -> Option<CssValue> {
        self.syntax.matches(value)
    }

    pub fn check_expanded_properties(&self, _values: &[CssValue]) -> bool {
        // if values.len() != self.expanded_properties.len() {
        //     return false;
        // }
        //
        // for (i, value) in values.iter().enumerate() {
        //     let prop = self.expanded_properties.get(i).unwrap();
        //     let prop_def = parse_definition_file().find(prop).unwrap();
        //     if !prop_def.matches(&vec![value.clone()]) {
        //         return false;
        //     }
        // }

        true
    }
}

#[derive(Debug, Clone)]
pub struct CssDefinitions {
    pub properties: HashMap<String, PropertyDefinition>,
    pub functions: HashMap<String, FunctionDefinition>,
    pub syntax: HashMap<String, SyntaxDefinition>,
}

impl CssDefinitions {
    pub fn new() -> Self {
        CssDefinitions {
            properties: HashMap::new(),
            functions: HashMap::new(),
            syntax: HashMap::new(),
        }
    }

    /// Load the CSS definitions resource files
    pub fn load() {
        parse_mdn_definition_files();
    }

    /// Add a new property definition
    pub fn add_property(&mut self, name: &str, property: PropertyDefinition) {
        self.properties.insert(name.to_string(), property);
    }

    /// Add a new syntax definition
    pub fn add_syntax(&mut self, name: &str, syntax: SyntaxDefinition) {
        self.syntax.insert(name.to_string(), syntax);
    }

    /// Add a new function definition
    pub fn add_function(&mut self, name: &str, function: FunctionDefinition) {
        self.functions.insert(name.to_string(), function);
    }

    /// Find a specific property
    pub fn find_property(&self, name: &str) -> Option<PropertyDefinition> {
        self.properties.get(name).cloned()
    }

    /// Returns the property definitions
    pub fn get_properties(self) -> HashMap<String, PropertyDefinition> {
        self.properties.clone()
    }

    /// Returns the length of the property definitions
    pub fn len(&self) -> usize {
        self.properties.len()
    }

    /// Returns true when the properties definitions are empty
    pub fn is_empty(&self) -> bool {
        self.properties.is_empty()
    }
}

//     // pub fn find_scalar(&self, name: &str) -> Option<SyntaxComponent> {
//     //     let scalars = vec![
//     //         "number",
//     //         "integer",
//     //         "percentage",
//     //         "dashed-ident",
//     //         "custom-ident",
//     //         "ident",
//     //         "repeat()",
//     //         "attr()",
//     //         "url",
//     //         "uri",
//     //         "named-color",
//     //         "system-color",
//     //         "unit()",
//     //         "string",
//     //         "tech()",
//     //         "length",
//     //         "reversed()",
//     //     ];
//     //
//     //     if scalars.contains(&name) {
//     //         return Some(SyntaxComponent::Scalar {
//     //             scalar: name.to_string(),
//     //             multiplier: SyntaxComponentMultiplier::Once,
//     //         });
//     //     }
//     //
//     //     None
//     // }
//
//     pub fn find(&self, name: &str) -> Option<CssPropertyTypeDef> {
//         let names = vec![
//             name.to_string(),
//             format!("<{}>", name),
//             format!("{}()", name),
//         ];
//
//         for name in names {
//             if let Some(typedef) = self.typedefs.get(&name) {
//                 return Some(typedef.clone());
//             }
//         }
//
//         None
//     }
//
//     pub fn len(&self) -> usize {
//         self.typedefs.len()
//     }
//
//     pub fn is_empty(&self) -> bool {
//         self.typedefs.is_empty()
//     }
//
//     pub fn get_typedefs(self) -> HashMap<String, CssPropertyTypeDef> {
//         self.typedefs.clone()
//     }
//
//     pub fn get_keys(self) -> Vec<String> {
//         let mut keys = vec![];
//         for (key, _) in self.typedefs.iter() {
//             keys.push(key.clone());
//         }
//         keys
//     }
// }

/// Parses the internal CSS definition file
#[memoize]
pub fn parse_mdn_definition_files() -> CssDefinitions {
    // First, parse all functions so we can use them in the properties and syntax
    let contents = include_str!("../resources/mdn_css_functions.json");
    let json: serde_json::Value =
        serde_json::from_str(contents).expect("JSON was not well-formatted");
    let functions = parse_mdn_functions_file(json);

    // parse all syntax so we can use them in the properties
    let contents = include_str!("../resources/mdn_css_syntax.json");
    let json: serde_json::Value =
        serde_json::from_str(contents).expect("JSON was not well-formatted");
    let syntax = parse_mdn_syntax_file(json);

    // Parse property definitions
    let contents = include_str!("../resources/mdn_css_properties.json");
    let json: serde_json::Value =
        serde_json::from_str(contents).expect("JSON was not well-formatted");
    let properties = parse_mdn_property_file(json);

    let definitions = CssDefinitions{
        properties,
        functions,
        syntax
    };

    /// Resolve all syntax and functions inside the definitions
    resolve_elements(&definitions)
}

/// Main function to return the definitions. THis will automatically load the definition files
/// and caches them if needed.
pub fn get_mdn_css_definitions() -> CssDefinitions {
    parse_mdn_definition_files()
}

/// Parses a function JSON import file
fn parse_mdn_functions_file(json: serde_json::Value) -> HashMap<String, FunctionDefinition> {
    let mut functions = HashMap::new();

    for obj in json.as_array().unwrap() {
        let syntax = obj.get("syntax").unwrap().as_str().unwrap();
        let syntax = CssSyntax::new(syntax).compile().expect("Could not compile syntax");
        functions.insert(
            obj.get("name").unwrap().to_string(),
            FunctionDefinition {
                name: obj.get("name").unwrap().clone().to_string(),
                syntax,
                mdn_url: obj.get("mdn_url").unwrap().to_string(),
            },
        );
    }

    functions
}

/// Parses a syntax JSON import file
fn parse_mdn_syntax_file(json: serde_json::Value) -> HashMap<String, SyntaxDefinition> {
    let mut syntaxes = HashMap::new();

    let entries = json.as_object().unwrap();
    for (name, entry) in entries.iter() {
        match CssSyntax::new(entry.as_str().unwrap()).compile() {
            Ok(ast) => {
                syntaxes.insert(
                    name.clone(),
                    SyntaxDefinition {
                        name: name.clone(),
                        syntax: ast.clone(),
                    },
                );
            }
            Err(e) => {
                log::warn!("Could not compile syntax for syntax {:?}: {:?}", name, e);
            }
        }
    }

    // Resolve all typedefs since we now have loaded them all
    // typedef_resolve_all(&mut typedefs);
    syntaxes
}

// /// Iterate all the typedefs and resolve any typedefs that are used in the syntax. After this call
// /// no more typedefs should exist in the syntax.
// fn typedef_resolve_all(typedefs: &mut CssPropertyTypeDefs) {
//     for name in <CssPropertyTypeDefs as Clone>::clone(typedefs).get_keys() {
//         typedef_resolve(typedefs, &name);
//     }
// }

// fn typedef_resolve_group(
//     typedefs: &mut CssPropertyTypeDefs,
//     components: &Vec<SyntaxComponent>,
// ) -> Vec<SyntaxComponent> {
//     let mut resolved_components = vec![];
//
//     for component in components {
//         match &component {
//             SyntaxComponent::TypeDefinition { definition, .. } => {
//                 // Is the type definition a scalar?
//                 if let Some(scalar) = typedefs.find_scalar(definition) {
//                     resolved_components.push(scalar);
//                     continue;
//                 }
//
//                 if let Some(_typedef) = typedefs.find(definition) {
//                     // Resolve the typedef (if it's not already resolved and will take care of recursive typedefs)
//                     typedef_resolve(typedefs, definition);
//                     resolved_components.push(
//                         typedefs
//                             .find(definition)
//                             .expect("Could not find typedef")
//                             .syntax
//                             .components[0]
//                             .clone(),
//                     );
//
//                     continue;
//                 }
//
//                 panic!(
//                     "Reference to typedef {:?} found. But it's not defined",
//                     definition
//                 );
//             }
//             SyntaxComponent::Group {
//                 components,
//                 combinator,
//                 multiplier,
//             } => {
//                 resolved_components.push(SyntaxComponent::Group {
//                     components: typedef_resolve_group(typedefs, components).clone(),
//                     combinator: combinator.clone(),
//                     multiplier: multiplier.clone(),
//                 });
//             }
//             _ => {
//                 resolved_components.push(component.clone());
//             }
//         }
//     }
//
//     resolved_components
// }

// fn typedef_resolve_syntaxtree(
//     typedefs: &mut CssPropertyTypeDefs,
//     syntax_tree: CssSyntaxTree,
// ) -> CssSyntaxTree {
//     let mut resolved_components = vec![];
//
//     for component in &syntax_tree.components {
//         // Resolve each component that needs resolving, or just return the component as-is.
//         // If a component is a group, we need to resolve all its components first.
//         match &component {
//             // Resolve type definition
//             SyntaxComponent::TypeDefinition { definition, .. } => {
//                 // Is the type definition a scalar?
//                 if let Some(scalar) = typedefs.find_scalar(definition) {
//                     resolved_components.push(scalar);
//
//                     continue;
//                 }
//
//                 if let Some(_typedef) = typedefs.find(definition) {
//                     // Resolve the typedef (if it's not already resolved and will take care of recursive typedefs)
//                     typedef_resolve(typedefs, definition);
//                     resolved_components.push(
//                         typedefs
//                             .find(definition)
//                             .expect("Could not find typedef")
//                             .syntax
//                             .components[0]
//                             .clone(),
//                     );
//
//                     continue;
//                 }
//
//                 panic!(
//                     "Reference to typedef {:?} found. But it's not defined",
//                     definition
//                 );
//             }
//             SyntaxComponent::Group {
//                 components,
//                 combinator,
//                 multiplier,
//             } => {
//                 let resolved_group = typedef_resolve_group(typedefs, components);
//
//                 resolved_components.push(SyntaxComponent::Group {
//                     combinator: combinator.clone(),
//                     components: resolved_group,
//                     multiplier: multiplier.clone(),
//                 });
//             }
//             _ => {
//                 // No need to resolve this component, just add it as-is
//                 resolved_components.push(component.clone());
//             }
//         }
//     }
//
//     CssSyntaxTree {
//         components: resolved_components,
//     }
// }
//
// /// Resolves a single typedef by recursively resolving all its components
// fn typedef_resolve(typedefs: &mut CssPropertyTypeDefs, name: &str) {
//     let mut typedef = typedefs.find(name).expect("Could not find typedef");
//     typedef.syntax = typedef_resolve_syntaxtree(typedefs, typedef.syntax);
//     typedefs.update_typedef(name, typedef);
// }

/// Parses the JSON input into a CSS property definitions structure
fn parse_mdn_property_file(json: serde_json::Value) -> HashMap<String, PropertyDefinition> {
    let mut properties = HashMap::new();

    for obj in json.as_array().unwrap() {
        let name = obj["name"].as_str().unwrap().to_string();

        // Compile syntax
        let syntax = obj.get("syntax").unwrap().as_str().unwrap();
        let syntax = CssSyntax::new(syntax).compile().expect(format!("Could not compile syntax: {:?}", syntax).as_str());

        //
        let computed = if obj["computed"].is_array() {
            obj["computed"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect()
        } else if obj["computed"].is_string() {
            vec![obj["computed"].as_str().unwrap().to_string()]
        } else {
            warn!("Computed property is not a string or array {:?}", obj);
            vec![]
        };

        let initial_value = if obj["initial_value"].is_array() {
            warn!("Initial value is an array, not supported {:?}", obj);
            // obj["initial_value"]
            //     .as_array()
            //     .unwrap()
            //     .iter()
            //     .map(|v| CssValue::from(v))
            //     .collect()
            None
        } else if obj["initial_value"].is_string() {
            match CssValue::parse_str(obj["initial_value"].as_str().unwrap()) {
                Ok(value) => Some(value),
                Err(e) => {
                    warn!("Could not parse initial value: {:?}", e);
                    None
                }
            }
        } else {
            warn!("Initial value is not a string or array {:?}", obj);
            None
        };

        properties.insert(
            name.clone(),
            PropertyDefinition {
                name: name.clone(),
                syntax,
                computed,
                initial_value,
                inherited: obj["inherited"].as_bool().unwrap(),
                mdn_url: obj["mdn_url"].as_str().unwrap().to_string(),
            },
        );
    }

    properties
}

#[cfg(test)]
mod tests {
    use super::*;
    use gosub_css3::colors::RgbColor;

    macro_rules! assert_none {
        ($e:expr) => {
            assert!($e.is_none());
        };
    }

    macro_rules! assert_some {
        ($e:expr) => {
            assert!($e.is_some());
        };
    }

    #[test]
    fn test_parse_definition_file() {
        let definitions = parse_mdn_definition_files();
        assert_eq!(definitions.len(), 563);
    }

    // #[test]
    // fn test_prop_border() {
    //     let definitions = parse_mdn_definition_files();
    //     let prop = definitions.find("border").unwrap();
    //
    //     // assert_some!(prop.clone().matches(&CssValue::List(vec![
    //     //     CssValue::Unit(1.0, "px".into()),
    //     //     CssValue::String("solid".into()),
    //     //     CssValue::Color(RgbColor::from("black")),
    //     // ])));
    //     //
    //     // assert_some!(prop.clone().matches(&CssValue::List(vec![
    //     //     CssValue::Color(RgbColor::from("black")),
    //     //     CssValue::String("solid".into()),
    //     //     CssValue::Unit(1.0, "px".into()),
    //     // ])));
    //     // assert_some!(prop.clone().matches(&CssValue::List(vec![
    //     //     CssValue::String("solid".into()),
    //     //     CssValue::Color(RgbColor::from("black")),
    //     //     CssValue::Unit(1.0, "px".into()),
    //     // ])));
    //     assert_some!(prop.clone().matches(&CssValue::Unit(1.0, "px".into())));
    //     assert_some!(prop.clone().matches(&CssValue::String("solid".into())));
    //     assert_some!(prop.clone().matches(&CssValue::List(vec![
    //         CssValue::String("solid".into()),
    //         CssValue::Color(RgbColor::from("black")),
    //     ])));
    //     assert_some!(prop.clone().matches(&CssValue::List(vec![
    //         CssValue::String("solid".into()),
    //         CssValue::Color(RgbColor::from("black")),
    //     ])));
    //     assert_some!(prop.clone().matches(&CssValue::String("solid".into())));
    //     assert_none!(prop.clone().matches(&CssValue::String("not-solid".into())));
    //     assert_none!(prop.clone().matches(&CssValue::List(vec![
    //         CssValue::String("solid".into()),
    //         CssValue::String("solid".into()),
    //         CssValue::Unit(1.0, "px".into()),
    //     ])));
    // }
    //
    // #[test]
    // fn test_property_definitions() {
    //     let mut definitions = CssPropertyDefinitions::empty();
    //     definitions.add_definition(
    //         "color",
    //         PropertyDefinition {
    //             name: "color".to_string(),
    //             computed: vec![],
    //             syntax: CssSyntax::new("color()")
    //                 .compile()
    //                 .expect("Could not compile syntax"),
    //             inherited: false,
    //             initial_value: None,
    //         },
    //     );
    //
    //     assert_eq!(definitions.len(), 1);
    //     assert!(definitions.find("color").is_some());
    //     assert!(definitions.find("border-top-style").is_none());
    //
    //     definitions.add_definition(
    //         "border-style",
    //         PropertyDefinition {
    //             name: "border-style".to_string(),
    //             computed: vec![
    //                 "border-top-style".to_string(),
    //                 "border-right-style".to_string(),
    //                 "border-bottom-style".to_string(),
    //                 "border-left-style".to_string(),
    //             ],
    //             syntax: CssSyntax::new("")
    //                 .compile()
    //                 .expect("Could not compile syntax"),
    //             inherited: false,
    //             initial_value: Some(CssValue::String("thick".to_string())),
    //         },
    //     );
    //
    //     assert_eq!(definitions.len(), 2);
    //     assert!(definitions.find("border-style").is_some());
    // }
    //
    // #[test]
    // fn test_azimuth() {
    //     let definitions = parse_mdn_definition_files().definitions;
    //     let def = definitions.get("azimuth").unwrap();
    //
    //     assert_some!(def.clone().matches(&CssValue::Unit(361.0, "deg".into())));
    //
    //     assert_none!(def.clone().matches(&CssValue::Unit(20.0, "blaat".into())));
    //
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::Unit(std::f32::consts::FRAC_PI_2, "rad".into())));
    //     assert_some!(def.clone().matches(&CssValue::Number(0.0)));
    //
    //     assert_some!(def.clone().matches(&CssValue::Unit(360.0, "deg".into())));
    //     assert_some!(def.clone().matches(&CssValue::Unit(360.0, "grad".into())));
    //     assert_some!(def.clone().matches(&CssValue::Unit(2.0, "grad".into())));
    //     assert_some!(def.clone().matches(&CssValue::Unit(-360.0, "grad".into())));
    //
    //     assert_none!(def.clone().matches(&CssValue::String("leftside".into())));
    //
    //     assert_some!(def.clone().matches(&CssValue::String("left-side".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("left".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("center".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("rightwards".into())));
    //     assert_some!(def.clone().matches(&CssValue::List(vec!(
    //         CssValue::String("far-right".into()),
    //         CssValue::String("behind".into()),
    //     ))));
    //     assert_some!(def.clone().matches(&CssValue::String("behind".into())));
    // }
    //
    // #[test]
    // fn test_background_color() {
    //     let definitions = parse_mdn_definition_files().definitions;
    //     let def = definitions.get("background-color").unwrap();
    //
    //     assert_some!(def.clone().matches(&CssValue::Inherit));
    //     assert_some!(def.clone().matches(&CssValue::String("transparent".into())));
    //
    //     assert_some!(def.clone().matches(&CssValue::String("red".into())));
    //     // System colors
    //     assert_some!(def.clone().matches(&CssValue::String("Canvas".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("CanvasText".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("CanvasText".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("Menu".into())));
    //
    //     assert_some!(def.clone().matches(&CssValue::String("blue".into())));
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::Color(RgbColor::from("#ff0000"))));
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::String("rebeccapurple".into())));
    //
    //     assert_none!(def
    //         .clone()
    //         .matches(&CssValue::String("thiscolordoesnotexist".into())));
    // }
    //
    // #[test]
    // fn test_background_attachments() {
    //     let definitions = parse_mdn_definition_files().definitions;
    //     let def = definitions.get("background-attachment").unwrap();
    //
    //     assert_some!(def.clone().matches(&CssValue::Inherit));
    //     assert_some!(def.clone().matches(&CssValue::String("scroll".into())));
    //     assert_some!(def.clone().matches(&CssValue::String("fixed".into())));
    //
    //     assert_none!(def.clone().matches(&CssValue::String("incorrect".into())));
    //     assert_none!(def
    //         .clone()
    //         .matches(&CssValue::String("rebeccapurple".into())));
    //     assert_none!(def.clone().matches(&CssValue::Zero));
    // }
    //
    // // #[test]
    // fn test_prop() {
    //     let definitions = parse_mdn_definition_files().definitions;
    //     let def = definitions.get("test-prop").unwrap();
    //
    //     // [ <percentage> | foo ] [ <number> | bar ]?
    //
    //     dbg!(&def);
    //
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("foo".into()),
    //         CssValue::String("bar".into()),
    //     ])));
    //
    //     assert_none!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("bar".into()),
    //         CssValue::String("foo".into()),
    //     ])));
    //
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::List(vec![CssValue::String("foo".into()),])));
    //
    //     assert_none!(def
    //         .clone()
    //         .matches(&CssValue::List(vec![CssValue::String("bar".into()),])));
    //
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(15.0),
    //         CssValue::String("bar".into()),
    //     ])));
    //
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(15.0),
    //         CssValue::Number(30.0),
    //     ])));
    //
    //     assert_none!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Number(30.0),
    //         CssValue::Percentage(15.0),
    //     ])));
    //
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("foo".into()),
    //         CssValue::Number(30.0),
    //     ])));
    //
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::List(vec![CssValue::Percentage(30.0),])));
    // }
    //
    // #[test]
    // fn test_background_position() {
    //     let definitions = parse_mdn_definition_files().definitions;
    //     let def = definitions.get("background-position").unwrap();
    //
    //     assert_none!(def.clone().matches(&CssValue::String("scroll".into())));
    //     assert_none!(def.clone().matches(&CssValue::String("fixed".into())));
    //     assert_none!(def.clone().matches(&CssValue::String("incorrect".into())));
    //     assert_none!(def
    //         .clone()
    //         .matches(&CssValue::String("rebeccapurple".into())));
    //
    //     assert_some!(def.clone().matches(&CssValue::Percentage(0.0)));
    //     assert_some!(def.clone().matches(&CssValue::Zero));
    //     assert_some!(def.clone().matches(&CssValue::Unit(12.34, "px".into())));
    //     assert_none!(def.clone().matches(&CssValue::Number(12.34)));
    //
    //     // background-position: left 10px top 20px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("left".into()),
    //         CssValue::Unit(10.0, "px".into()),
    //         CssValue::String("top".into()),
    //         CssValue::Unit(20.0, "px".into()),
    //     ])));
    //
    //     // background-position: right 15% bottom 5%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("right".into()),
    //         CssValue::Percentage(15.0),
    //         CssValue::String("bottom".into()),
    //         CssValue::Percentage(5.0),
    //     ])));
    //
    //     // background-position: center center;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("center".into()),
    //         CssValue::String("center".into()),
    //     ])));
    //
    //     // background-position: 75% 50%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(75.0),
    //         CssValue::Percentage(50.0),
    //     ])));
    //
    //     // background-position: 75%;
    //     assert_some!(def
    //         .clone()
    //         .matches(&CssValue::List(vec![CssValue::Percentage(75.0),])));
    //
    //     // background-position: top 10px center;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("top".into()),
    //         CssValue::Unit(10.0, "px".into()),
    //         CssValue::String("center".into()),
    //     ])));
    //
    //     // background-position: bottom 20px right 30px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("bottom".into()),
    //         CssValue::Unit(20.0, "px".into()),
    //         CssValue::String("right".into()),
    //         CssValue::Unit(30.0, "px".into()),
    //     ])));
    //
    //     // background-position: 20% 80%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(20.0),
    //         CssValue::Percentage(80.0),
    //     ])));
    //
    //     // background-position: left 5px bottom 15px, right 10px top 20px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("left".into()),
    //         CssValue::Unit(5.0, "px".into()),
    //         CssValue::String("bottom".into()),
    //         CssValue::Unit(15.0, "px".into()),
    //         CssValue::String("right".into()),
    //         CssValue::Unit(10.0, "px".into()),
    //         CssValue::String("top".into()),
    //         CssValue::Unit(20.0, "px".into()),
    //     ])));
    //
    //     // background-position: center top 35px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("center".into()),
    //         CssValue::String("top".into()),
    //         CssValue::Unit(35.0, "px".into()),
    //     ])));
    //
    //     // background-position: left 45% bottom 25%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("left".into()),
    //         CssValue::Percentage(45.0),
    //         CssValue::String("bottom".into()),
    //         CssValue::Percentage(25.0),
    //     ])));
    //
    //     // background-position: right 10% top 50px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("right".into()),
    //         CssValue::Percentage(10.0),
    //         CssValue::String("top".into()),
    //         CssValue::Unit(50.0, "px".into()),
    //     ])));
    //
    //     // background-position: 0% 0%, 100% 100%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(0.0),
    //         CssValue::Percentage(0.0),
    //         CssValue::Percentage(100.0),
    //         CssValue::Percentage(100.0),
    //     ])));
    //
    //     // background-position: left top, right bottom;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("left".into()),
    //         CssValue::String("top".into()),
    //         CssValue::String("right".into()),
    //         CssValue::String("bottom".into()),
    //     ])));
    //
    //     // background-position: 100% 0, 0 100%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(100.0),
    //         CssValue::Number(0.0),
    //         CssValue::Number(0.0),
    //         CssValue::Percentage(100.0),
    //     ])));
    //
    //     // background-position: left 25px bottom, center top;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("left".into()),
    //         CssValue::Unit(25.0, "px".into()),
    //         CssValue::String("bottom".into()),
    //         CssValue::String("center".into()),
    //         CssValue::String("top".into()),
    //     ])));
    //
    //     // background-position: top 10% left 20%, bottom 10% right 20%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("top".into()),
    //         CssValue::Percentage(10.0),
    //         CssValue::String("left".into()),
    //         CssValue::Percentage(20.0),
    //         CssValue::String("bottom".into()),
    //         CssValue::Percentage(10.0),
    //         CssValue::String("right".into()),
    //         CssValue::Percentage(20.0),
    //     ])));
    //
    //     // background-position: 10px 30px, 90% 10%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Unit(10.0, "px".into()),
    //         CssValue::Unit(30.0, "px".into()),
    //         CssValue::Percentage(90.0),
    //         CssValue::Percentage(10.0),
    //     ])));
    //
    //     // background-position: top right, bottom left 15px;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("top".into()),
    //         CssValue::String("right".into()),
    //         CssValue::String("bottom".into()),
    //         CssValue::String("left".into()),
    //         CssValue::Unit(15.0, "px".into()),
    //     ])));
    //
    //     // background-position: 50% 25%, 25% 75%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::Percentage(50.0),
    //         CssValue::Percentage(25.0),
    //         CssValue::Percentage(25.0),
    //         CssValue::Percentage(75.0),
    //     ])));
    //
    //     // background-position: right 5% bottom 5%, left 5% top 5%;
    //     assert_some!(def.clone().matches(&CssValue::List(vec![
    //         CssValue::String("right".into()),
    //         CssValue::Percentage(5.0),
    //         CssValue::String("bottom".into()),
    //         CssValue::Percentage(5.0),
    //         CssValue::String("left".into()),
    //         CssValue::Percentage(5.0),
    //         CssValue::String("top".into()),
    //         CssValue::Percentage(5.0),
    //     ])));
    // }
}
