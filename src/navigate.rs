//! Navigation is controlled by a `Navigation_Rules.yaml` file in conjunction with preferences.
//! See preference documentation for more info on navigation preferences.
#![allow(clippy::needless_return)]

use std::cell::{Ref, RefCell, RefMut};
use sxd_xpath::context::Evaluation;
use sxd_xpath::Value;
use sxd_document::dom::Element;
use sxd_document::Package;

use std::fmt;
use crate::canonicalize::{name, get_parent};
use crate::pretty_print::mml_to_string;
use crate::speech::{NAVIGATION_RULES, CONCAT_INDICATOR, CONCAT_STRING, SpeechRules, SpeechRulesWithContext, remove_optional_indicators, intent_from_mathml, overview_mathml, NAV_NODE_SPEECH_NOT_FOUND, speak_mathml};
use crate::infer_intent::add_fixity_children;
use crate::interface::copy_mathml;
#[cfg(not(target_family = "wasm"))]
use std::time::Instant;
use crate::errors::*;
use phf::phf_set;
use log::{debug};
use crate::xpath_functions::is_leaf;

pub const ID_OFFSET: &str = "data-id-offset";

const MAX_PLACE_MARKERS: usize = 10;

thread_local!{
    /// The current set of navigation rules
    pub static NAVIGATION_STATE: RefCell<NavigationState> =
            RefCell::new( NavigationState::new() );
}

pub static NAV_COMMANDS: phf::Set<&str> = phf_set! {
    "MovePrevious", "MoveNext", "MoveStart", "MoveEnd", "MoveLineStart", "MoveLineEnd", 
    "MoveCellPrevious", "MoveCellNext", "MoveCellUp", "MoveCellDown", "MoveColumnStart", "MoveColumnEnd", 
    "ZoomIn", "ZoomOut", "ZoomOutAll", "ZoomInAll", 
    "MoveLastLocation", 
    "ReadPrevious", "ReadNext", "ReadCurrent", "ReadCellCurrent", "ReadStart", "ReadEnd", "ReadLineStart", "ReadLineEnd", 
    "DescribePrevious", "DescribeNext", "DescribeCurrent", 
    "WhereAmI", "WhereAmIAll", 
    "ToggleZoomLockUp", "ToggleZoomLockDown", "ToggleSpeakMode", 
    "Exit", 
    "MoveTo0","MoveTo1","MoveTo2","MoveTo3","MoveTo4","MoveTo5","MoveTo6","MoveTo7","MoveTo8","MoveTo9",
    "Read0","Read1","Read2","Read3","Read4","Read5","Read6","Read7","Read8","Read9",
    "Describe0","Describe1","Describe2","Describe3","Describe4","Describe5","Describe6","Describe7","Describe8","Describe9",
    "SetPlacemarker0","SetPlacemarker1","SetPlacemarker2","SetPlacemarker3","SetPlacemarker4","SetPlacemarker5","SetPlacemarker6","SetPlacemarker7","SetPlacemarker8","SetPlacemarker9",
};

#[derive(Clone, PartialEq, Debug)]
pub struct NavigationPosition {
    pub current_node: String,           // id of current node
    pub current_node_offset: usize,     // for leaves, char offset in leaf (default = 0), otherwise id for artificial intent node
}

impl fmt::Display for NavigationPosition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}[+{}]", self.current_node, self.current_node_offset);
    }
}

const ILLEGAL_NODE_ID: &str = "!not set";     // an illegal 'id' value
impl Default for NavigationPosition {
    fn default() -> Self {
        NavigationPosition {
            current_node: ILLEGAL_NODE_ID.to_string(),
            current_node_offset: 0
        }
     }
}


#[derive(Debug, Clone)]
pub struct NavigationState {
    // it might be better to use a linked for the stacks, with the first node being the top
    // these two stacks should be kept in sync.
    position_stack: Vec<NavigationPosition>,    // all positions, so we can go back to them
    command_stack: Vec<&'static str>,           // all commands, so we can undo them
    place_markers: [NavigationPosition; MAX_PLACE_MARKERS],
    where_am_i: NavigationPosition,             // current 'where am i' location

    #[cfg(target_family = "wasm")]
    where_am_i_start_time: usize,               // FIX: for web
    #[cfg(not(target_family = "wasm"))]
    where_am_i_start_time: Instant,
    mode: String,                               // one of "Character", "Simple", or "Enhanced"
    speak_overview: bool,                       // true => describe after move; false => (standard) speech rules
}

impl fmt::Display for NavigationState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "NavigationState{{")?;
        write!(f, "  Position Stack: ")?;
        for (i, nav_state) in self.position_stack.iter().enumerate() {
            write!(f, "{}{}", if i==0 {""} else {", "}, nav_state)?;
        }
        writeln!(f)?;
        write!(f, "  Command Stack: ")?;
        for (i, nav_state) in self.command_stack.iter().enumerate() {
            write!(f, "{}{}", if i==0 {""} else {", "}, *nav_state)?;
        }
        writeln!(f)?;
        writeln!(f, "  where_am_i: {}, start_time: {:?}", self.where_am_i, self.where_am_i_start_time)?;
        writeln!(f, "  mode: {}, speak_overview: {}", self.mode, self.speak_overview)?;
        writeln!(f, "}}")?;
        return Ok( () );
    }
}

