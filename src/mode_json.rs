use quote::quote;

pub fn get_json_str() -> String {
    quote! {{
      "modes": [
        {
          "name": "num_mode",
          "activation_keys": [
            "A",
            ";"
          ],
          "auto_modifiers": [

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
          "auto_modifiers": [

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
          "auto_modifiers": [

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
            "F",
            "J"
          ],
          "auto_modifiers": [

          ],
          "key_mapping": {

            "C": {
              "key": "C",
              "modifiers": ["CTRL"]
            },
            "V": {
              "key": "V",
              "modifiers": ["RWIN"]
            },
            "P": {
              "key": "P",
              "modifiers": ["CTRL"]
            },
            "K": {
              "key": "LEFT",
              "modifiers": []
            },
            "L": {
              "key": "DOWN",
              "modifiers": []
            },
            "O": {
              "key": "UP",
              "modifiers": []
            },
            ";": {
              "key": "RIGHT",
              "modifiers": []
            },
            "I": {
              "key": "PAGEUP",
              "modifiers": []
            },
            "J": {
              "key": "PAGEDOWN",
              "modifiers": []
            },
            "CAPSLOCK": {
              "key": "HOME",
              "modifiers": []
            },
            "G": {
              "key": "END",
              "modifiers": []
            },
            "R": {
              "key": "PAGEUP",
              "modifiers": []

            },
            "F": {
              "key": "PAGEDOWN",
              "modifiers": []
            },
            "W": {
              "key": "UP",
              "modifiers": []
            },
            "A": {
              "key": "LEFT",
              "modifiers": []
            },
            "S": {
              "key": "DOWN",
              "modifiers": []
            },
            "D": {
              "key": "RIGHT",
              "modifiers": []
            },
            "'": {
              "key": "END",
              "modifiers": []
            },
            "H": {
              "key": "HOME",
              "modifiers": []
            }
          }
        },
        {
          "name": "F-Keys",
          "activation_keys": [
            "G",
            "H"
          ],
          "auto_modifiers": [

          ],
          "key_mapping": {
            "A": {
              "key": "F1",
              "modifiers": []
            },
            "S": {
              "key": "F2",
              "modifiers": []
            },
            "D": {
              "key": "F3",
              "modifiers": []
            },
            "F": {
              "key": "F4",
              "modifiers": []
            },
            "G": {
              "key": "F5",
              "modifiers": []
            },
            "H": {
              "key": "F6",
              "modifiers": []
            },
            "J": {
              "key": "F7",
              "modifiers": []
            },
            "K": {
              "key": "F8",
              "modifiers": []
            },
            "L": {
              "key": "F9",
              "modifiers": []
            },
            ";": {
              "key": "F10",
              "modifiers": []
            },
            "'": {
              "key": "F11",
              "modifiers": []
            },
            "ENTER": {
              "key": "F12",
              "modifiers": []
            },
            "Q": {
              "key": "ESC",
              "modifiers": []
            }
          }
        },

        {
          "name": "CTRL",
          "activation_keys": [
            "Q",
            "P"
          ],
          "auto_modifiers": [
            "CTRL"
          ],
          "key_mapping": {

          }
        }
      ]
    }    }
    .to_string()
}
