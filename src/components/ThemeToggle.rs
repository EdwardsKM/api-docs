use dioxus::prelude::*;
#[derive(Clone, Copy, Debug)]
enum Theme {
    Dark,
    Light,
}

impl Theme {
    fn next(&self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
}

impl std::str::FromStr for Theme {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dark" => Ok(Theme::Dark),
            _ => Ok(Theme::Light),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Theme::Dark => write!(f, "dark"),
            Theme::Light => write!(f, "light"),
        }
    }
}

#[component]
pub fn ThemeToggle() -> Element {
    let mut theme = use_signal(|| Theme::Light);

    let toggle_theme = match theme.read().clone() {
        Theme::Dark => {
            eval(
                "
                    localStorage.setItem('data-theme', 'light');
                    document.documentElement.setAttribute('data-theme', 'light');
                ",
            );
            theme().next()
        }
        _ => {
            eval(
                "
                    localStorage.setItem('data-theme', 'dark');
                    document.documentElement.setAttribute('data-theme', 'dark');
                ",
            );
            theme().next()
        }
    };

    let future = use_resource(move || async move {
        let mut eval = eval(&format!(
            r#"
                dioxus.send(window.localStorage.getItem('data-theme'));
            "#
        ));
        let res = eval.recv().await.unwrap();
        res
    });

    // match future.value().as_ref() {
    //     Some(v) => theme.set(v.clone().as_str()?.parse::<Theme>().unwrap()),
    //     _ => {}
    // }

    rsx! {
        button {
            id : "theme__toggle",
            class : "button",
            "aria-label" : format!("Change to {theme} theme"),
            "theme-state" : "{theme}",
                onclick: move |_| {
                theme.set(toggle_theme);
            },
                svg {
                    version: "1.1",
                    id: "themetoggleicon",
                    xmlns: "http://www.w3.org/2000/svg",
                    x: "0",
                    y: "0",
                    class: "theme-toggle",
                    style: "vertical-align: middle; fill: currentColor; height : 20px; width : 20px;",
                    view_box: "0 0 27 27",

                    path {
                        class: "toggle__sun",
                        "theme-state" : "{theme}",
                        d: "m13.5 0.10497-3.9194 3.9194h-5.5407v5.5407l-3.9194 3.9194 3.9194 3.9194v5.5407h5.5407l3.9194 3.9194 3.9194-3.9194h5.5407v-5.5407l3.9194-3.9194-3.9194-3.9194v-5.5407h-5.5407zm0 4.5313a8.849 8.849 0 0 1 8.8481 8.8481 8.849 8.849 0 0 1-8.8481 8.8481 8.849 8.849 0 0 1-8.8481-8.8481 8.849 8.849 0 0 1 8.8481-8.8481z"
                    }
                    circle {
                        class: "toggle__circle",
                        "theme-state": "{theme}",
                        cx: "13.5",
                        cy: "13.5",
                        r: "7.3404"
                    }
                }
            }
    }
}