impl NavigationState {
    fn new() -> NavigationState {
        return NavigationState {
            position_stack: Vec::with_capacity(1024),
            command_stack: Vec::with_capacity(1024),
            place_markers: Default::default(),
            where_am_i: NavigationPosition::default(),
            // FIX: figure this out for the web
            #[cfg(target_family = "wasm")]
            where_am_i_start_time: 0,           // FIX: for web
            #[cfg(not(target_family = "wasm"))]
            where_am_i_start_time: Instant::now(),      // need to give it some value, and "default()" isn't an option
            mode: "".to_string(),                       // set latter when we have some context
            speak_overview: false,                      // set latter when we have some context
        };
    }

    pub fn reset(&mut self) {
        self.position_stack.clear();
        self.command_stack.clear();
        self.where_am_i = NavigationPosition::default();
        self.reset_start_time()
    }


    // defining reset_start_time because of the following message if done inline
    // attributes on expressions are experimental
    // see issue #15701 <https://github.com/rust-lang/rust/issues/15701> for more information
    #[cfg(target_family = "wasm")]
    fn reset_start_time(&mut self) {
         self.where_am_i_start_time = 0;
    }

    #[cfg(not(target_family = "wasm"))]
    fn reset_start_time(&mut self) {
         self.where_am_i_start_time = Instant::now();      // need to give it some value, and "default()" isn't an option
    }


    pub fn push(&mut self, position: NavigationPosition, command: &'static str) {
        self.position_stack.push(position);
        self.command_stack.push(command);
    }

    fn pop(&mut self) -> Option<(NavigationPosition, &'static str)> {
        assert_eq!(self.position_stack.len(), self.command_stack.len());
        if self.position_stack.is_empty() {
            return None;
        } else {
            return Some( (self.position_stack.pop().unwrap(), self.command_stack.pop().unwrap()) );
        }
    }

    fn top(&self) -> Option<(&NavigationPosition, &'static str)> {
        if self.position_stack.is_empty() {
            return None;
        }
        let last = self.position_stack.len()-1;
        return Some( (&self.position_stack[last], self.command_stack[last]) );
    }

    pub fn get_navigation_mathml<'a>(&self, mathml: Element<'a>) -> Result<(Element<'a>, usize)> {
        if self.position_stack.is_empty() {
            return Ok( (mathml, 0) );
        } else {
            let (position, _) = self.top().unwrap();
            return match get_node_by_id(mathml, position) {
                None => bail!("internal error: id '{}' was not found in mathml:\n{}",
                                position.current_node, mml_to_string(mathml)),
                Some(found) => Ok( (found, position.current_node_offset) )
            };
        }
    }

    pub fn get_navigation_mathml_id(&self, mathml: Element) -> (String, usize) {
        if self.position_stack.is_empty() {
            return (mathml.attribute_value("id").unwrap().to_string(), 0);
        } else {
            let (position, _) = self.top().unwrap();
            return (position.current_node.clone(), position.current_node_offset);
        }
    }

    fn init_navigation_context(&self, context: &mut sxd_xpath::Context, command: &'static str,
                               nav_state_top: Option<(&NavigationPosition, &'static str)>) {
        context.set_variable("NavCommand", command);

        if command == "WhereAmI" && self.where_am_i == NavigationPosition::default() {
            context.set_variable("NavNode", self.where_am_i.current_node.as_str());
            context.set_variable("NavNodeOffset", self.where_am_i.current_node_offset as f64);
        } else {
            let position = &self.position_stack[self.position_stack.len()-1];
            context.set_variable("NavNode", position.current_node.as_str());
            context.set_variable("NavNodeOffset", position.current_node_offset as f64);
        }

        // get the index from command (e.g., '3' in 'SetPlacemarker3 or MoveTo3' and set 'PlaceMarker' to it's position)
        if command.ends_with(|ch: char| ch.is_ascii_digit()) {
            let index = convert_last_char_to_number(command);
            let position = &self.place_markers[index];
            context.set_variable("PlaceMarkerIndex", index as f64);
            context.set_variable("PlaceMarker", position.current_node.as_str());
            context.set_variable("PlaceMarkerOffset", position.current_node_offset as f64);
        }
           
        context.set_variable("Overview", self.speak_overview);
        context.set_variable("ReadZoomLevel", (if self.mode == "Enhanced" {-1} else {1}) as f64);
        context.set_variable("MatchCounter", 0 as f64);

        if command == "MoveLastLocation" {
            let previous_command = match nav_state_top {
                None => "None",
                Some( (_, previous_command) ) => previous_command,
            };
            context.set_variable("PreviousNavCommand", previous_command);
        }

        // used by nav rules for speech -- needs an initial value so tests don't fail
        context.set_variable("SayCommand", "" );
        context.set_variable("Move2D", "" );
        context.set_variable("SpeakExpression", true );    // default is to speak the expr after navigation
        return;

        fn convert_last_char_to_number(str: &str) -> usize {
            let last_char = str.as_bytes()[str.len()-1];
            assert!( last_char.is_ascii_digit() );
            return (last_char - b'0') as usize;
        }
    }
}

