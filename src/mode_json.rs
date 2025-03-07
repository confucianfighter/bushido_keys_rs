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
                        "A": {
                            "key": "1"
                        },
                        "S": {
                            "key": "2"
                        },
                        "D": {
                            "key": "3"
                        },
                        "F": {
                            "key": "4"
                        },
                        "G": {
                            "key": "5"
                        },
                        "H": {
                            "key": "6"
                        },
                        "J": {
                            "key": "7"
                        },
                        "K": {
                            "key": "8"
                        },
                        "L": {
                            "key": "9"
                        },
                        ";": {
                            "key": "0"
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
                        "A": {
                            "key": "!",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "S": {
                            "key": "@",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "D": {
                            "key": "#",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "F": {
                            "key": "$",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "G": {
                            "key": "%",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "H": {
                            "key": "^",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "J": {
                            "key": "&",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "K": {
                            "key": "*",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        "L": {
                            "key": "(",
                            "modifiers": [
                                "SHIFT"
                            ]
                        },
                        ";": {
                            "key": ")",
                            "modifiers": [
                                "SHIFT"
                            ]
                        }
                    }
                }
            ]
        }
    }.to_string()
}
