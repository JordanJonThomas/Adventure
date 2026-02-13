//! The grammar module defines how phrases are properly structured for each verb.
//!
//! For example, to get an item a player may type "Get foo", "Get foo and bar",
//! "Get foo, bar and baz", or "Get all".
//!
//! Some phrases may only have 1 correct grammatical structure and some may have many.

// TODO: The printed text from NotParseable uses placeholder text right now. Instead, each specific 
// non-parsable instance should declare to the user what they said wrong.

use crate::{errors::ParseError, models::{Determiner, Direction}, parser::{command::Command, tokens::Token}};

#[derive(Debug, Clone, PartialEq)]
pub struct NounPhrase {
    pub adjectives: Vec<String>,
    pub noun: String,
}

/// A verb the player is attempting to execute.
///
/// Verbs are used to parse the input from the user
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verb {
    // Move can be written as "go north" or just "north"
    // So the identified verb of moving can be with or
    // without context based on the first token alone.
    Move { dir: Option<Direction> },
    Get,
    Drop,
    Put,
    Look,
    Read,
    Open,
    Close,
    Attack,
    Unlock,
}

/// Grammatically defines the relation between two interacting nouns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preposition {
    At,
    In,
    On,
    With,
    To,
    Through,
}

pub fn parse_verb(word: &str) -> Option<Verb> {
    match word {
        "move" | "go" | "walk"  => Some(Verb::Move { dir: None }),
        "get" | "take" | "grab" => Some(Verb::Get),
        "drop" | "release" => Some(Verb::Drop),
        "put" | "place" => Some(Verb::Put),
        "look" | "examine" | "inspect" | "l" => Some(Verb::Look),
        "open" => Some(Verb::Open),
        "close" | "shut" => Some(Verb::Close),
        "attack" | "hit" | "fight" | "kill" => Some(Verb::Attack),
        "unlock" => Some(Verb::Unlock),
        "read" => Some(Verb::Read),
        word => {
            // If the player specified the direction only
            if let Ok(dir) = Direction::try_from(word) {
                return Some(Verb::Move { dir: Some(dir) });
            }

            None
        }
    }
}

pub fn parse_preposition(word: &str) -> Option<Preposition> {
    match word {
        "at" => Some(Preposition::At),
        "in" => Some(Preposition::In),
        "on" => Some(Preposition::On),
        "with" => Some(Preposition::With),
        "to" => Some(Preposition::To),
        "through" => Some(Preposition::Through),
        _ => None
    }
}

/// Main dispatcher for parsing tokens based on verb-specific grammar rules.
pub fn parse_phrase(verb: Verb, tokens: &[Token]) -> Result<Command, ParseError> {
    match verb {
        Verb::Move { .. } => parse_move(verb, tokens), // verb must be passed here as direction
                                                       // names are considered a verb
        Verb::Get => parse_get(tokens),
        Verb::Drop => parse_drop(tokens),
        Verb::Put => parse_put(tokens),
        Verb::Look => parse_look(tokens),
        Verb::Open => parse_open(tokens),
        Verb::Close => parse_close(tokens),
        Verb::Attack => parse_attack(tokens),
        Verb::Unlock => parse_unlock(tokens),
        Verb::Read => parse_read(tokens),
    }
}

/// Parses a noun phrase from a slice of tokens.
/// The last token is treated as the noun, all preceding tokens as adjectives.
fn parse_noun_phrase(tokens: &[Token]) -> Result<NounPhrase, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    // exclude "the", "some", etc
    let filtered: Vec<&Token> = tokens
        .iter()
        .filter(|tk| Determiner::try_from(tk.text.as_str()).is_err())
        .collect();

    if filtered.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let noun = filtered.last().unwrap().text.clone();
    let adjectives = filtered[..filtered.len() - 1]
        .iter()
        .map(|t| t.text.clone())
        .collect();

    Ok(NounPhrase { adjectives, noun })
}

/// Finds a preposition in tokens that matches one of the valid prepositions.
/// Returns the index of the first matching preposition, or None if not found.
fn find_preposition(tokens: &[Token], valid_preps: &[Preposition]) -> Option<usize> {
    for (i, token) in tokens.iter().enumerate() {
        if let Some(prep) = parse_preposition(&token.text) {
            if valid_preps.contains(&prep) {
                return Some(i);
            }
        }
    }
    None
}

