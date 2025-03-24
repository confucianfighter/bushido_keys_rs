use quote::quote;

pub fn get_json_str() -> String {
    quote! {{
      "fast_up_key": "W",
      "fast_down_key": "S",
      "fast_left_key": "A",
      "fast_right_key": "D",
      "slow_up_key": "O",
      "slow_down_key": "L",
      "slow_left_key": "K",
      "slow_right_key": ";",
      "fast_acceleration": 4000.0,
      "slow_acceleration": 1000.0,
      "friction": 0.87,
      "max_speed": 2000.0,
      "fps": 60.0,
      "left_click_key": "Q",
      "right_click_key": "E",
      "middle_click_key": "M",
      "scroll_left_key": "H",
      "scroll_right_key": "'",
      "scroll_up_key": "I",
      "scroll_down_key": "J",
      "dual_wield_multiplier": 2.0,
      "activation_keys": [
        " "
      ],
      "scroll_acceleration": 700.0,
      "scroll_friction": 0.87,
      "scroll_max_speed": 1000.0,
      "auto_modifiers": []
    }    }
    .to_string()
}
