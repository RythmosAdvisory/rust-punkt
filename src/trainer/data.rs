use std::hash::Hash;
use std::str::FromStr;
use std::default::Default;
use std::borrow::BorrowFrom;
use std::collections::{HashSet, HashMap};
use std::collections::hash_set::Iter as HashSetIter;
use std::collections::hash_map::Iter as HashMapIter;

use ortho::OrthographicContext;

use xxhash::XXState;
use rustc_serialize::json::Json;

/// Represents already compiled data that is used by the PunktTrainer, 
/// and / or stores incrementally compiled data generated by a PunktTrainer. 
///
/// # Examples
///
/// Precompiled data can be loaded via a language specific constructor.
///
/// ```
/// use punkt::trainer::TrainingData;
///
/// let eng_data = TrainingData::english();
/// let ger_data = TrainingData::german();
/// ``` 
#[derive(Show)]
pub struct TrainingData {
  abbrev_types: HashSet<String, XXState>,
  collocations: HashMap<String, HashSet<String, XXState>, XXState>,
  sentence_starters: HashSet<String, XXState>,
  orthographic_context: HashMap<String, OrthographicContext, XXState>
}

// Macro for generating functions to load precompiled data.
macro_rules! preloaded_data(
  ($lang:ident, $file:expr) => (
    impl TrainingData {
      #[inline]
      pub fn $lang() -> TrainingData {
        FromStr::from_str(include_str!($file)).unwrap()
      }
    }
  )
);

preloaded_data!(czech, "data/czech.json");
preloaded_data!(danish, "data/danish.json");
preloaded_data!(dutch, "data/dutch.json");
preloaded_data!(english, "data/english.json");
preloaded_data!(estonian, "data/estonian.json");
preloaded_data!(finnish, "data/finnish.json");
preloaded_data!(french, "data/french.json");
preloaded_data!(german, "data/german.json");
preloaded_data!(greek, "data/greek.json");
preloaded_data!(italian, "data/italian.json");
preloaded_data!(norwegian, "data/norwegian.json");
preloaded_data!(polish, "data/polish.json");
preloaded_data!(portuguese, "data/portuguese.json");
preloaded_data!(slovene, "data/slovene.json");
preloaded_data!(spanish, "data/spanish.json");
preloaded_data!(swedish, "data/swedish.json");
preloaded_data!(turkish, "data/turkish.json");

impl TrainingData {
  /// Returns the inner representation of compiled abbreviation types.
  #[inline]
  fn abbrev_types(&self) -> &HashSet<String, XXState> {
    &self.abbrev_types
  }

  /// Returns the inner representation of compiled collocations types.
  #[inline]
  fn collocations(
    &self
  ) -> &HashMap<String, HashSet<String, XXState>, XXState> {
    &self.collocations
  }

  /// Returns the inner representation of compiled sentence starters.
  #[inline]
  fn sentence_starters(&self) -> &HashSet<String, XXState> {
    &self.sentence_starters
  }

  /// Returns the inner representation of compiled orthographic contexts for 
  /// corresponding keys (tokens).
  #[inline]
  fn orthographic_context(
    &self
  ) -> &HashMap<String, OrthographicContext, XXState> {
    &self.orthographic_context
  }

  /// Internal - returns a mutable reference to the inner container holding 
  /// abbreviation types.
  #[inline]
  fn mut_abbrev_types(&mut self) -> &mut HashSet<String, XXState> {
    &mut self.abbrev_types
  }

  /// Internal - returns a mutable reference to the inner container holding 
  /// collocations.
  #[inline]
  fn mut_collocations(
    &mut self
  ) -> &mut HashMap<String, HashSet<String, XXState>, XXState> {
    &mut self.collocations
  }

  /// Internal - returns a mutable reference to the inner container holding 
  /// sentence starters.
  #[inline]
  fn mut_sentence_starters(&mut self) -> &mut HashSet<String, XXState> {
    &mut self.sentence_starters
  }

  /// Internal - returns a mutable reference to the inner container holding 
  /// tokens and the orthographic context they are in.
  #[inline]
  fn mut_orthographic_context(
    &mut self
  ) -> &mut HashMap<String, OrthographicContext, XXState> {
    &mut self.orthographic_context
  }

  /// Insert an abbreviation if it doesn't already exist.
  #[inline]
  pub fn insert_abbrev(&mut self, abbrev: &str) -> bool {
    if !self.abbrev_types().contains(abbrev) {
      self.mut_abbrev_types().insert(abbrev.to_string())
    } else {
      false
    }
  }

