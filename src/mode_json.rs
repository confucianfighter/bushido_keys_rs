use quote::quote;

pub fn get_json_str() -> String {
    quote! {
          {
          "modes": [
            {
              "name": "num_mode",
              "activation_keys": [
                "A",
                ";"
              ],
              "key_mapping": {
                "D": {
                  "key": "3",
                  "modifiers": []
                },
                ";": {
                  "key": "0",
                  "modifiers": []
                },
                "G": {
                  "key": "5",
                  "modifiers": []
                },
                "H": {
                  "key": "6",
                  "modifiers": []
                },
                "S": {
                  "key": "2",
                  "modifiers": []
                },
                "K": {
                  "key": "8",
                  "modifiers": []
                },
                "L": {
                  "key": "9",
                  "modifiers": []
                },
                "J": {
                  "key": "7",
                  "modifiers": []
                },
                "F": {
                  "key": "4",
                  "modifiers": []
                },
                "A": {
                  "key": "1",
                  "modifiers": []
                }
              }
            },
            {
              "name": "symbols",
              "activation_keys": [
                "S",
                "L"
              ],
              "key_mapping": {
                "H": {
                  "key": "6",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "D": {
                  "key": "3",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "G": {
                  "key": "5",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "K": {
                  "key": "8",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "L": {
                  "key": "9",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                ";": {
                  "key": "0",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "S": {
                  "key": "2",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "A": {
                  "key": "1",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "J": {
                  "key": "7",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "F": {
                  "key": "4",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "O": {
                  "key": "-",
                  "modifiers": []
                },
                "P": {
                  "key": "=",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "U": {
                  "key": "-",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "E": {
                  "key": "=",
                  "modifiers": []
                },
                "Q": {
                  "key": "BACKSPACE",
                  "modifiers": []
                },
                "T": {
                  "key": "`",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "B": {
                  "key": "`",
                  "modifiers": []
                },
                "'": {
                  "key": "\\",
                  "modifiers": []
                },
                "R": {
                  "key": "BACKSPACE",
                  "modifiers": [
                    "SHIFT"
                  ]
                }
              }
            },
            {
              "name": "delimeter",
              "activation_keys": [
                "D",
                "K"
              ],
              "key_mapping": {
                "H": {
                  "key": "|",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "D": {
                  "key": "<",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "G": {
                  "key": "|",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "K": {
                  "key": ">",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "L": {
                  "key": "]",
                  "modifiers": []
                },
                ";": {
                  "key": "}",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "S": {
                  "key": "[",
                  "modifiers": []
                },
                "A": {
                  "key": "{",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "J": {
                  "key": "0",
                  "modifiers": [
                    "SHIFT"
                  ]
                },
                "F": {
                  "key": "9",
                  "modifiers": [
                    "SHIFT"
                  ]
                }
              }
            },
            {
              "name": "vim arrow keys",
              "activation_keys": [
                "F"
              ],
              "key_mapping": {
                "H": {
                  "key": "LEFT",
                  "modifiers": []
                },
                "J": {
                  "key": "DOWN",
                  "modifiers": []
                },
                "K": {
                  "key": "UP",
                  "modifiers": []
                },
                "L": {
                  "key": "RIGHT",
                  "modifiers": []
                }
              }
            },{
              "name": "F-keys",
              "activation_keys": [
                "G",
                "H"
              ],
              "key_mapping": {
                "A": {
                  "key": "F1",
                  "modifiers": [

                  ]
                },
                "S": {
                  "key": "F2",
                  "modifiers": [

                  ]
                },
                "D": {
                  "key": "F3",
                  "modifiers": [

                  ]
                },
                "F": {
                  "key": "F4",
                  "modifiers": [

                  ]
                },
                "G": {
                  "key": "F5",
                  "modifiers": [

                  ]
                },
                "H": {
                  "key": "F6",
                  "modifiers": [

                  ]
                },
                "J": {
                  "key": "F7",
                  "modifiers": [

                  ]
                },
                "K": {
                  "key": "F8",
                  "modifiers": [

                  ]
                },
                "L": {
                  "key": "F9",
                  "modifiers": [

                  ]
                },
                ";": {
                  "key": "F10",
                  "modifiers": [

                  ]
                },
                "'": {
                  "key": "F11",
                  "modifiers": []
                },
                "ENTER": {
                  "key": "F12",
                  "modifiers": [

                  ]
                }
              }
            }
          ]
        }
    }
    .to_string()
}