// convert the last digit of a Placemarker command to an integer
fn convert_last_char_to_number(str: &str) -> usize {
    let last_char = str.as_bytes()[str.len()-1];
    assert!( last_char.is_ascii_digit() );
    return (last_char - b'0') as usize;
}

/// Get the node associated with a `NavigationPosition`.
/// This can be called on an intent tree 
fn get_node_by_id<'a>(mathml: Element<'a>, pos: &NavigationPosition) -> Option<Element<'a>> {
    if let Some(mathml_id) = mathml.attribute_value("id") &&
       mathml_id == pos.current_node.as_str() &&
        (is_leaf(mathml) ||
        mathml.attribute_value(ID_OFFSET).unwrap_or("0") == pos.current_node_offset.to_string()) {
        return Some(mathml);
    }

    for child in mathml.children() {
        if let Some(child) = child.element() &&
           let Some(found) = get_node_by_id(child, pos) {
                return Some(found);
            }
    }
    return None;
}

/// Search the mathml for the id and set the navigation node to that id
/// Resets the navigation stack
pub fn set_navigation_node_from_id(mathml: Element, id: &str, offset: usize) -> Result<()> {
    let current_node = id.to_string();
    let pos = NavigationPosition { current_node: current_node.clone(), current_node_offset: offset };
    let node = get_node_by_id(mathml, &pos);
    if node.is_some() {
        return NAVIGATION_STATE.with(|nav_state| {
            let mut nav_state = nav_state.borrow_mut();
            nav_state.reset();
            nav_state.push(NavigationPosition{
                current_node,
                current_node_offset: offset
            }, "None");
            return Ok( () );
        })
    } else {
        bail!("Id {} not found in MathML {}", id, mml_to_string(mathml));
    }
}

/// Get's the Nav Node from the context, with some exceptions such as Toggle commands where it isn't set.
/// Note: mathml can be any node. It isn't really used but some Element needs to be part of Evaluate().
pub fn get_nav_node<'c>(context: &sxd_xpath::Context<'c>, var_name: &str, mathml: Element<'c>, start_node: Element<'c>, command: &str, nav_mode: &str) -> Result<String> {
    let start_id = start_node.attribute_value("id").unwrap_or_default();
    if command.starts_with("Toggle") {
        return Ok( start_id.to_string() );
    } else {
        return context_get_variable(context, var_name, mathml)
                .with_context(|| format!("When trying to {} starting at id={} in {} mode",
                                                command, start_node.attribute_value("id").unwrap_or_default(), nav_mode));
    }
}

// FIX: think of a better place to put this, and maybe a better interface
/// Note: mathml can be any node. It isn't really used but some Element needs to be part of Evaluate().
/// If the context variable has String, Number, or Boolean xpath value, return it as a string. Otherwise it is an error
pub fn context_get_variable<'c>(context: &sxd_xpath::Context<'c>, var_name: &str, mathml: Element<'c>) -> Result<String> {
    // This is slightly roundabout because Context doesn't expose a way to get the values.
    // Instead, we create an "Evaluation", which is just one level of indirection.
    use sxd_xpath::nodeset::Node;
    let evaluation = Evaluation::new(context, Node::Element(mathml));
    return match evaluation.value_of(var_name.into()) {
        Some(value) => match value {
            Value::String(s) => Ok(s.clone()),
            Value::Number(f) => Ok(f.to_string()),
            Value::Boolean(b) => Ok(format!("{b}")),    // "true" or "false"
            Value::Nodeset(nodes) => {
                if nodes.size() == 1 &&
                   let Some(attr) = nodes.document_order_first().unwrap().attribute() {
                        return Ok(attr.value().to_string());
                    };
                let mut error_message = format!("Variable '{var_name}' set somewhere in navigate.yaml is nodeset and not an attribute: ");
                if nodes.size() == 0 {
                    error_message += &format!("0 nodes (false) -- {} set to non-existent node in\n{}",
                                              var_name, mml_to_string(mathml));
                } else {
                    let singular = nodes.size()==1;
                    error_message += &format!("{} node{}. {}:",
                            nodes.size(),
                            if singular {""} else {"s"},
                            if singular {"Node is"} else {"Nodes are"});
                    nodes.document_order()
                        .iter()
                        .enumerate()
                        .for_each(|(i, node)| {
                            match node {
                                Node::Element(mathml) =>
                                    error_message += &format!("#{}:\n{}",i, mml_to_string(*mathml)),
                                _ => error_message += &format!("'{node:?}'"),
                            }   
                        })    
                };
                bail!(error_message);
            },
        },
        None => bail!("Could not find value for navigation variable '{}'", var_name),
    }
}

/// Wrapper around context_get_variable to get an integer variable
fn context_get_int_variable<'c>(context: &sxd_xpath::Context<'c>, var_name: &str, mathml: Element<'c>) -> Result<usize> {
    let value = context_get_variable(context, var_name, mathml)?;
    return match value.parse::<usize>() {
        Ok(i) => Ok(i),
        Err(e) => bail!("Could not parse navigation variable '{}' with value '{}' as integer: {}", var_name, value, e),
    }
}