  /// Insert a collocation if it doesn't already exist.
  #[inline]
  pub fn insert_collocation(&mut self, tok0: &str, tok1: &str) -> bool {
    if !self.collocations().contains_key(tok0) {
      self.mut_collocations().insert(
        tok0.to_string(), 
        HashSet::with_hash_state(XXState::new()));
    }

    self.mut_collocations().get_mut(tok0).unwrap().insert(tok1.to_string())
  }

  /// Insert a sentence starter if it doesn't already exist.
  #[inline]
  pub fn insert_sentence_starter(&mut self, tok: &str) -> bool {
    if !self.sentence_starters().contains(tok) {
      self.mut_sentence_starters().insert(tok.to_string())
    } else {
      false
    }
  }

  /// Insert a token and its orthographic context  if it doesn't already exist.
  #[inline]
  pub fn insert_orthographic_context(
    &mut self, 
    tok: &str, 
    ctxt: OrthographicContext
  ) -> bool {
    if !self.orthographic_context().contains_key(tok) {
      self.mut_orthographic_context().insert(tok.to_string(), ctxt).is_none()
    } else {
      false
    }
  }

  /// Clear all abbreviations.
  #[inline]
  pub fn clear_abbrevs(&mut self) {
    self.mut_abbrev_types().clear();
  }

  /// Clear all collocations.
  #[inline]
  pub fn clear_collocations(&mut self) {
    self.mut_collocations().clear();
  }

  /// Clear all sentence starters.
  #[inline]
  pub fn clear_sentence_starters(&mut self) {
    self.mut_sentence_starters().clear();
  }

  /// Clear all tokens and their orthographic contexts.
  #[inline]
  pub fn clear_orthographic_context(&mut self) {
    self.mut_orthographic_context().clear();
  }

  /// Remove an abbreviation by token key.
  #[inline]
  pub fn remove_abbrev<Q: ?Sized>(&mut self, abbrev: &Q) -> bool 
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.mut_abbrev_types().remove(abbrev)
  }

  /// Remove a collocation by left and right tokens.
  #[inline]
  pub fn remove_collocation<Q: ?Sized>(&mut self, tok0: &Q, tok1: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.mut_collocations().get_mut(tok0).map(|s| s.remove(tok1)).unwrap_or(false)
  }

  /// Remove a sentence starter by token key.
  #[inline]
  pub fn remove_sentence_starter<Q: ?Sized>(&mut self, tok: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.mut_sentence_starters().remove(tok)
  }

  /// Remove a token and its orthographic context by token key.
  #[inline]
  pub fn remove_orthographic_context<Q: ?Sized>(&mut self, tok: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.mut_orthographic_context().remove(tok).is_some()
  }

  /// Checks whether or not a collocation already exists by left and right 
  /// tokens.
  #[inline]
  pub fn contains_collocation<Q: ?Sized>(&self, tok0: &Q, tok1: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.collocations().get(tok0).map(|s| s.contains(tok1)).unwrap_or(false)
  }

  /// Checks whether or not a sentence starter already exists by token key.
  #[inline]
  pub fn contains_sentence_starter<Q: ?Sized>(&self, tok: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.sentence_starters().contains(tok)
  }

  /// Checks whether or not an abbreviation already exists by token key.
  #[inline]
  pub fn contains_abbrev<Q: ?Sized>(&self, tok: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.abbrev_types().contains(tok)
  }

  /// Checks if a token exists in the collection of tokens and their orthographic
  /// contexts.
  #[inline]
  pub fn contains_orthographic_context<Q: ?Sized>(&self, tok: &Q) -> bool
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.orthographic_context().contains_key(tok)
  }

  /// Gets a token's orthographic context by its key if it exists. Returns None if 
  /// it does not exist in the collection.
  #[inline]
  pub fn get_orthographic_context<Q: ?Sized>(
    &self, 
    tok: &Q
  ) -> Option<&OrthographicContext>
    where Q: Hash<XXState> + Eq + BorrowFrom<String>
  {
    self.orthographic_context().get(tok)
  }

  /// Returns the number of collocations.
  #[inline]
  pub fn collocations_len(&self) -> usize {
    self.collocations().iter().fold(0, |acc, (_, s)| acc + s.len())
  }

  /// Returns the number of abbreviations.
  #[inline]
  pub fn abbrevs_len(&self) -> usize {
    self.abbrev_types().len()
  }

  /// Returns the number of sentence starters.
  #[inline]
  pub fn sentence_starters_len(&self) -> usize {
    self.sentence_starters().len()
  }

  /// Returns the number of tokens in the orthographic context collection.
  #[inline]
  pub fn orthographic_context_len(&self) -> usize {
    self.orthographic_context().len()
  }

  /// Returns an iterator accross all the abbreviation types.
  #[inline]
  pub fn abbrevs_iter(&self) -> HashSetIter<String> {
    self.abbrev_types().iter()
  }

  /// Returns an iterator accross all the sentence starters.
  #[inline]
  pub fn sentence_starters_iter(&self) -> HashSetIter<String> {
    self.sentence_starters().iter()
  }

  /// Returns an iterator across all collocations.
  #[inline]
  pub fn collocations_iter(&self) -> CollocationsIterator {
    CollocationsIterator { iter: self.collocations().iter(), cur: None }
  }

  #[inline]
  pub fn orthographic_context_iter(&self) -> HashMapIter<String, u8> {
    self.orthographic_context().iter()
  }
}