/// Parses a list of noun phrases separated by "and" or commas.
/// Examples: "sword", "sword and shield", "sword, shield and helmet"
fn parse_noun_list(tokens: &[Token]) -> Result<Vec<NounPhrase>, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let mut nouns = Vec::new();
    let mut current_tokens = Vec::new();

    for token in tokens {
        // "and" and commas are separators
        if token.text == "and" || token.text == "," {
            if !current_tokens.is_empty() {
                nouns.push(parse_noun_phrase(&current_tokens)?);
                current_tokens.clear();
            }
        } else {
            current_tokens.push(token.clone());
        }
    }

    // Don't forget the last noun phrase
    if !current_tokens.is_empty() {
        nouns.push(parse_noun_phrase(&current_tokens)?);
    }

    if nouns.is_empty() {
        return Err(ParseError::NotParseable);
    }

    Ok(nouns)
}

// Verb parse methods

// Move: "north", "go north"
fn parse_move(verb: Verb, tokens: &[Token]) -> Result<Command, ParseError> {
    // If direction was already captured (e.g., just "north"), remaining should be empty
    if let Verb::Move { dir: Some(_) } = verb {
        if !tokens.is_empty() {
            return Err(ParseError::NotParseable);
        }
        return Ok(Command {
            verb,
            direct: vec![],
            preposition: None,
            indirect: None,
        });
    }

    // Otherwise parse "go north", needs exactly one token that is a direction
    if tokens.len() != 1 {
        return Err(ParseError::NotParseable);
    }

    let dir = Direction::try_from(tokens[0].text.as_str())
        .map_err(|_| ParseError::NotParseable)?;

    Ok(Command {
        verb: Verb::Move { dir: Some(dir) },
        direct: vec![],
        preposition: None,
        indirect: None,
    })
}

// Get: "get sword", "get sword and shield", "get red sword, blue shield and helmet"
fn parse_get(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let direct = parse_noun_list(tokens)?;

    Ok(Command {
        verb: Verb::Get,
        direct,
        preposition: None,
        indirect: None,
    })
}

// Drop: "drop sword", "drop sword and shield"
fn parse_drop(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let direct = parse_noun_list(tokens)?;

    Ok(Command {
        verb: Verb::Drop,
        direct,
        preposition: None,
        indirect: None,
    })
}

// Put: "put sword in bag", "put key on table"
fn parse_put(tokens: &[Token]) -> Result<Command, ParseError> {
    // Need at least: direct_noun preposition indirect_noun (minimum 3 tokens)
    if tokens.len() < 3 {
        return Err(ParseError::NotParseable);
    }

    // Find preposition - only "in" or "on" are valid for put
    let valid_preps = &[Preposition::In, Preposition::On];
    let prep_idx = find_preposition(tokens, valid_preps)
        .ok_or(ParseError::NotParseable)?;

    // Preposition can't be at start or end
    if prep_idx == 0 || prep_idx >= tokens.len() - 1 {
        return Err(ParseError::NotParseable);
    }

    // Parse direct object (before preposition)
    let direct = parse_noun_phrase(&tokens[..prep_idx])?;

    // Parse indirect object (after preposition)
    let indirect = parse_noun_phrase(&tokens[prep_idx + 1..])?;

    // Get the actual preposition
    let preposition = parse_preposition(&tokens[prep_idx].text);

    Ok(Command {
        verb: Verb::Put,
        direct: vec![direct],
        preposition,
        indirect: Some(indirect),
    })
}

// Look: "look", "look at sword", "look sword"
fn parse_look(tokens: &[Token]) -> Result<Command, ParseError> {
    // Empty tokens means just "look" at surroundings
    if tokens.is_empty() {
        return Ok(Command {
            verb: Verb::Look,
            direct: vec![],
            preposition: None,
            indirect: None,
        });
    }

    // Check if first token is "at" and skip it
    let tokens_to_parse = if tokens[0].text == "at" {
        if tokens.len() == 1 {
            // Just "look at" with nothing after is invalid
            return Err(ParseError::NotParseable);
        }
        &tokens[1..]
    } else {
        tokens
    };

    // Parse the target noun phrase
    let direct = parse_noun_phrase(tokens_to_parse)?;

    Ok(Command {
        verb: Verb::Look,
        direct: vec![direct],
        preposition: None,
        indirect: None,
    })
}

// Read: "read book"
fn parse_read(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let direct = parse_noun_phrase(tokens)?;

    Ok(Command {
        verb: Verb::Read,
        direct: vec![direct],
        preposition: None,
        indirect: None,
    })
}

// Open: "open door", "open red door", "open door with key"
fn parse_open(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    // Look for "with" preposition. If found, redirect to unlock
    let valid_preps = &[Preposition::With];
    if let Some(_prep_idx) = find_preposition(tokens, valid_preps) {
        return parse_unlock(tokens);
    }

    let direct = parse_noun_phrase(tokens)?;

    Ok(Command {
        verb: Verb::Open,
        direct: vec![direct],
        preposition: None,
        indirect: None,
    })
}

