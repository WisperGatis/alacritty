#[cfg(test)]
mod tests {
    use super::super::bindings::*;
    use winit::keyboard::ModifiersState;

    #[test]
    fn test_linux_tab_bindings_exist() {
        // Test that Linux tabbing bindings are included in common_keybindings
        let bindings = super::common_keybindings();
        
        // Check for CreateNewTab binding
        let create_tab_binding = bindings.iter().find(|b| {
            b.action == Action::CreateNewTab && 
            b.mods == (ModifiersState::CONTROL | ModifiersState::SHIFT) &&
            b.trigger == super::BindingKey::Keycode { 
                key: winit::keyboard::Key::Character("t".into()), 
                location: super::KeyLocation::Any 
            }
        });
        assert!(create_tab_binding.is_some(), "CreateNewTab binding not found");
        
        // Check for SelectNextTab binding
        let next_tab_binding = bindings.iter().find(|b| {
            b.action == Action::SelectNextTab && 
            b.mods == (ModifiersState::CONTROL | ModifiersState::SHIFT) &&
            b.trigger == super::BindingKey::Keycode { 
                key: winit::keyboard::Key::Character("]".into()), 
                location: super::KeyLocation::Any 
            }
        });
        assert!(next_tab_binding.is_some(), "SelectNextTab binding not found");
        
        // Check for SelectPreviousTab binding
        let prev_tab_binding = bindings.iter().find(|b| {
            b.action == Action::SelectPreviousTab && 
            b.mods == (ModifiersState::CONTROL | ModifiersState::SHIFT) &&
            b.trigger == super::BindingKey::Keycode { 
                key: winit::keyboard::Key::Character("[".into()), 
                location: super::KeyLocation::Any 
            }
        });
        assert!(prev_tab_binding.is_some(), "SelectPreviousTab binding not found");
        
        // Check for SelectTab1-9 bindings
        for i in 1..=9 {
            let binding = bindings.iter().find(|b| {
                let action = match i {
                    1 => Action::SelectTab1,
                    2 => Action::SelectTab2,
                    3 => Action::SelectTab3,
                    4 => Action::SelectTab4,
                    5 => Action::SelectTab5,
                    6 => Action::SelectTab6,
                    7 => Action::SelectTab7,
                    8 => Action::SelectTab8,
                    9 => Action::SelectLastTab,
                    _ => unreachable!(),
                };
                
                b.action == action && 
                b.mods == ModifiersState::ALT &&
                b.trigger == super::BindingKey::Keycode { 
                    key: winit::keyboard::Key::Character(i.to_string().into()), 
                    location: super::KeyLocation::Any 
                }
            });
            assert!(binding.is_some(), "SelectTab{} binding not found", i);
        }
    }
}