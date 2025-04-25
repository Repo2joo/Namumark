use crate::{toskens::Tokens, Compiler};

impl Compiler {
    pub fn lex (&mut self) {
        self.chars = self.string.chars().collect();
          let mut literal_temp = String::new();
          loop {
              let curr_string = match self.curr_string() {
                  Some(char) => {char},
                  None => {
                      self.tokens.push(Tokens::Literal(literal_temp));
                      break;
                  }
              };
              match curr_string {
                  '|' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      match self.nexto(1) {
                          '|' => {
                              self.tokens.push(Tokens::TablePipe);
                              self.inc();
                          },
                          _ => {
                              self.tokens.push(Tokens::PipeLine);
                          }
                      }
                      self.inc();
                  },
                  '=' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone())));
                      literal_temp.clear();
                      let mut header_count = 1;
                      for _ in 1..6 {
                          self.inc();
                          match self.nexto(0) {
                              '=' => {
                                  header_count += 1;
                              },
                              _ => {
                                  break;
                              }
                          }
                      }
                      self.tokens.push(Tokens::Header(header_count))
                  },
                  '\'' => { //count 1, i 2, idx curr+2, nexto curr +2, count 3
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone())));
                      literal_temp.clear();
                      let mut count = 1;
                      for i in 1..=5 {
                          self.inc();
                          if i != 5 {
                              match self.nexto(0) {
                                  '\'' => {
                                      count += 1;
                                  },
                                  _ => {
                                      if count == 3 || count == 4 {
                                          self.tokens.push(Tokens::Bold);
                                          if count == 4 {
                                              self.tokens.push(Tokens::Literal(String::from("'")))
                                          }
                                      } else {
                                          if count == 5 {
                                              self.tokens.push(Tokens::BoldItalic)
                                          } else if count == 1 {
                                              self.tokens.push(Tokens::Literal(String::from("'")))
                                          } else {
                                              self.tokens.push(Tokens::Italic)
                                          }
                                      }
                                      break;
                                  }
                              }
                          } else {
                              self.tokens.push(Tokens::BoldItalic)
                          }
                      }
                          
                  },
                  '_' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      if self.nexto(1) == '_' {
                          self.tokens.push(Tokens::UnderLine);
                          self.inc();
                      } else {
                          self.tokens.push(Tokens::Literal(String::from("_")));
                      }
                      self.inc();
                  },
                  '~' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      if self.nexto(1) == '~' {
                          self.tokens.push(Tokens::DeletedWave);
                          self.inc();
                      } else {
                          self.tokens.push(Tokens::Literal(String::from("~")));
                      }
                      self.inc();
                  },
                  '-' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      if self.nexto(1) == '-' {
                          self.tokens.push(Tokens::DeletedBar);
                          self.inc();
                      } else {
                          self.tokens.push(Tokens::Literal(String::from("-")));
                      }
                      self.inc();
                  },
                  '^' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      if self.nexto(1) == '^' {
                          self.tokens.push(Tokens::Sup);
                          self.inc();
                      } else {
                          self.tokens.push(Tokens::Literal(String::from("^")));
                      }
                      self.inc();
                  },
                  ',' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      if self.nexto(1) == ',' {
                          self.tokens.push(Tokens::Sub);
                          self.inc();
                      } else {
                          self.tokens.push(Tokens::Literal(String::from(",")));
                      }
                      self.inc();
                  },
                  '{' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      let mut count = 1;
                      for _ in 1..3 {
                          self.inc();
                          match self.nexto(0) {
                              '{' => {
                                  count += 1;
                                  if count == 3 {
                                      self.tokens.push(Tokens::TripleOpen);
                                  } 
                              },
                              _ => {
                                  if count == 2 {
                                      self.tokens.push(Tokens::Literal(String::from("{{")));
                                  } else {
                                      self.tokens.push(Tokens::Literal(String::from("{")));
                                  }
                                  break;
                              }
                          }
                      }
                      self.inc();
                  },
                  '}' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      let mut count = 1;
                      for _ in 1..3 {
                          self.inc();
                          match self.nexto(0) {
                              '}' => {
                                  count += 1;
                                  if count == 3 {
                                      self.tokens.push(Tokens::TripleClose);
                                  } 
                              },
                              _ => {
                                  if count == 2 {
                                      self.tokens.push(Tokens::Literal(String::from("}}")));
                                  } else {
                                      self.tokens.push(Tokens::Literal(String::from("}")));
                                  }
                                  break;
                              }
                          }
                      }
                      self.inc();
                  },
                  '[' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      match self.nexto(0) {
                          '[' => {
                              self.tokens.push(Tokens::LinkOpen);
                              self.inc();
                          }
                          _ => {
                              match self.nexto(0) {
                                  '*' => {
                                      self.tokens.push(Tokens::Reference);
                                      self.inc();
                                  }
                                  _ => {self.tokens.push(Tokens::MacroOpen)}
                              }
                          }
                      }
                  },
                  ']' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      match self.nexto(0) {
                          ']' => {
                              self.tokens.push(Tokens::LinkClose);
                              self.inc();
                          }
                          _ => {self.tokens.push(Tokens::MacroClose)}
                      }
                  },
                  '#' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      match self.nexto(0) {
                          '#' => {
                              self.tokens.push(Tokens::Comment);
                              self.inc();
                          }
                          _ => {self.tokens.push(Tokens::MacroClose)}
                      }
                  },
                  '@' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      match self.nexto(0) {
                          '#' => {
                              self.inc();
                              match self.nexto(0) {
                                  '#' => {
                                      self.tokens.push(Tokens::FixedComment);
                                      self.inc();
                                  }
                                  _ => {
                                      self.tokens.push(Tokens::Literal(String::from("@#")));
                                  }
                              }
                          }
                          _ => {self.tokens.push(Tokens::Sharp)}
                      }
                  },
                  ' ' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      self.tokens.push(Tokens::Space);
                  },
                  '\n' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      self.tokens.push(Tokens::NewLine);
                  },
                  ')' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      self.tokens.push(Tokens::Happy);
                  },
                  '(' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      self.tokens.push(Tokens::Sad);
                  },
                  '\\' => {
                      self.tokens.push(Tokens::Literal(String::from(literal_temp.clone()))); //literal_tamp를 그대로 넣어버리면 clear시 같이 삭제댐 타입 특성상
                      literal_temp.clear();
                      self.inc();
                      self.tokens.push(Tokens::Escape(self.nexto(0)));
                      self.inc();
                  }
                  last => {
                      self.inc();
                      literal_temp.push(last);
                  }
              }
          }
          self.tokens.retain(|x| match x {
              Tokens::Literal(string) => {
                  string != ""
              },
              _ => {true}
          });
          return ();
      }
      fn curr_string(&self) -> Option<char> {
        return self.chars.get(self.idx).copied();
    }
    fn nexto(&self, how_much:usize) -> char {
        return self.chars.get(self.idx+how_much).copied().unwrap_or_default();
    }
    fn inc (&mut self) {
        self.idx += 1;
    }
}