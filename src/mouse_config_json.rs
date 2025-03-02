use quote::quote;

pub fn get_json_str() -> String {
    quote! {
        {
            "fast_up_key": "W",
            "fast_down_key": "S",
            "fast_left_key": "A",
            "fast_right_key": "D",
            "slow_up_key": "O",
            "slow_down_key": "L",
            "slow_left_key": "K",
            "slow_right_key": ";",
            "fast_acceleration": 1500.0,
            "slow_acceleration": 500.0,
            "friction": 0.99,
            "max_speed": 2000.0,
            "fps": 60.0,
            "left_click_key": "Q",
            "right_click_key": "E",
            "middle_click_key": "M",
            "dual_wield_multiplier": 0.5,
            "activation_keys": [
                   " "
            ]
        }
    }
    .to_string()
}