/// Given a key code along with the modifier keys, the current node is moved accordingly (or value reported in some cases).]
/// The spoken text for the new current node is returned.
pub fn do_mathml_navigate_key_press(mathml: Element,
            key: usize, shift_key: bool, control_key: bool, alt_key: bool, meta_key: bool) -> Result<String> {
    let (command, param) = key_press_to_command_and_param(key, shift_key, control_key, alt_key, meta_key)?;
    return do_navigate_command_and_param(mathml, command, param);
}

pub fn do_navigate_command_and_param(mathml: Element, command: NavigationCommand, param: NavigationParam) -> Result<String> {
    return do_navigate_command_string(mathml, navigation_command_string(command, param));
}

pub fn do_navigate_command_string(mathml: Element, nav_command: &'static str) -> Result<String> {   
    // first check to see if nav file has been changed -- don't bother checking in loop below
    NAVIGATION_RULES.with(|rules| {
        rules.borrow_mut().read_files()
    })?;

    if mathml.children().is_empty() {
        bail!("MathML has not been set -- can't navigate");
    };

    return NAVIGATION_STATE.with(|nav_state| {
        let mut nav_state = nav_state.borrow_mut();
        // debug!("MathML: {}", mml_to_string(mathml));
        if nav_state.position_stack.is_empty() {
            // initialize to root node
            nav_state.push(NavigationPosition{
                current_node: mathml.attribute_value("id").unwrap().to_string(),
                current_node_offset: 0
            }, "None")
        };

        return NAVIGATION_RULES.with(|rules| {
            let rules = rules.borrow();
            let new_package = Package::new();
            let mut rules_with_context = SpeechRulesWithContext::new(&rules, new_package.as_document(), "", 0);
            
            nav_state.mode = rules.pref_manager.as_ref().borrow().pref_to_string("NavMode");
            nav_state.speak_overview = rules.pref_manager.as_ref().borrow().pref_to_string("Overview") == "true";

            nav_state.init_navigation_context(rules_with_context.get_context(), nav_command, nav_state.top());
            
            // start navigation off at the right node
            if nav_command == "MoveLastLocation" {
                nav_state.pop();
            }

            // If no speech happened for some calls, we try the call again (e.g, no speech for invisible times).
            // To prevent to infinite loop, we limit the number of tries
            const LOOP_LIMIT: usize = 3;
            let mut cumulative_speech = String::with_capacity(120);
            for loop_count in 0..LOOP_LIMIT {
                match apply_navigation_rules(mathml, nav_command, &rules, &mut rules_with_context, &mut nav_state, loop_count) {
                    Ok( (speech, done)) => {
                        cumulative_speech = cumulative_speech + if loop_count==0 {""} else {" "} + speech.trim();
                        if done {
                            let (tts, rate) = {
                                let prefs = rules.pref_manager.borrow();
                                (prefs.pref_to_string("TTS"), prefs.pref_to_string("MathRate"))
                            };
                            if rate != "100" {
                                match tts.as_str() {
                                    "SSML"
                                        if !cumulative_speech.starts_with("<prosody rate") => {
                                            cumulative_speech = format!("<prosody rate='{}%'>{}</prosody>", &rate, &cumulative_speech);
                                        }
                                    "SAPI5"
                                        if !cumulative_speech.starts_with("<rate speed") => {
                                            cumulative_speech = format!(
                                                "<rate speed='{:.1}'>{}</rate>",
                                                10.0 * (0.01 * rate.parse::<f32>().unwrap_or(100.0)).log(3.0),
                                                cumulative_speech
                                            );
                                        }
                                    _ => (),  // do nothing
                                }
                            }
                                                return Ok( rules.pref_manager.borrow().get_tts()
                                            .merge_pauses(remove_optional_indicators(
                                                &cumulative_speech.replace(CONCAT_STRING, "")
                                                                    .replace(CONCAT_INDICATOR, "")                            
                                                            )
                                            .trim_start().trim_end_matches([' ', ',', ';'])) );
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            bail!("Internal error: Navigation exceeded limit of number of times no speech generated
                   when attempting to {} in {} mode start at id={} in this MathML:\n{}.",
                   nav_command, nav_state.mode, nav_state.top().unwrap().0.current_node, mml_to_string(mathml));
        });
    });

    fn get_start_node<'m>(mathml: Element<'m>, nav_state: &RefMut<NavigationState>) -> Result<Element<'m>>  {
        let element = match nav_state.top() {
            None => {
                let nav_position = NavigationPosition { current_node: mathml.attribute_value("id").unwrap().to_string(), current_node_offset: 0 };
                get_node_by_id(mathml, &nav_position)
            },
            Some( (position, _) ) => get_node_by_id(mathml, position),
        };

        return match element {
            Some(node) => Ok(node),
            None => {
                bail!("Internal Error: didn't find id/offset '{:?}' while attempting to start navigation. MathML is\n{}",
                      nav_state.top().map(|t| t.0), mml_to_string(mathml));
            }
        };
    }



    fn apply_navigation_rules<'c, 'm:'c>(mathml: Element<'m>, nav_command: &'static str,
            rules: &Ref<SpeechRules>, rules_with_context: &mut SpeechRulesWithContext<'c, '_, 'm>, nav_state: &mut RefMut<NavigationState>,
            loop_count: usize) -> Result<(String, bool)> {
        {
            let context = rules_with_context.get_context();
            context.set_variable("MatchCounter", loop_count as f64);
            nav_state.mode = context_get_variable(context, "NavMode", mathml)?;
        }

        let mut add_literal = nav_state.mode == "Character";
        let (intent, nav_intent) = if add_literal {
            (mathml, mathml)
        } else {
            let intent = intent_from_mathml(mathml, rules_with_context.get_document())?;
            (intent, add_fixity_children(copy_mathml(intent)))
        };

        let mut properties = "";
        if add_literal {
            properties  = mathml.attribute_value("data-intent-property").unwrap_or_default();
            if properties.contains(":literal:") {
                add_literal = false;
            } else {
                mathml.set_attribute_value("data-intent-property", (":literal:".to_string() + properties).as_str());
            };
        }
        // we should always find the start node.
        // however, if we were navigating by character, then switched the NavMode, the intent tree might not have that node in it
        let start_node = match get_start_node(nav_intent, nav_state) {
            Ok(node) => node,
            Err(_) => {
                // find the node in the other tree (probably mathml) and walk up to find a parent that has an id in both
                debug!("Could not find start_node in nav_intent -- trying other_tree");
                let other_tree = if nav_state.mode == "Character" {nav_intent} else {mathml};
                let mut found_node = get_start_node(other_tree, nav_state)?;
                while name(found_node) != "math" {
                    found_node = get_parent(found_node);
                    // debug!("found_node:\n{}", mml_to_string(found_node));
                    let temp_pos = NavigationPosition {
                        current_node: found_node.attribute_value("id").unwrap_or_default().to_string().clone(),
                        current_node_offset: found_node.attribute_value(ID_OFFSET).unwrap_or_default().parse::<usize>().unwrap_or_default(),
                    };
                    if let Some(intent_node) = get_node_by_id(nav_intent, &temp_pos) {
                        found_node = intent_node;
                        break;
                    }
                }
                found_node
            }
        };

        // debug!("intent=\n{}", mml_to_string(intent));
        // debug!("nav intent=\n{}", mml_to_string(nav_intent));
        // debug!("start_node id={}\n{}", nav_state.top().unwrap().0.current_node.as_str(), mml_to_string(start_node));
        // if name(start_node) != "math" {
        //     let mut parent= get_parent(start_node);
        //     if name(parent) != "math" {
        //         parent = get_parent(parent);
        //     }
        //     debug!("parent or grandparent of start_node:\n{}", mml_to_string(parent));
        // }
        let offset = context_get_int_variable(rules_with_context.get_context(), "NavNodeOffset", intent)?;
        rules_with_context.set_nav_node_offset(offset);
        debug!("starting nav_position: {}, start node ={}", nav_state.top().unwrap().0, name(start_node));

        let raw_speech_string = rules_with_context.match_pattern::<String>(start_node)
                    .context("Pattern match/replacement failure during math navigation!")?;
        let speech = rules.pref_manager.borrow().get_tts()
                    .merge_pauses(remove_optional_indicators(
                        &raw_speech_string.replace(CONCAT_STRING, "")
                                                .replace(CONCAT_INDICATOR, "")                            
                                    )
                    .trim());
        // debug!("Nav Speech: {}", speech);

        // FIX: add things that need to do a speech replacement based on some marker for "where am i" and others that loop ([Speak: id])???
        // what else needs to be done/set???

        // transfer some values that might have been set into the prefs
        let offset = context_get_int_variable(rules_with_context.get_context(), "NavNodeOffset", intent)?;
        rules_with_context.set_nav_node_offset(offset);
        let context = rules_with_context.get_context();
        nav_state.speak_overview = context_get_variable(context, "Overview", intent)? == "true";
        nav_state.mode = context_get_variable(context, "NavMode", intent)?;
        rules.pref_manager.as_ref().borrow_mut().set_user_prefs("NavMode", &nav_state.mode)?;

        debug!("context value of NavNodeOffset: {:?}", context_get_variable(context, "NavNodeOffset", intent)?);
        let nav_position = NavigationPosition {
                current_node: get_nav_node(context, "NavNode", intent, start_node, nav_command, &nav_state.mode)?,
                current_node_offset: context_get_int_variable(context, "NavNodeOffset", intent)?,
            };

        // after a command, we either read or describe the new location (part of state)
        // also some commands are DescribeXXX/ReadXXX, so we need to look at the commands also
        let use_read_rules = if nav_command.starts_with("Read") {
            true
        } else if nav_command.starts_with("Describe") {
            false
        } else {
            !nav_state.speak_overview
        };

        debug!("after match nav_position: {}", nav_position);
        // push the new location on the stack
        if nav_position != NavigationPosition::default() && &nav_position != nav_state.top().unwrap().0 {
            nav_state.push(nav_position.clone(), nav_command);
        }

        if nav_command.starts_with("SetPlacemarker") {
            let new_node_id = get_nav_node(context, "NavNode", intent, start_node, nav_command, &nav_state.mode)?;
            nav_state.place_markers[convert_last_char_to_number(nav_command)] = NavigationPosition{
                current_node: new_node_id,
                current_node_offset: context_get_int_variable(context, "NavNodeOffset", intent)?,
            }
        }

        let nav_mathml = get_node_by_id(intent, &nav_position);
        if nav_mathml.is_some() && context_get_variable(context, "SpeakExpression", intent)? == "true" {
            // Speak/Overview of where we landed (if we are supposed to speak it) -- use intent, not nav_intent
            // Note: NavMode might have changed, so we need to recheck the mode to see if we use LiteralSpeak
            let literal_speak = nav_state.mode == "Character";
            let node_speech_result = speak(mathml, intent, &nav_position, literal_speak, use_read_rules);
            remove_literal_property(mathml, add_literal, properties);
            let node_speech = match node_speech_result {
                Ok(speech) => speech,
                Err(e) => {
                    if e.to_string() == NAV_NODE_SPEECH_NOT_FOUND {
                        bail!("Internal error: With {}/{} in {} mode, can't {} from expression with id '{}' inside:\n{}",
                              rules.pref_manager.as_ref().borrow().pref_to_string("Language"),
                              rules.pref_manager.as_ref().borrow().pref_to_string("SpeechStyle"),
                              &nav_state.mode, nav_command, &nav_position.current_node, mml_to_string(if literal_speak {mathml} else {intent}));
                    }
                    return Err(e);
                }
            };

            // debug!("node_speech: '{}', speech: '{}'\n", node_speech, speech);
            if node_speech.is_empty() {
                // try again in loop
                return Ok( (speech, false));
            } else {
                pop_stack(nav_state, loop_count, nav_command);
                // debug!("returning: '{}'", speech.clone() + " " + &node_speech);
                return Ok( (speech + " " + &node_speech, true) );
            }
        } else {
            remove_literal_property(mathml, add_literal, properties);
            pop_stack(nav_state, loop_count, nav_command);
            return Ok( (speech, true) );
        };

        fn remove_literal_property(mathml: Element, add_literal: bool, properties: &str) {
            if add_literal {
                if properties.is_empty() {
                    mathml.remove_attribute("data-intent-property");
                } else {
                    mathml.set_attribute_value("data-intent-property", properties);
                }
            }
        }

    }


    fn pop_stack(nav_state: &mut NavigationState, count: usize, nav_command: &'static str) {
        // save the final state and pop the intermediate states that did nothing
        let push_command_on_stack = (nav_command.starts_with("Move") && nav_command != "MoveLastLocation") || nav_command.starts_with("Zoom");
        // debug!("pop_stack: nav_command={}, count={}, push? {} stack=\n{}", nav_command, count, push_command_on_stack, nav_state);
        if count == 0 {
            if !push_command_on_stack && nav_command == nav_state.top().unwrap().1 {
                nav_state.pop();    // remove ReadXXX, SetPlacemarker, etc. commands that don't change the state
            }
            return;
        }
        let (top_position, top_command) = nav_state.pop().unwrap();
        let mut count = count - 1;
        loop {
            // debug!("  ... loop count={}", count);
            nav_state.pop();
            if count == 0 {
                break;
            };
            count -= 1;
        };
        if push_command_on_stack {
            nav_state.push(top_position, top_command);
        }
        // debug!("END pop_stack: stack=\n{}", nav_state);
    }
}

/// Speak the intent tree at the nav_node_id if that id exists in the intent tree; otherwise use the mathml tree.
/// If full_read is true, we speak the tree, otherwise we use the overview rules.
/// If literal_speak is true, we use the literal speak rules (and use the mathml tree).
fn speak(mathml: Element, intent: Element, nav_position: &NavigationPosition, literal_speak: bool, full_read: bool) -> Result<String> {
    if full_read {
        // In something like x^3, we might be looking for the '3', but it will be "cubed", so we don't find it.
        // Or we might be on a "(" surrounding a matrix and that isn't part of the intent
        // We are probably safer in terms of getting the same speech if we retry intent starting at the nav node,
        //  but the node to speak is almost certainly trivial.
        // By speaking the non-intent tree, we are certain to speak on the next try
        if !literal_speak && get_node_by_id(intent, nav_position).is_some() {
                // debug!("speak: nav_node_id={}, intent=\n{}", nav_node_id, mml_to_string(intent));
            match speak_mathml(intent, &nav_position.current_node, nav_position.current_node_offset) {
                Ok(speech) => return Ok(speech),
                Err(e) => {
                    if e.to_string() != NAV_NODE_SPEECH_NOT_FOUND {
                        return Err(e);
                    }
                    // else could be something like '3' in 'x^3' ("cubed")
                },
            }
        }
        // debug!("speak (literal): nav_node_id={}, mathml=\n{}", nav_node_id, mml_to_string(mathml));
        let speech = speak_mathml(mathml,
                &nav_position.current_node, nav_position.current_node_offset);
        // debug!("speech from speak: {:?}", speech);
        return speech;
    } else {
        return overview_mathml(mathml, &nav_position.current_node, nav_position.current_node_offset);
    }
}


// MathPlayer's interface mentions these, so we keep them.
// These (KeyboardEvent.keyCode) are consistent across platforms (mostly?) but are deprecated.
//   KeyboardEvent.code is recommended instead (a string)
const VK_LEFT: usize = 0x25;
const VK_RIGHT: usize = 0x27;
const VK_UP: usize = 0x26;
const VK_DOWN: usize = 0x28;
const VK_RETURN: usize = 0x0D;
const VK_SPACE: usize = 0x20;
const VK_HOME: usize = 0x24;
const VK_END: usize = 0x23;
const VK_BACK: usize = 0x08;
const VK_ESCAPE: usize = 0x1B;

// Utilities that returns one of four commands/params based on shift/control key combinations

pub enum NavigationCommand {
    Move,
    Zoom,
    MoveLastLocation,
    Read,
    Describe,
    ReadTo,
    Locate,
    ChangeNavMode,
    ToggleSpeakMode,
    SetPlacemarker,
    Exit,
    Last,
}

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum NavigationParam {
    Placemarker0,
    Placemarker1,
    Placemarker2,
    Placemarker3,
    Placemarker4,
    Placemarker5,
    Placemarker6,
    Placemarker7,
    Placemarker8,
    Placemarker9,
    Previous,
    Current,
    Next,
    Start,
    End,
    LineStart,
    LineEnd,
    CellPrevious,
    CellCurrent,
    CellNext,
    ColStart,
    ColEnd,
    CellUp,
    CellDown,
    Last 
}


fn choose_command(
	shift_key: bool,
	control_key: bool,
	none: NavigationCommand,
	shift: NavigationCommand,
	control: NavigationCommand,
	shift_control: NavigationCommand
) -> NavigationCommand {
	   if shift_key && control_key {
		return shift_control;
    } else if control_key {
        return control;
    } else if shift_key {
		return shift;
	} else {
		return none;
    }
}

fn choose_param(
	shift_key: bool,
	control_key: bool,
	none: NavigationParam,
	shift: NavigationParam,
	control: NavigationParam,
	shift_control: NavigationParam
) -> NavigationParam {
    if shift_key && control_key {
		return shift_control;
    } else if control_key {
        return control;
    } else if shift_key {
		return shift;
	} else {
		return none;
    }
}

fn key_press_to_command_and_param(
    key: usize,
	shift_key: bool,
	control_key: bool,
	alt_key: bool,
	meta_key: bool,
) -> Result<(NavigationCommand, NavigationParam)> {
	// key press mapping should probably be stored externally (registry) with an app that allows changes
	// for now, we build in the defaults

    // this is a hack to map alt+ctl+arrow to ctl+arrow to change table mappings (github.com/NSoiffer/MathCAT/issues/105)
    // if this change sticks, choose_command() needs to be changed and this hack should go away
    let mut alt_key = alt_key;
    if alt_key && control_key && [VK_LEFT, VK_RIGHT, VK_UP, VK_DOWN].contains(&key) {
        alt_key = false;
    }
	if alt_key || meta_key {
        bail!("Invalid argument to key_press_to_command_and_param");
    }

    let command;
    let param;
	match key {
        VK_LEFT => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move,   NavigationCommand::Read,	NavigationCommand::Move,	   NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous, NavigationParam::Previous, NavigationParam::CellPrevious, NavigationParam::Previous);
            },
        VK_RIGHT => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move,	NavigationCommand::Read, NavigationCommand::Move,	  NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Next, NavigationParam::Next, NavigationParam::CellNext, NavigationParam::Next);
            },
        VK_UP => {
            command = choose_command(shift_key, control_key, NavigationCommand::Zoom,      NavigationCommand::ChangeNavMode, NavigationCommand::Move,   NavigationCommand::Zoom);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous,  NavigationParam::Previous,      NavigationParam::CellUp, NavigationParam::Start);
            },
        VK_DOWN => {
            command = choose_command(shift_key, control_key, NavigationCommand::Zoom, NavigationCommand::ChangeNavMode, NavigationCommand::Move,     NavigationCommand::Zoom);
            param =   choose_param(  shift_key, control_key, NavigationParam::Next, NavigationParam::Next,          NavigationParam::CellDown, NavigationParam::End);
            },
        VK_RETURN => {
            command = choose_command(shift_key, control_key, NavigationCommand::Locate,  NavigationCommand::Last, NavigationCommand::Locate, NavigationCommand::Last);
            param =   choose_param(  shift_key, control_key, NavigationParam::Previous,NavigationParam::Last, NavigationParam::Last,    NavigationParam::Last);
            },
        VK_SPACE => {
            command = choose_command(shift_key, control_key, NavigationCommand::Read,		NavigationCommand::ToggleSpeakMode,    NavigationCommand::Read,        NavigationCommand::Describe);
            param =   choose_param(  shift_key, control_key, NavigationParam::Current, NavigationParam::Last,                NavigationParam::CellCurrent, NavigationParam::Current);
            },
    
        VK_HOME => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Move,	   NavigationCommand::Move,      NavigationCommand::ReadTo);
            param =   choose_param(  shift_key, control_key, NavigationParam::Start,NavigationParam::ColStart, NavigationParam::LineStart, NavigationParam::Start);
            },
        VK_END => {
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Move,   NavigationCommand::Move,    NavigationCommand::ReadTo);
            param =   choose_param(  shift_key, control_key, NavigationParam::End,  NavigationParam::ColEnd, NavigationParam::LineEnd, NavigationParam::End);
            },
        VK_BACK => {
            command = NavigationCommand::MoveLastLocation;
            param = NavigationParam::Last;
            },
        VK_ESCAPE => {
            command = NavigationCommand::Exit;
            param = NavigationParam::Last;
            },
        0x30..=0x39 => {  // '0' ... '9'
            command = choose_command(shift_key, control_key, NavigationCommand::Move, NavigationCommand::Read, NavigationCommand::SetPlacemarker, NavigationCommand::Describe);
            static PLACE_MARKER: &[NavigationParam] = &[
                NavigationParam::Placemarker0,
                NavigationParam::Placemarker1,
                NavigationParam::Placemarker2,
                NavigationParam::Placemarker3,
                NavigationParam::Placemarker4,
                NavigationParam::Placemarker5,
                NavigationParam::Placemarker6,
                NavigationParam::Placemarker7,
                NavigationParam::Placemarker8,
                NavigationParam::Placemarker9,
            ];
            param = PLACE_MARKER[key-0x30];
        },
        _ => bail!("Unknown key press/command"),
    };
    
	return Ok( (command, param) );
}

