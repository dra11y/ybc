use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, Storage};
use ybc::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BulmaTheme {
    pub name: String,
    pub background: String,
    pub text: String,
    pub link: String,
    pub primary: String,
    pub info: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
}

impl Default for BulmaTheme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            background: "#ffffff".to_string(),
            text: "#4a4a4a".to_string(),
            link: "#3273dc".to_string(),
            primary: "#00d1b2".to_string(),
            info: "#3273dc".to_string(),
            success: "#23d160".to_string(),
            warning: "#ffdd57".to_string(),
            danger: "#ff3860".to_string(),
        }
    }
}

impl BulmaTheme {
    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: "#0a0a0a".to_string(),
            text: "#b5b5b5".to_string(),
            link: "#3273dc".to_string(),
            primary: "#00d1b2".to_string(),
            info: "#3273dc".to_string(),
            success: "#23d160".to_string(),
            warning: "#ffdd57".to_string(),
            danger: "#ff3860".to_string(),
        }
    }

    pub fn cerulean() -> Self {
        Self {
            name: "Cerulean".to_string(),
            background: "#ffffff".to_string(),
            text: "#333333".to_string(),
            link: "#2fa4e7".to_string(),
            primary: "#2fa4e7".to_string(),
            info: "#033c73".to_string(),
            success: "#73a839".to_string(),
            warning: "#dd5600".to_string(),
            danger: "#c71c22".to_string(),
        }
    }

    pub fn colors(&self) -> [(&str, &str); 8] {
        [
            ("Background", &self.background),
            ("Text", &self.text),
            ("Link", &self.link),
            ("Primary", &self.primary),
            ("Info", &self.info),
            ("Success", &self.success),
            ("Warning", &self.warning),
            ("Danger", &self.danger),
        ]
    }

    pub fn set_color(&mut self, index: usize, color: String) {
        match index {
            0 => self.background = color,
            1 => self.text = color,
            2 => self.link = color,
            3 => self.primary = color,
            4 => self.info = color,
            5 => self.success = color,
            6 => self.warning = color,
            7 => self.danger = color,
            _ => {}
        }
    }
}

fn get_local_storage() -> Option<Storage> {
    web_sys::window()?.local_storage().ok()?
}

fn save_themes_to_storage(themes: &HashMap<String, BulmaTheme>) {
    if let Some(storage) = get_local_storage() {
        if let Ok(json) = serde_json::to_string(themes) {
            let _ = storage.set_item("bulma_themes", &json);
        }
    }
}

fn load_themes_from_storage() -> HashMap<String, BulmaTheme> {
    let mut themes = HashMap::new();

    // Add default themes
    let default = BulmaTheme::default();
    themes.insert(default.name.clone(), default);

    let dark = BulmaTheme::dark();
    themes.insert(dark.name.clone(), dark);

    let cerulean = BulmaTheme::cerulean();
    themes.insert(cerulean.name.clone(), cerulean);

    // Load saved themes
    if let Some(storage) = get_local_storage() {
        if let Ok(Some(json)) = storage.get_item("bulma_themes") {
            if let Ok(saved_themes) = serde_json::from_str::<HashMap<String, BulmaTheme>>(&json) {
                themes.extend(saved_themes);
            }
        }
    }

    themes
}