impl Default for TrainingData {
  /// Returns a default TrainingData object with no data.
  fn default() -> TrainingData {
    TrainingData {
      abbrev_types: HashSet::with_hash_state(XXState::new()),
      collocations: HashMap::with_hash_state(XXState::new()),
      sentence_starters: HashSet::with_hash_state(XXState::new()),
      orthographic_context: HashMap::with_hash_state(XXState::new())
    }
  }
}

impl FromStr for TrainingData {
  /// Deserializes JSON and loads the data into a new TrainingData object.
  fn from_str(s: &str) -> Option<TrainingData> {
    match Json::from_str(s) {
      Ok(Json::Object(mut obj)) => {
        let mut data: TrainingData = Default::default();

        // Macro that gets a Json array by a path on the object. Then does a 
        // pattern match on a specified pattern, and runs a specified action.
        macro_rules! read_json_array_data(
          ($path:expr, $mtch:pat, $act:expr) => (
            match obj.remove($path) {
              Some(Json::Array(arr)) => {
                for x in arr.into_iter() {
                  match x {
                    $mtch => { $act; }
                        _ => ()
                  }
                }
              }
              _ => return None
            }
          );
        );

        read_json_array_data!(
          "abbrev_types", Json::String(st), data.mut_abbrev_types().insert(st));

        read_json_array_data!(
          "sentence_starters", Json::String(st), data.mut_sentence_starters().insert(st));

        // Load collocations, these come as an array with 2 members in them (or they should). 
        // Pop them in reverse order, then insert into the proper bucket. 
        read_json_array_data!("collocations", Json::Array(mut ar), {
          match (ar.pop(), ar.pop()) {
            (Some(Json::String(r)), Some(Json::String(l))) => {
              if !data.collocations().contains_key(l.as_slice()) {
                let mut bucket = HashSet::with_hash_state(XXState::new());

                bucket.insert(r);

                data.mut_collocations().insert(l, bucket);
              } else {
                data.mut_collocations().get_mut(l.as_slice()).unwrap().insert(r);
              }
            }
            _ => return None
          };
        });

        // Orthographic context is a Json object, where the word is the key, and the 
        // value is the context it is in. This value corresponds to a value in punkt::consts.
        match obj.remove("ortho_context") {
          Some(Json::Object(obj)) => {
            for (k, v) in obj.into_iter() {
              if v.is_u64() { 
                data.mut_orthographic_context().insert(k, v.as_u64().unwrap() as u8); 
              }
            }
          }
          _ => return None
        }

        Some(data)
      }
      _ => None
    }
  }
}

/// Iterator for collocations stored in TrainingData. Uses a recursive algorithm to 
/// generate next items from internal collocation collection in TrainingData.
pub struct CollocationsIterator<'a> {
  iter: HashMapIter<'a, String, HashSet<String, XXState>>,
  cur: Option<(&'a String, HashSetIter<'a String>)>
}

impl<'a> Iterator for CollocationsIterator<'a> {
  type Item = (&'a str, &'a str);

  #[inline]
  fn next<'b>(&'b mut self) -> Option<(&'a str, &'a str)> {
    let mut recurse = false;

    match self.cur {
      None => self.cur = self.iter.next().map(|(k, s)| (k, s.iter())),
         _ => ()
    };

    let res = match self.cur {
      Some((k, ref mut it)) => {
        match it.next() {
          Some(v) => Some((k.as_slice(), v.as_slice())),
          None    => { recurse = true; None }
        }
      }
      None => None
    };

    if recurse {
      self.cur = None;
      self.next()
    } else {
      res
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.iter.size_hint().0, None)
  }
}

#[test]
fn test_data_load_from_json_test() {
  let data: TrainingData = FromStr::from_str(include_str!("data/english.json")).unwrap();

  assert!(data.orthographic_context().len() > 0);
  assert!(data.abbrev_types().len() > 0);
  assert!(data.sentence_starters().len() > 0);
  assert!(data.collocations().len() > 0);
}