// translate the key presses into commands


fn navigation_command_string(command: NavigationCommand, param: NavigationParam) -> &'static str {
	match command {
	    NavigationCommand::Move => {
            return match param {
                NavigationParam::Previous => "MovePrevious",
                NavigationParam::Next => "MoveNext",
                NavigationParam::Start => "MoveStart",
                NavigationParam::End => "MoveEnd",
                NavigationParam::LineStart => "MoveLineStart",
                NavigationParam::LineEnd => "MoveLineEnd",
                NavigationParam::CellPrevious => "MoveCellPrevious",
                NavigationParam::CellNext => "MoveCellNext",
                NavigationParam::CellUp => "MoveCellUp",
                NavigationParam::CellDown => "MoveCellDown",
                NavigationParam::ColStart => "MoveColumnStart",
                NavigationParam::ColEnd => "MoveColumnEnd",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Move");
                    }
                    static MOVE_TO: &[&str] = &["MoveTo0","MoveTo1","MoveTo2","MoveTo3","MoveTo4","MoveTo5","MoveTo6","MoveTo7","MoveTo8","MoveTo9"];
                    return MOVE_TO[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                }
            }
        },
        NavigationCommand::Zoom => {
            return match param {
                NavigationParam::Next => "ZoomIn",
                NavigationParam::Previous => "ZoomOut",
                NavigationParam::Start => "ZoomOutAll",
                NavigationParam::End => "ZoomInAll",
                _  => panic!("Illegal param for NavigationCommand::Zoom"),
            }
        },
        NavigationCommand::MoveLastLocation => {
            return "MoveLastLocation";
        },
        NavigationCommand::Read => {
            return match param {
                NavigationParam::Previous => "ReadPrevious",
                NavigationParam::Next => "ReadNext",
                NavigationParam::Current => "ReadCurrent",
                NavigationParam::CellCurrent => "ReadCellCurrent",
                NavigationParam::Start => "ReadStart",
                NavigationParam::End => "ReadEnd",
                NavigationParam::LineStart => "ReadLineStart",
                NavigationParam::LineEnd => "ReadLineEnd",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Move");
                    }
                    static READ_PLACE_MARKERS: &[&str] = &["Read0","Read1","Read2","Read3","Read4","Read5","Read6","Read7","Read8","Read9"];
                    return READ_PLACE_MARKERS[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                },
            }
        },
        NavigationCommand::Describe => {
            return match param {
                NavigationParam::Previous => "DescribePrevious",
                NavigationParam::Next => "DescribeNext",
                NavigationParam::Current => "DescribeCurrent",
                _ => {
                    if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                        panic!("Internal Error: Found illegal value for param of NavigationCommand::Describe");
                    }
                    static DESCRIBE_PLACE_MARKERS: &[&str] = &["Describe0","Describe1","Describe2","Describe3","Describe4","Describe5","Describe6","Describe7","Describe8","Describe9"];
                    return DESCRIBE_PLACE_MARKERS[(param as usize) - (NavigationParam::Placemarker0 as usize)];
                }
            }
        },
        NavigationCommand::ReadTo => {
            todo!("ReadTo navigation command")
        },
        NavigationCommand::Locate => {
            if param ==NavigationParam::Previous {
                return "WhereAmI";
            } else if param ==NavigationParam::Last {
                return "WhereAmIAll";
            }
        },
        NavigationCommand::ChangeNavMode => {
            if param ==NavigationParam::Previous {
                return "ToggleZoomLockUp";
            } else if param ==NavigationParam::Next {
                return "ToggleZoomLockDown";
            }
        },
        NavigationCommand::ToggleSpeakMode => {
            return "ToggleSpeakMode";
        },
        NavigationCommand::SetPlacemarker => {
            if param < NavigationParam::Placemarker0 || param > NavigationParam::Placemarker9 {
                panic!("Internal Error: Found illegal value for param of NavigationCommand::SetPlacemarker");
            }
            static SET_PLACE_MARKER: &[&str] = &["SetPlacemarker0","SetPlacemarker1","SetPlacemarker2","SetPlacemarker3","SetPlacemarker4","SetPlacemarker5","SetPlacemarker6","SetPlacemarker7","SetPlacemarker8","SetPlacemarker9"];
            return SET_PLACE_MARKER[(param as usize) - (NavigationParam::Placemarker0 as usize)];
        },
        NavigationCommand::Exit => {
            return "Exit";
        },
        NavigationCommand::Last => {
            return "Error";
        }
    };
    return "Error";
}