// Close: "close door"
fn parse_close(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    let direct = parse_noun_phrase(tokens)?;

    Ok(Command {
        verb: Verb::Close,
        direct: vec![direct],
        preposition: None,
        indirect: None,
    })
}

// Attack: "attack goblin", "attack goblin with sword"
fn parse_attack(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    // Look for "with" preposition
    let valid_preps = &[Preposition::With];
    if let Some(prep_idx) = find_preposition(tokens, valid_preps) {
        // "attack goblin with sword"

        // Preposition can't be at start or end
        if prep_idx == 0 || prep_idx >= tokens.len() - 1 {
            return Err(ParseError::NotParseable);
        }

        // Parse target (before "with")
        let direct = parse_noun_phrase(&tokens[..prep_idx])?;

        // Parse weapon (after "with")
        let indirect = parse_noun_phrase(&tokens[prep_idx + 1..])?;

        Ok(Command {
            verb: Verb::Attack,
            direct: vec![direct],
            preposition: Some(Preposition::With),
            indirect: Some(indirect),
        })
    } else {
        // "attack goblin" with no weapon specified
        let direct = parse_noun_phrase(tokens)?;

        Ok(Command {
            verb: Verb::Attack,
            direct: vec![direct],
            preposition: None,
            indirect: None,
        })
    }
}

// Unlock: "unlock door with key", "unlock chest with rusty key"
fn parse_unlock(tokens: &[Token]) -> Result<Command, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError::NotParseable);
    }

    // Look for "with" preposition
    let valid_preps = &[Preposition::With];
    if let Some(prep_idx) = find_preposition(tokens, valid_preps) {
        // Preposition can't be at start or end
        if prep_idx == 0 || prep_idx >= tokens.len() - 1 {
            return Err(ParseError::NotParseable);
        }

        // Parse target (before "with")
        let direct = parse_noun_phrase(&tokens[..prep_idx])?;

        // Parse key (after "with")
        let indirect = parse_noun_phrase(&tokens[prep_idx + 1..])?;

        Ok(Command {
            verb: Verb::Unlock,
            direct: vec![direct],
            preposition: Some(Preposition::With),
            indirect: Some(indirect),
        })
    } else {
        Err(ParseError::NotParseable)
    }
}


#[cfg(test)]
mod test {
    use crate::parser::parse;
    use super::*;

    // Helper to create a NounPhrase easily
    fn noun(word: &str) -> NounPhrase {
        NounPhrase {
            adjectives: vec![],
            noun: word.to_string(),
        }
    }

    fn noun_with_adj(adj: &str, word: &str) -> NounPhrase {
        NounPhrase {
            adjectives: vec![adj.to_string()],
            noun: word.to_string(),
        }
    }

    // ===== MOVE Tests =====

    #[test]
    fn move_direction_only() {
        let inputs = ["north", "south", "east", "west", "up", "down"];
        for input in inputs {
            let result = parse(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let cmd = result.unwrap();
            assert!(matches!(cmd.verb, Verb::Move { dir: Some(_) }));
            assert!(cmd.direct.is_empty());
        }
    }

    #[test]
    fn move_with_go() {
        let strs = ["go north", "move north", "walk south"];
        for input in strs {
            let result = parse(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            let cmd = result.unwrap();
            assert!(matches!(cmd.verb, Verb::Move { dir: Some(_) }));
        }
    }

    #[test]
    fn move_invalid() {
        let invalid = ["go", "north extra", "move up down"];
        for input in invalid {
            let result = parse(input);
            assert!(result.is_err(), "Should fail to parse: {}", input);
        }
    }

    // ===== GET Tests =====

    #[test]
    fn get_single_item() {
        let result = parse("get sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Get);
        assert_eq!(cmd.direct.len(), 1);
        assert_eq!(cmd.direct[0], noun("sword"));
    }

    #[test]
    fn get_with_adjective() {
        let result = parse("get red sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun_with_adj("red", "sword"));
    }

    #[test]
    fn get_multiple_items() {
        let result = parse("get sword and shield");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Get);
        assert_eq!(cmd.direct.len(), 2);
        assert_eq!(cmd.direct[0], noun("sword"));
        assert_eq!(cmd.direct[1], noun("shield"));
    }

    #[test]
    fn get_list_with_commas() {
        let result = parse("get sword, shield and helmet");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct.len(), 3);
    }

    #[test]
    fn get_invalid() {
        assert!(parse("get").is_err());
    }

    // ===== DROP Tests =====

    #[test]
    fn drop_single_item() {
        let result = parse("drop sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Drop);
        assert_eq!(cmd.direct.len(), 1);
    }

    #[test]
    fn drop_multiple_items() {
        let result = parse("drop sword and shield");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct.len(), 2);
    }

    #[test]
    fn drop_invalid() {
        assert!(parse("drop").is_err());
    }

    // ===== PUT Tests =====

    #[test]
    fn put_in_container() {
        let result = parse("put sword in bag");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Put);
        assert_eq!(cmd.direct[0], noun("sword"));
        assert_eq!(cmd.preposition, Some(Preposition::In));
        assert_eq!(cmd.indirect, Some(noun("bag")));
    }

    #[test]
    fn put_on_surface() {
        let result = parse("put key on table");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.preposition, Some(Preposition::On));
    }