#[function_component(ThemePicker)]
pub fn theme_picker() -> Html {
    let themes = use_state(|| load_themes_from_storage());
    let current_theme = use_state(|| BulmaTheme::default());
    let selected_theme_name = use_state(|| "Default".to_string());
    let theme_name_input = use_state(|| "New Theme".to_string());

    let on_theme_select = {
        let themes = themes.clone();
        let current_theme = current_theme.clone();
        let selected_theme_name = selected_theme_name.clone();
        let theme_name_input = theme_name_input.clone();
        Callback::from(move |theme_name: String| {
            if let Some(theme) = themes.get(&theme_name) {
                current_theme.set(theme.clone());
                selected_theme_name.set(theme_name.clone());
                theme_name_input.set(theme_name);
            }
        })
    };

    let on_color_change = {
        let current_theme = current_theme.clone();
        Callback::from(move |(index, color): (usize, String)| {
            let mut theme = (*current_theme).clone();
            theme.set_color(index, color);
            current_theme.set(theme);
        })
    };

    let on_name_input = {
        let theme_name_input = theme_name_input.clone();
        Callback::from(move |value: String| {
            theme_name_input.set(value);
        })
    };

    let on_add_theme = {
        let themes = themes.clone();
        let current_theme = current_theme.clone();
        let theme_name_input = theme_name_input.clone();
        let selected_theme_name = selected_theme_name.clone();
        Callback::from(move |_| {
            let mut new_themes = (*themes).clone();
            let mut new_theme = (*current_theme).clone();
            new_theme.name = (*theme_name_input).clone();
            new_themes.insert(new_theme.name.clone(), new_theme.clone());
            save_themes_to_storage(&new_themes);
            themes.set(new_themes);
            selected_theme_name.set(new_theme.name.clone());
            theme_name_input.set(format!("{} Copy", new_theme.name));
        })
    };

    let on_save_theme = {
        let themes = themes.clone();
        let current_theme = current_theme.clone();
        let theme_name_input = theme_name_input.clone();
        let selected_theme_name = selected_theme_name.clone();
        Callback::from(move |_| {
            let mut new_themes = (*themes).clone();
            let mut updated_theme = (*current_theme).clone();
            updated_theme.name = (*theme_name_input).clone();

            // Remove old theme if name changed
            if *selected_theme_name != updated_theme.name {
                new_themes.remove(&*selected_theme_name);
            }

            new_themes.insert(updated_theme.name.clone(), updated_theme.clone());
            save_themes_to_storage(&new_themes);
            themes.set(new_themes);
            selected_theme_name.set(updated_theme.name.clone());
        })
    };

    let on_delete_theme = {
        let themes = themes.clone();
        let selected_theme_name = selected_theme_name.clone();
        let current_theme = current_theme.clone();
        let theme_name_input = theme_name_input.clone();
        Callback::from(move |_| {
            let mut new_themes = (*themes).clone();
            new_themes.remove(&*selected_theme_name);
            save_themes_to_storage(&new_themes);

            // Switch to default theme
            let default_theme = BulmaTheme::default();
            current_theme.set(default_theme.clone());
            selected_theme_name.set(default_theme.name.clone());
            theme_name_input.set(default_theme.name.clone());
            themes.set(new_themes);
        })
    };

    html! {<Card><CardContent><Columns>
        <Column>
            <Title tag="h3" size={HeaderSize::Is5}>{"Theme Picker"}</Title>

            // Theme selector dropdown
            <Field>
                <Control>
                    <Select
                        name="theme-selector"
                        value={(*selected_theme_name).clone()}
                        update={on_theme_select}
                    >
                        { for themes.iter().map(|(name, _)| html! {
                            <option value={name.clone()}>{name}</option>
                        })}
                    </Select>
                </Control>
            </Field>

            // Theme name input
            <Field>
                <Control>
                    <Input
                        name="theme-name"
                        placeholder="Theme name"
                        value={(*theme_name_input).clone()}
                        update={on_name_input}
                    />
                </Control>
            </Field>

            // Control buttons
            <Field classes="is-grouped">
                <Control>
                    <Button classes="is-small is-primary" onclick={on_add_theme}>
                        <span class="icon is-small">
                            <i class="fas fa-plus"></i>
                        </span>
                    </Button>
                </Control>
                <Control>
                    <Button classes="is-small is-info" onclick={on_save_theme}>
                        <span class="icon is-small">
                            <i class="fas fa-save"></i>
                        </span>
                    </Button>
                </Control>
                <Control>
                    <Button
                        classes="is-small is-danger"
                        onclick={on_delete_theme}
                        disabled={["Default", "Dark", "Cerulean"].contains(&selected_theme_name.as_str())}
                    >
                        <span class="icon is-small">
                            <i class="fas fa-trash"></i>
                        </span>
                    </Button>
                </Control>
            </Field>

            // Color swatches
            <div class="mt-4">
                { for current_theme.colors().iter().enumerate().map(|(index, (label, color))| {
                    let color = color.to_string();
                    let onchange = {
                        let on_color_change = on_color_change.clone();
                        Callback::from(move |e: Event| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            on_color_change.emit((index, target.value()));
                        })
                    };

                    html! {
                        <Field classes="mb-2">
                            <div class="is-flex is-align-items-center">
                                <div class="is-flex-grow-1">
                                    <Columns>
                                        <Column>
                                            <label class="label is-small">{label}</label>
                                        </Column>
                                        <Column classes="is-size-7 has-text-right has-text-grey">
                                            {color.to_uppercase()}
                                        </Column>
                                    </Columns>
                                    <div
                                        class="button is-fullwidth"
                                        style={format!("background-color: {color}; border: 1px solid #dbdbdb; height: 2rem;")}
                                    >
                                    <ButtonInput
                                        r#type={ButtonInputType::Color { onchange, hidden: true }}
                                        value={color.clone()}
                                        />
                                    </div>
                                </div>
                            </div>
                        </Field>
                    }
                }) }
            </div>
        </Column>
    </Columns></CardContent></Card>}
}