    #[test]
    fn put_with_adjectives() {
        let result = parse("put red sword in wooden chest");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun_with_adj("red", "sword"));
        assert_eq!(cmd.indirect, Some(noun_with_adj("wooden", "chest")));
    }

    #[test]
    fn put_invalid_preposition() {
        // "with", "at", "to" are not valid for put
        assert!(parse("put sword with bag").is_err());
        assert!(parse("put sword at table").is_err());
    }

    #[test]
    fn put_missing_parts() {
        assert!(parse("put sword").is_err());
        assert!(parse("put sword in").is_err());
        assert!(parse("put in bag").is_err());
    }

    // ===== LOOK Tests =====

    #[test]
    fn look_no_target() {
        let result = parse("look");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Look);
        assert!(cmd.direct.is_empty());
    }

    #[test]
    fn look_at_target() {
        let result = parse("look at sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct.len(), 1);
        assert_eq!(cmd.direct[0], noun("sword"));
    }

    #[test]
    fn look_without_at() {
        let result = parse("look sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun("sword"));
    }

    #[test]
    fn look_at_nothing() {
        // "look at" with nothing after is invalid
        assert!(parse("look at").is_err());
    }

    // ===== OPEN Tests =====

    #[test]
    fn open_door() {
        let result = parse("open door");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Open);
        assert_eq!(cmd.direct[0], noun("door"));
    }

    #[test]
    fn open_with_adjective() {
        let result = parse("open red door");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun_with_adj("red", "door"));
    }

    #[test]
    fn open_invalid() {
        assert!(parse("open").is_err());
    }

    // ===== CLOSE Tests =====

    #[test]
    fn close_door() {
        let result = parse("close door");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Close);
        assert_eq!(cmd.direct[0], noun("door"));
    }

    #[test]
    fn close_invalid() {
        assert!(parse("close").is_err());
    }

    // ===== ATTACK Tests =====

    #[test]
    fn attack_target_only() {
        let result = parse("attack goblin");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.verb, Verb::Attack);
        assert_eq!(cmd.direct[0], noun("goblin"));
        assert_eq!(cmd.preposition, None);
        assert_eq!(cmd.indirect, None);
    }

    #[test]
    fn attack_with_weapon() {
        let result = parse("attack goblin with sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun("goblin"));
        assert_eq!(cmd.preposition, Some(Preposition::With));
        assert_eq!(cmd.indirect, Some(noun("sword")));
    }

    #[test]
    fn attack_with_adjectives() {
        let result = parse("attack big goblin with magic sword");
        assert!(result.is_ok());
        let cmd = result.unwrap();
        assert_eq!(cmd.direct[0], noun_with_adj("big", "goblin"));
        assert_eq!(cmd.indirect, Some(noun_with_adj("magic", "sword")));
    }

    #[test]
    fn attack_invalid() {
        assert!(parse("attack").is_err());
        assert!(parse("attack goblin with").is_err());
        assert!(parse("attack with sword").is_err());
    }

    // ===== VERB ALIASES Tests =====

    #[test]
    fn test_verb_aliases() {
        // Test various aliases work
        assert!(parse("take sword").is_ok());
        assert!(parse("grab sword").is_ok());
        assert!(parse("release sword").is_ok());
        assert!(parse("place sword in bag").is_ok());
        assert!(parse("examine sword").is_ok());
        assert!(parse("l").is_ok()); // shorthand for look
        assert!(parse("shut door").is_ok());
        assert!(parse("hit goblin").is_ok());
        assert!(parse("kill goblin with sword").is_ok());
    }

    // ===== EDGE CASES =====

    #[test]
    fn test_empty_input() {
        assert!(parse("").is_err());
        assert!(parse("   ").is_err());
    }

    #[test]
    fn test_unknown_verb() {
        assert!(parse("dance around").is_err());
        assert!(parse("xyz foo bar").is_err());
    }
}
