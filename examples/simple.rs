mod msgs {
    pub enum Curl {
        Url(String),
        Body(String),
        Responce(String),
        Run,
    }

    pub enum Pictures {
        Add(Vec<String>),
        Idx(usize),
        Scale(f64),
        Del(bool),
        Shift(bool),
    }

    pub enum Sudoku {
        Push(usize, usize, i32),
        Solve,
        Clear,
    }

    pub enum Dialect {
        Run,
        Quit,
        Switch,
        Source(String),
        Target(String),
        SaveAs(String),
        Open(String),
        To(i32),
        From(i32),
        Lang(Vec<(String, String)>),
    }
}

mod models {
    #[derive(Default)]
    pub struct Calculator {
        pub prev: f64,
        pub operation: String,
        pub current: String,
        pub output: String,
    }
    impl Calculator {
        pub fn click(&mut self, value: &str) {
            match value {
                "/" | "x" | "+" | "-" | "%" => {
                    if self.current != "0" {
                        if self.operation.is_empty() {
                            self.prev = self.current.parse().unwrap_or(0.0);
                        } else {
                            self.equil();
                        }
                        self.output.push_str(&format!("{} {}", self.prev, value));
                        self.operation = value.to_string();
                        self.current = String::from("0");
                    }
                }
                "=" => {
                    if !self.operation.is_empty() {
                        self.equil();
                        self.operation.clear();
                    }
                }
                "CE" => {
                    self.output.clear();
                    self.operation.clear();
                    self.current = String::from("0");
                    self.prev = 0f64;
                }
                "@<-" => {
                    let label = self.current.clone();
                    self.current = if label.len() > 1 {
                        String::from(&label[..label.len() - 1])
                    } else {
                        String::from("0")
                    };
                }
                "C" => self.current = String::from("0"),
                "." => {
                    if !self.current.contains('.') {
                        self.current.push('.');
                    }
                }
                _ => {
                    if self.current == "0" {
                        self.current.clear();
                    }
                    self.current.push_str(value);
                }
            };
        }
        fn equil(&mut self) {
            self.output.push_str(&format!(" {}\n", self.current));
            let current: f64 = self.current.parse().unwrap_or(0.0);
            self.prev = match self.operation.as_str() {
                "/" => self.prev / current,
                "x" => self.prev * current,
                "+" => self.prev + current,
                "-" => self.prev - current,
                _ => self.prev / 100.0 * current,
            };
            self.output.push_str(&format!("    = {}\n", self.prev));
            self.current = String::from("0");
        }
    }

    #[derive(Default)]
    pub struct Pictures {
        list: Vec<String>,
        pub idx: usize,
        pub scale: f64,
    }
    impl Pictures {
        pub fn shift(&mut self, dir: bool) {
            let step: isize = match dir {
                true => 1,
                false => -1,
            };
            let next: isize = self.idx as isize + step;
            self.idx = match (0..self.list.len()).contains(&(next as usize)) {
                true => next,
                false => self.list.len() as isize - next * step,
            } as usize;
        }
        pub fn del(&mut self, value: bool) {
            if let Some(path) = self.path() {
                if value {
                    if std::fs::remove_file(path).is_ok() {
                        self.list.remove(self.idx);
                    }
                } else {
                    self.list.remove(self.idx);
                };
                self.shift(true);
            }
        }
        pub fn path(&self) -> Option<String> {
            match self.list.is_empty() {
                true => None,
                false => Some(self.list[self.idx].clone()),
            }
        }
        pub fn set_list(&mut self, value: Vec<String>) {
            for item in value {
                self.list.push(item);
            }
            self.idx = 0;
        }
        pub fn list(&self) -> Vec<&str> {
            self.list.iter().map(|x| x.as_str()).collect()
        }
    }

    #[derive(Default)]
    pub struct Sudoku(pub [[i32; 9]; 9]);
    impl Sudoku {
        pub fn clear(&mut self) {
            self.0 = [[0; 9]; 9];
        }
        fn check_solvable(&mut self) -> bool {
            let mut items: [i32; 9];
            for row in self.0 {
                items = [0; 9];
                for value in row {
                    if value > 0 && value < 10 {
                        items[(value - 1) as usize] += 1;
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
            for i in 0..9 {
                items = [0; 9];
                for row in self.0 {
                    if row[i] > 0 && row[i] < 10 {
                        items[(row[i] - 1) as usize] += 1;
                    }
                }
                if items.iter().any(|&n| n > 1) {
                    return false;
                }
            }
            for &x in [0, 3, 6].iter() {
                for &y in [0, 3, 6].iter() {
                    items = [0; 9];
                    for i in 0..3 {
                        for j in 0..3 {
                            if self.0[y + i][x + j] > 0 && self.0[y + i][x + j] < 10 {
                                items[(self.0[y + i][x + j] - 1) as usize] += 1;
                            }
                        }
                    }
                    if items.iter().any(|&n| n > 1) {
                        return false;
                    }
                }
            }
            true
        }
        fn check_possible(&self, y: usize, x: usize, number: i32) -> bool {
            if self.0[y].contains(&number) {
                return false;
            }
            if self.0.iter().any(|n| n[x] == number) {
                return false;
            }
            let x0: usize = (x / 3) * 3;
            let y0: usize = (y / 3) * 3;
            for i in 0..3 {
                for j in 0..3 {
                    if self.0[y0 + i][x0 + j] == number {
                        return false;
                    }
                }
            }
            true
        }
        fn find_next_cell2fill(&self) -> (usize, usize) {
            for (x, row) in self.0.iter().enumerate() {
                for (y, &val) in row.iter().enumerate() {
                    if val == 0 {
                        return (x, y);
                    }
                }
            }
            (99, 99)
        }
        fn solve(&mut self) -> bool {
            let (i, j) = self.find_next_cell2fill();
            if i == 99 {
                return true;
            }
            for e in 1..10 {
                if self.check_possible(i, j, e) {
                    self.0[i][j] = e;
                    if self.solve() {
                        return true;
                    }
                    self.0[i][j] = 0;
                }
            }
            false
        }
        pub fn answer(&mut self) {
            if self.check_solvable() {
                self.solve();
            } else {
                self.clear();
            }
        }
    }

    #[derive(Default)]
    pub struct Dialect {
        pub lang: Vec<(String, String)>,
        pub source: String,
        pub target: String,
        pub from: i32,
        pub to: i32,
    }

    impl Dialect {
        const SERVICE: &str = r#"https://lingva.ml/api/v1"#;
        const NAME: &str = "Dialect";
        pub fn read(&mut self, value: Vec<(String, String)>) {
            self.lang = value;
            if let Ok(value) = std::fs::read(Self::file())
                && value.len() >= 2
            {
                self.from = value[0] as i32;
                self.to = value[1] as i32;
            }
            let last = self.lang.len().saturating_sub(1) as i32;
            self.from = self.from.clamp(0, last);
            self.to = self.to.clamp(0, last);
        }
        fn file() -> std::path::PathBuf {
            #[cfg(windows)]
            let root = std::env::var_os("APPDATA")
                .map(std::path::PathBuf::from)
                .or_else(|| {
                    std::env::var_os("USERPROFILE")
                        .map(|home| std::path::PathBuf::from(home).join("AppData/Roaming"))
                });
            #[cfg(not(windows))]
            let root = std::env::var_os("XDG_CONFIG_HOME")
                .map(std::path::PathBuf::from)
                .or_else(|| {
                    std::env::var_os("HOME")
                        .map(|home| std::path::PathBuf::from(home).join(".config"))
                });
            root.unwrap_or_else(|| {
                std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
            })
            .join(Self::NAME)
        }
        pub fn switch(&mut self) {
            std::mem::swap(&mut self.from, &mut self.to);
        }
        pub fn save(&self) -> std::io::Result<()> {
            let path = Self::file();
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, [self.from as u8, self.to as u8])
        }
        pub fn url(&self) -> Option<String> {
            let from = self.lang.get(usize::try_from(self.from).ok()?)?;
            let to = self.lang.get(usize::try_from(self.to).ok()?)?;
            Some(format!(
                "{}/{}/{}/{}",
                Self::SERVICE,
                from.0,
                to.0,
                &self
                    .source
                    .replace("%", "%25")
                    .replace("/", "%20")
                    .replace(r#"\"#, "%20")
                    .replace(" ", "%20")
                    .replace("\n", "%0A")
                    .replace("?", "%3F")
            ))
        }
        pub fn lang(&self) -> Vec<&str> {
            self.lang.iter().map(|lang| lang.1.as_str()).collect()
        }
    }
}

#[cfg(target_os = "linux")]
mod components {
    use easytk::prelude::*;

    #[derive(Default)]
    pub struct Application {}
    impl Component for Application {
        type Event = bool;
        type State = bool;
        fn handle(msg: Self::Event, _: &mut Self::State, _: Sender<Self::Event>) -> bool {
            msg
        }
        fn update(&self, _: &Self::State) {}
        fn view(&self, _: Sender<Self::Event>) -> gtk::Box {
            let stack = gtk::Stack::default();
            let sidebar = gtk::StackSidebar::builder().stack(&stack).build();
            let hbox = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
            hbox.add(&sidebar);
            hbox.add(&stack);
            stack.add_titled(&Converter::mount(), "Conv", "Conv");
            stack.add_titled(&Curl::mount(), "Curl", "Curl");
            stack.add_titled(&Calc::mount(), "Calc", "Calc");
            stack.add_titled(&Dial::mount(), "Dial", "Dial");
            stack.add_titled(&Sudo::mount(), "Sudo", "Sudo");
            stack.add_titled(
                &{
                    let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
                    wgt.set_margin(PAD);
                    let list_store = gtk::ListStore::new(&[
                        gtk::glib::types::Type::BOOL,
                        gtk::glib::types::Type::STRING,
                        gtk::glib::types::Type::STRING,
                        gtk::glib::Type::U32,
                    ]);
                    for url in ["https://ipinfo.io/json"] {
                        let name = url.rsplit('/').next().unwrap_or(url);
                        list_store.set(&list_store.append(), &[(0, &true), (1, &name), (2, &url)]);
                    }
                    let tree_view = gtk::TreeView::with_model(&list_store);
                    for (ord, name) in ["STATUS", "NAME", "URL"].into_iter().enumerate() {
                        match name {
                            "STATUS" => {
                                let renderer = gtk::CellRendererToggle::new();
                                renderer.connect_toggled({
                                    let list_store = list_store.clone();
                                    move |_, path| {
                                        if let Some(iter) = list_store.iter(&path) {
                                            let active = list_store
                                                .value(&iter, ord as i32)
                                                .get::<bool>()
                                                .unwrap_or(false);
                                            list_store.set_value(
                                                &iter,
                                                ord as u32,
                                                &(!active).to_value(),
                                            );
                                        }
                                    }
                                });
                                tree_view.append_column(&gtk::TreeViewColumn::with_attributes(
                                    name,
                                    &renderer,
                                    &[("active", ord as i32)],
                                ))
                            }
                            _ => tree_view.append_column(&gtk::TreeViewColumn::with_attributes(
                                name,
                                &gtk::CellRendererText::new(),
                                &[("text", ord as i32)],
                            )),
                        };
                    }
                    let run = gtk::Button::with_mnemonic("Run");
                    let result = gtk::Label::with_mnemonic("Result");
                    wgt.add(&tree_view);
                    wgt.add(&{
                        let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                        wgt.add(&run);
                        wgt.add(&result);
                        wgt
                    });
                    run.connect_clicked({
                        let run = run.clone();
                        move |_| {
                            list_store.foreach(|store, _, iter| {
                                if store.value(iter, 0).get::<bool>().unwrap_or(false) {
                                    if let Ok(url) = list_store.value(iter, 2).get::<String>() {
                                        println!("{url}");
                                    }
                                };
                                false
                            });
                            let value = result.text();
                            result.set_text(&(value.to_string() + " Done!"));
                            run.set_sensitive(false);
                        }
                    });
                    let adjustment = gtk::Adjustment::new(0.0, 0.0, 255.0, 1.0, 10.0, 0.0);
                    wgt.add(&{
                        let wgt = gtk::SpinButton::with_range(0.0, 255.0, 1.0);
                        wgt.set_adjustment(&adjustment);
                        wgt
                    });
                    wgt.add(&{
                        let wgt =
                            gtk::Scale::with_range(gtk::Orientation::Horizontal, 0.0, 255.0, 1.0);
                        wgt.set_adjustment(&adjustment);
                        wgt
                    });
                    wgt
                },
                "Othe",
                "Othe",
            );
            hbox
        }
    }

    #[derive(Default)]
    pub struct Converter(gtk::Entry, gtk::Entry);
    impl Component for Converter {
        type Event = f64;
        type State = f64;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            *model = msg;
            true
        }
        fn update(&self, model: &Self::State) {
            self.0.update(&((*model - 32.0) * 5.0 / 9.0).to_string());
            self.1.update(&((*model * 9.0 / 5.0) + 32.0).to_string());
        }
        fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
            let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
            wgt.set_margin(PAD);
            wgt.add(&{
                let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                wgt.add({
                    self.0.set_placeholder_text(Some("°C"));
                    self.0.set_hexpand(true);
                    self.0.connect_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_visible_focus()
                                && let Ok(value) = wgt.buffer().text().parse::<f64>()
                            {
                                let _ = sender.send(value);
                            }
                        }
                    });
                    &self.0
                });
                wgt.add({
                    self.1.set_placeholder_text(Some("°F"));
                    self.1.set_hexpand(true);
                    self.1.connect_changed({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_visible_focus()
                                && let Ok(value) = wgt.buffer().text().parse::<f64>()
                            {
                                let _ = sender.send(value);
                            }
                        }
                    });
                    &self.1
                });
                wgt
            });
            wgt
        }
    }

    const PAD: i32 = 10;

    #[derive(Default)]
    pub struct Calc(gtk::TextBuffer, gtk::Label, gtk::Label, gtk::Label);
    impl Component for Calc {
        type Event = String;
        type State = super::models::Calculator;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            model.click(&msg);
            true
        }
        fn update(&self, model: &Self::State) {
            self.0.set_text(&model.output);
            self.1.set_label(&model.operation);
            self.2.set_label(&model.prev.to_string());
            self.3.set_label(&model.current);
        }
        fn view(&self, _sender: Sender<Self::Event>) -> gtk::Box {
            let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
            wgt.set_margin(PAD);
            wgt.add(
                &gtk::ScrolledWindow::builder()
                    .hscrollbar_policy(gtk::PolicyType::Automatic)
                    .vscrollbar_policy(gtk::PolicyType::Automatic)
                    .vexpand(true)
                    .child(
                        &gtk::TextView::builder()
                            .buffer(&self.0)
                            .monospace(true)
                            .build(),
                    )
                    .build(),
            );
            wgt.add(&{
                let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                wgt.set_margin(PAD);
                wgt.add(&self.1);
                wgt.add(&self.2);
                wgt.add(&self.3);
                wgt
            });
            wgt
        }
    }

    #[derive(Default)]
    pub struct Curl(gtk::TextBuffer);
    impl Component for Curl {
        type Event = super::msgs::Curl;
        type State = (String, String, String);
        fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Url(value) => model.0 = value,
                Self::Event::Body(value) => model.1 = value,
                Self::Event::Responce(value) => {
                    model.2 = value;
                    return true;
                }
                Self::Event::Run => {
                    let url = model.0.clone();
                    std::thread::spawn({
                        let sender = sender.clone();
                        move || {
                            let value = match reqwest::blocking::get(&url)
                                .and_then(reqwest::blocking::Response::error_for_status)
                                .and_then(reqwest::blocking::Response::text)
                            {
                                Ok(value) => value,
                                Err(error) => format!("Request failed: {error}"),
                            };
                            let _ = sender.send(Self::Event::Responce(value));
                        }
                    });
                }
            };
            false
        }
        fn update(&self, model: &Self::State) {
            self.0.set_text(&model.2);
        }
        fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
            let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
            wgt.set_margin(PAD);
            wgt.add(&{
                let list = gtk::ListStore::new(&[gtk::glib::Type::STRING]);
                for line in [
                    r#"https://jsonplaceholder.typicode.com/users"#,
                    r#"https://jsonplaceholder.typicode.com/posts"#,
                    r#"https://jsonplaceholder.typicode.com/albums"#,
                    r#"https://jsonplaceholder.typicode.com/todos"#,
                    r#"https://jsonplaceholder.typicode.com/comments"#,
                    r#"https://jsonplaceholder.typicode.com/posts"#,
                    r#"https://lingva.ml/api/v1/languages"#,
                    r#"https://lingva.ml/api/v1/en/de/rust"#,
                    r#"https://ipinfo.io/json"#,
                ] {
                    list.set(&list.append(), &[(0, &line)]);
                }
                let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                wgt.add(&{
                    let wgt = gtk::ComboBoxText::new();
                    wgt.set_tooltip_text(Some("Method"));
                    wgt.append_text("GET");
                    wgt.append_text("POST");
                    wgt
                });
                wgt.add(&{
                    let wgt = gtk::Entry::default();
                    wgt.connect_changed({
                        let sender = sender.clone();
                        move |entry| {
                            if entry.is_sensitive() {
                                let _ = sender
                                    .send(Self::Event::Url(entry.buffer().text().to_string()));
                            }
                        }
                    });
                    wgt.set_placeholder_text(Some("URL"));
                    wgt.set_completion(Some(&{
                        let wgt = gtk::EntryCompletion::default();
                        wgt.set_popup_completion(true);
                        wgt.set_model(Some(&list));
                        wgt.set_text_column(0);
                        wgt
                    }));
                    wgt.set_hexpand(true);
                    wgt
                });
                wgt.add(&{
                    let wgt = gtk::Button::with_mnemonic("list-add");
                    wgt.set_tooltip_text(Some("Run"));
                    wgt.connect_clicked({
                        let sender = sender.clone();
                        move |_| {
                            let _ = sender.send(Self::Event::Run);
                        }
                    });
                    wgt
                });
                wgt
            });
            wgt.add(&{
                let wgt = gtk::Notebook::new();
                wgt.append_page(
                    &gtk::ScrolledWindow::builder()
                        .hscrollbar_policy(gtk::PolicyType::Automatic)
                        .vscrollbar_policy(gtk::PolicyType::Automatic)
                        .child(&{
                            let wgt = gtk::TextView::with_buffer(&{
                                let wgt = gtk::TextBuffer::default();
                                wgt.connect_text_notify({
                                    let sender = sender.clone();
                                    move |buffer| {
                                        let value = buffer
                                            .start_iter()
                                            .text(&buffer.end_iter())
                                            .unwrap_or_default()
                                            .to_string();
                                        let _ = sender.send(Self::Event::Body(value));
                                    }
                                });
                                wgt
                            });
                            wgt.set_hexpand(true);
                            wgt.set_tooltip_text(Some("Body"));
                            wgt
                        })
                        .vexpand(true)
                        .build(),
                    Some(&gtk::Label::with_mnemonic("Body")),
                );
                wgt.append_page(
                    &gtk::ScrolledWindow::builder()
                        .hscrollbar_policy(gtk::PolicyType::Automatic)
                        .vscrollbar_policy(gtk::PolicyType::Automatic)
                        .child(&{
                            let wgt = gtk::TextView::with_buffer(&self.0);
                            wgt.set_hexpand(true);
                            wgt.set_monospace(true);
                            wgt
                        })
                        .vexpand(true)
                        .build(),
                    Some(&gtk::Label::with_mnemonic("Request")),
                );
                wgt
            });
            wgt
        }
    }

    #[derive(Default)]
    pub struct Dial(
        gtk::ComboBoxText,
        gtk::ComboBoxText,
        gtk::TextBuffer,
        gtk::TextBuffer,
        gtk::Button,
    );
    impl Component for Dial {
        type Event = super::msgs::Dialect;
        type State = super::models::Dialect;
        fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Switch => {
                    model.switch();
                    return true;
                }
                Self::Event::Open(value) => {
                    match std::fs::read_to_string(value) {
                        Ok(source) => model.source = source,
                        Err(error) => eprintln!("Could not open file: {error}"),
                    }
                    return true;
                }
                Self::Event::Target(value) => {
                    model.target = value;
                    return true;
                }
                Self::Event::Lang(value) => {
                    model.read(value);
                    return true;
                }
                Self::Event::SaveAs(value) => {
                    if let Err(error) = std::fs::write(value, model.target.as_bytes()) {
                        eprintln!("Could not save file: {error}");
                    }
                }
                Self::Event::To(value) => model.to = value,
                Self::Event::From(value) => model.from = value,
                Self::Event::Source(value) => model.source = value,
                Self::Event::Quit => {
                    if let Err(error) = model.save() {
                        eprintln!("Could not save preferences: {error}");
                    }
                }
                Self::Event::Run => {
                    if model.from != model.to && !model.source.is_empty() {
                        if let Some(url) = model.url() {
                            std::thread::spawn({
                                let sender = sender.clone();
                                move || {
                                    let result = reqwest::blocking::get(&url)
                                        .and_then(reqwest::blocking::Response::error_for_status)
                                        .and_then(|response| {
                                            response
                                                .json::<std::collections::HashMap<String, String>>()
                                        })
                                        .map_err(|error| error.to_string())
                                        .and_then(|value| {
                                            value.get("translation").cloned().ok_or_else(|| {
                                                "response has no translation field".to_string()
                                            })
                                        });
                                    let target = result.unwrap_or_else(|error| {
                                        format!("Translation failed: {error}")
                                    });
                                    let _ = sender.send(Self::Event::Target(target));
                                }
                            });
                        }
                    };
                }
            }
            false
        }
        fn update(&self, model: &Self::State) {
            if self.0.active_text().is_none() && !model.lang.is_empty() {
                for (_, name) in &model.lang {
                    self.0.append_text(name);
                    self.1.append_text(name);
                }
            }
            self.0.set_active(Some(model.from.max(0) as u32));
            self.1.set_active(Some(model.to.max(0) as u32));
            let source = self
                .2
                .start_iter()
                .text(&self.2.end_iter())
                .unwrap_or_default();
            if source.as_str() != model.source {
                self.2.set_text(&model.source);
            }
            let target = self
                .3
                .start_iter()
                .text(&self.3.end_iter())
                .unwrap_or_default();
            if target.as_str() != model.target {
                self.3.set_text(&model.target);
            }
        }
        fn view(&self, sender: Sender<Self::Event>) -> gtk::Box {
            self.0.set_tooltip_text(Some("From"));
            self.0.set_hexpand(true);
            self.1.set_tooltip_text(Some("To"));
            self.1.set_hexpand(true);
            self.0.connect_changed({
                let sender = sender.clone();
                move |combo| {
                    if let Some(value) = combo.active() {
                        let _ = sender.send(Self::Event::From(value as i32));
                    }
                }
            });
            self.1.connect_changed({
                let sender = sender.clone();
                move |combo| {
                    if let Some(value) = combo.active() {
                        let _ = sender.send(Self::Event::To(value as i32));
                    }
                }
            });
            self.2.connect_changed({
                let sender = sender.clone();
                move |buffer| {
                    let value = buffer
                        .start_iter()
                        .text(&buffer.end_iter())
                        .unwrap_or_default();
                    let _ = sender.send(Self::Event::Source(value.to_string()));
                }
            });
            std::thread::spawn({
                let sender = sender.clone();
                move || {
                    let response = reqwest::blocking::get("https://lingva.ml/api/v1/languages")
                        .and_then(reqwest::blocking::Response::error_for_status)
                        .and_then(|response| {
                            response.json::<HashMap<String, Vec<HashMap<String, String>>>>()
                        });
                    match response {
                        Ok(data) => {
                            let languages = data
                                .get("languages")
                                .into_iter()
                                .flatten()
                                .filter_map(|language| {
                                    Some((
                                        language.get("code")?.clone(),
                                        language.get("name")?.clone(),
                                    ))
                                })
                                .collect::<Vec<_>>();
                            if !languages.is_empty() {
                                let _ = sender.send(Self::Event::Lang(languages));
                            }
                        }
                        Err(error) => eprintln!("Could not load language list: {error}"),
                    }
                }
            });
            let wgt = gtk::Box::new(gtk::Orientation::Vertical, PAD);
            wgt.set_margin(PAD);
            wgt.add(&{
                let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                wgt.add(&self.0);
                wgt.add(&{
                    let wgt = gtk::Button::with_mnemonic("Switch");
                    wgt.connect_clicked({
                        let sender = sender.clone();
                        move |_| {
                            let _ = sender.send(Self::Event::Switch);
                        }
                    });
                    wgt
                });
                wgt.add(&self.1);
                wgt
            });
            wgt.add(&{
                let wgt = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                wgt.add(
                    &gtk::ScrolledWindow::builder()
                        .hscrollbar_policy(gtk::PolicyType::Automatic)
                        .vscrollbar_policy(gtk::PolicyType::Automatic)
                        .vexpand(true)
                        .child(
                            &gtk::TextView::builder()
                                .buffer(&self.2)
                                .hexpand(true)
                                .monospace(true)
                                .build(),
                        )
                        .build(),
                );
                wgt.add(
                    &gtk::ScrolledWindow::builder()
                        .hscrollbar_policy(gtk::PolicyType::Automatic)
                        .vscrollbar_policy(gtk::PolicyType::Automatic)
                        .vexpand(true)
                        .child(
                            &gtk::TextView::builder()
                                .buffer(&self.3)
                                .hexpand(true)
                                .monospace(true)
                                .build(),
                        )
                        .build(),
                );
                wgt
            });
            wgt
        }
    }

    #[derive(Default)]
    pub struct Sudo([[gtk::Button; 9]; 9]);
    impl Component for Sudo {
        type Event = super::msgs::Sudoku;
        type State = super::models::Sudoku;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Push(row, col, value) => model.0[row][col] = value,
                Self::Event::Clear => model.clear(),
                Self::Event::Solve => model.answer(),
            };
            true
        }
        fn update(&self, model: &Self::State) {
            for row in 0..9 {
                for col in 0..9 {
                    self.0[row][col].set_label(&match model.0[row][col] {
                        0 => String::new(),
                        _ => model.0[row][col].to_string(),
                    });
                }
            }
        }
        fn view(&self, _sender: Sender<Self::Event>) -> gtk::Box {
            let y = gtk::Box::new(gtk::Orientation::Vertical, PAD);
            y.set_margin(PAD);
            for row in 0..9 {
                let x = gtk::Box::new(gtk::Orientation::Horizontal, PAD);
                x.set_margin(PAD);
                for col in 0..9 {
                    x.add(&self.0[row][col]);
                }
                y.add(&x);
            }
            y
        }
    }
}

#[cfg(target_os = "windows")]
mod components {
    use easytk::prelude::*;
    use std::collections::HashMap;

    #[derive(Clone, Default)]
    pub struct Converter(Input, Input);
    impl Component for Converter {
        type Event = f64;
        type State = f64;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            *model = msg;
            true
        }
        fn update(&mut self, model: &Self::State) {
            self.0.update(&((*model - 32.0) * 5.0 / 9.0).to_string());
            self.1.update(&((*model * 9.0 / 5.0) + 32.0).to_string());
        }
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
            let mut wgt = Wizard::default_fill();
            wgt.set_frame(FrameType::FlatBox);
            wgt.add(&info().with_label("Info"));
            wgt.add(&{
                let mut wgt = Flex::default_fill().with_label("Converter");
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_margin(10);
                wgt.add({
                    self.0.set_tooltip("Cel");
                    self.0.set_type(InputType::Float);
                    self.0.set_trigger(CallbackTrigger::Changed);
                    self.0.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus()
                                && let Ok(value) = wgt.value().parse::<f64>()
                            {
                                sender.send(value).unwrap();
                            }
                        }
                    });
                    &self.0
                });
                wgt.add({
                    self.1.set_tooltip("Far");
                    self.1.set_type(InputType::Float);
                    self.1.set_trigger(CallbackTrigger::Changed);
                    self.1.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus()
                                && let Ok(value) = wgt.value().parse::<f64>()
                            {
                                sender.send(value).unwrap();
                            }
                        }
                    });
                    &self.1
                });
                wgt
            });
            wgt.add(&Calculator::default().mount());
            wgt.add(&Sudoku::default().mount());
            wgt.add(&Pictures::default().mount());
            wgt.add(&Dialect::default().mount());
            wgt.end();
            wgt
        }
    }

    fn info() -> Flex {
        let mut wgt = Flex::default_fill();
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_margin(10);
        wgt.add(&{
            let mut wgt = Flex::default().column();
            wgt.set_frame(FrameType::RFlatBox);
            wgt.set_color(Color::Background2);
            wgt.set_pad(0);
            wgt.set_margin(5);
            wgt.add(&{
                let mut wgt = HelpView::default();
                wgt.set_color(Color::Background2);
                wgt.set_scrollbar_size(3);
                wgt.set_text_size(16);
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_tooltip("README");
                wgt.set_value(include_str!("../assets/README.html"));
                wgt
            });
            wgt.add(&{
                let mut wgt = Frame::default();
                wgt.config();
                wgt
            });
            wgt.add(&{
                let mut wgt = TextDisplay::default();
                wgt.config();
                wgt.set_frame(FrameType::FlatBox);
                wgt.set_tooltip("LICENSE");
                wgt.insert(include_str!("../LICENSE"));
                wgt
            });
            wgt.end();
            wgt.handle(Flex::listen);
            wgt.handle_event(Event::Resize);
            wgt
        });
        wgt.end();
        wgt.handle(add_wizard);
        wgt
    }

    fn add_wizard(wgt: &mut Flex, event: Event) -> bool {
        let wizard = Wizard::from_dyn_widget(&wgt.parent().unwrap()).unwrap();
        match event {
            Event::Push => match event_mouse_button() {
                MouseButton::Right => {
                    let mut wgt = MenuButton::default();
                    wgt.add_choice(
                        &(0..wizard.children())
                            .map(|x| {
                                let label = wizard.child(x).unwrap().label();
                                match wizard.try_current_widget().unwrap().label() == label {
                                    true => format!("@->  {label}"),
                                    false => format!("@-  {label}"),
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("|"),
                    );
                    wgt.set_callback({
                        let mut wizard = wizard.clone();
                        move |menu| {
                            if let Some(mut child) = wizard.child(menu.value()) {
                                wizard.set_current_widget(&child);
                                if let Some(mut window) = wizard.window() {
                                    window.set_label(&format!(
                                        "{}::{}",
                                        window.xclass().unwrap(),
                                        child.label()
                                    ));
                                };
                                child.handle_event(Event::Resize);
                            }
                        }
                    });
                    wgt.popup();
                    true
                }
                _ => false,
            },
            _ => false,
        }
    }

    #[derive(Clone, Default)]
    pub struct Calculator {
        prev: Frame,
        oper: Frame,
        curr: Frame,
        outp: TextDisplay,
    }

    const PAD: i32 = 10;
    const HEIGHT: i32 = PAD * 3;

    impl Component for Calculator {
        type Event = String;
        type State = super::models::Calculator;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            model.click(&msg);
            true
        }
        fn update(&mut self, model: &Self::State) {
            self.prev.set_label(&model.prev.to_string());
            self.oper.set_label(&model.operation);
            self.curr.set_label(&model.current);
            self.outp.update(&model.output);
        }
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
            let mut wgt = Flex::default_fill();
            wgt.set_label("Calculator");
            wgt.set_frame(FrameType::FlatBox);
            wgt.set_margin(0);
            wgt.set_pad(0);
            wgt.add(&{
                let mut wgt = Flex::default_fill();
                wgt.set_margin(PAD);
                wgt.add(&Frame::default()); //LEFT
                wgt.fixed(
                    &{
                        let mut wgt = Flex::default_fill().column();
                        wgt.add(&{
                            let mut wgt = Flex::default_fill().column(); //UP
                            wgt.set_margin(0);
                            wgt.set_pad(PAD);
                            wgt.add(&{
                                let mut wgt = Flex::default_fill().column(); //OUTPUT
                                wgt.set_frame(FrameType::RFlatBox);
                                wgt.set_color(Color::Background2);
                                wgt.set_margin(5);
                                wgt.set_pad(0);
                                wgt.add({
                                    self.outp.set_tooltip("Output");
                                    self.outp.set_text_size(HEIGHT - 2);
                                    self.outp.set_scrollbar_size(3);
                                    self.outp.set_frame(FrameType::FlatBox);
                                    self.outp.config();
                                    &self.outp
                                });
                                wgt.fixed(
                                    &{
                                        let mut wgt = Frame::default();
                                        wgt.set_frame(FrameType::FlatBox);
                                        wgt
                                    },
                                    3,
                                );
                                wgt.fixed(
                                    &{
                                        let mut wgt = Flex::default_fill();
                                        wgt.set_pad(0);
                                        wgt.set_margin(0);
                                        wgt.add({
                                            config_frame(&mut self.oper, "Operation");
                                            &self.oper
                                        });
                                        wgt.fixed(&self.oper, HEIGHT);
                                        wgt.add(&{
                                            let mut wgt = Flex::default_fill().column();
                                            wgt.set_pad(0);
                                            wgt.set_margin(0);
                                            wgt.add({
                                                config_frame(&mut self.prev, "Previous");
                                                &self.prev
                                            });
                                            wgt.add({
                                                config_frame(&mut self.curr, "Current");
                                                &self.curr
                                            });
                                            wgt.end();
                                            wgt
                                        });
                                        wgt.end();
                                        wgt
                                    },
                                    HEIGHT * 2,
                                );
                                wgt.end();
                                wgt
                            });
                            wgt.fixed(
                                &{
                                    let mut buttons = Flex::default_fill().column();
                                    buttons.set_pad(PAD);
                                    buttons.set_margin(0);
                                    for line in [
                                        ["CE", "C", "%", "/"],
                                        ["7", "8", "9", "x"],
                                        ["4", "5", "6", "-"],
                                        ["1", "2", "3", "+"],
                                        ["0", ".", "@<-", "="],
                                    ] {
                                        let mut hbox = Flex::default_fill();
                                        for label in line {
                                            hbox.add(&{
                                                let mut wgt = btn();
                                                wgt.set_label(label);
                                                wgt.set_label_size(HEIGHT);
                                                wgt.set_frame(FrameType::OFlatBox);
                                                wgt.set_color(
                                                    match [
                                                        ".", "@<-", "CE", "C", "x", "/", "+", "-",
                                                        "%",
                                                    ]
                                                    .contains(&label)
                                                    {
                                                        false => Color::Selection,
                                                        true => Color::Inactive,
                                                    },
                                                );
                                                wgt.set_callback({
                                                    let sender = sender.clone();
                                                    move |wgt| {
                                                        sender.send(wgt.label()).unwrap();
                                                    }
                                                });
                                                wgt
                                            });
                                        }
                                        hbox.end();
                                        hbox.set_pad(PAD);
                                        hbox.set_margin(0);
                                    }
                                    buttons.end();
                                    buttons
                                },
                                425,
                            );
                            wgt.end();
                            wgt
                        });
                        wgt.end();
                        wgt
                    },
                    340,
                );
                wgt.add(&Frame::default());
                wgt.end();
                wgt
            });
            wgt.end();
            wgt.handle(add_wizard);
            wgt
        }
    }

    fn config_frame(wgt: &mut Frame, tooltip: &str) {
        wgt.set_tooltip(tooltip);
        wgt.set_align(Align::Right | Align::Inside);
        wgt.set_color(Color::Background2);
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_label_size(HEIGHT);
    }

    fn btn() -> Frame {
        let mut color = Color::Background2;
        let mut wgt = Frame::default();
        wgt.set_frame(FrameType::RFlatBox);
        wgt.set_label_color(color);
        wgt.handle(move |wgt, event| match event {
            Event::Push => {
                wgt.do_callback();
                color = wgt.color();
                wgt.set_color(color.lighter());
                wgt.redraw();
                true
            }
            Event::Released => {
                wgt.set_color(color);
                wgt.redraw();
                true
            }
            Event::Enter => {
                draw::set_cursor(Cursor::Hand);
                true
            }
            Event::Leave => {
                draw::set_cursor(Cursor::Arrow);
                true
            }
            _ => false,
        });
        wgt
    }

    #[derive(Clone, Default)]
    pub struct Pictures {
        prev: Button,
        next: Button,
        remo: Button,
        canv: Frame,
        list: Choice,
        valu: Slider,
    }

    impl Component for Pictures {
        type Event = super::msgs::Pictures;
        type State = super::models::Pictures;
        fn update(&mut self, model: &Self::State) {
            self.prev.update(model.list().len() > 1);
            self.next.update(model.list().len() > 1);
            self.remo.update(model.list().len() > 1);
            self.valu.update(!model.list().is_empty());
            self.list.update((&model.list(), model.idx as i32));
            self.canv.update((model.path(), model.scale as i32));
        }
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Add(value) => model.set_list(value),
                Self::Event::Idx(value) => model.idx = value,
                Self::Event::Shift(value) => model.shift(value),
                Self::Event::Del(value) => model.del(value),
                Self::Event::Scale(value) => model.scale = value,
            };
            true
        }
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
            let mut wgt = Flex::default_fill().column();
            wgt.set_label("Pictures");
            wgt.set_frame(FrameType::FlatBox);
            wgt.set_margin(PAD);
            wgt.set_pad(PAD);
            wgt.fixed(
                &{
                    let mut wgt = Flex::default();
                    wgt.set_pad(0);
                    wgt.fixed(
                        &{
                            let mut wgt = Button::default();
                            wgt.set_label("@+");
                            wgt.set_tooltip("Add ...");
                            wgt.set_label_color(Color::Green);
                            wgt.set_shortcut(Shortcut::Ctrl | 'a');
                            wgt.set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    if let Some(value) = choice_files("*.{png,svg}") {
                                        sender.send(Self::Event::Add(value)).unwrap();
                                    }
                                }
                            });
                            wgt
                        },
                        HEIGHT,
                    );
                    wgt.add(&self.prev);
                    wgt.fixed(
                        {
                            self.prev.set_label("@<-");
                            self.prev.set_tooltip("Prev ...");
                            self.prev.set_shortcut(Shortcut::Ctrl | 'j');
                            self.prev.set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Self::Event::Shift(false)).unwrap();
                                }
                            });
                            &self.prev
                        },
                        HEIGHT,
                    );
                    wgt.add({
                        self.list.set_tooltip("List");
                        self.list.set_callback({
                            let sender = sender.clone();
                            move |menu| {
                                sender
                                    .send(Self::Event::Idx(menu.value() as usize))
                                    .unwrap();
                            }
                        });
                        &self.list
                    });
                    wgt.add(&self.next);
                    wgt.fixed(
                        {
                            self.next.set_label("@->");
                            self.next.set_tooltip("Next ...");
                            self.next.set_shortcut(Shortcut::Ctrl | 'k');
                            self.next.set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Self::Event::Shift(true)).unwrap();
                                }
                            });
                            &self.next
                        },
                        HEIGHT,
                    );
                    wgt.add(&self.remo);
                    wgt.fixed(
                        {
                            self.remo.set_label("@1+");
                            self.remo.set_tooltip("Remove ...");
                            self.remo.set_shortcut(Shortcut::Ctrl | 'd');
                            self.remo.set_label_color(Color::Red);
                            self.remo.set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    match choice2_default(
                                        "Remove ...?",
                                        "Remove",
                                        "Cancel",
                                        "Permanent",
                                    ) {
                                        Some(0) => sender.send(Self::Event::Del(false)).unwrap(),
                                        Some(2) => sender.send(Self::Event::Del(true)).unwrap(),
                                        _ => {}
                                    };
                                }
                            });
                            &self.remo
                        },
                        HEIGHT,
                    );
                    wgt.end();
                    wgt
                },
                HEIGHT,
            );
            wgt.add({
                config_canvas(&mut self.canv);
                &self.canv
            });
            wgt.add(&self.valu);
            wgt.fixed(
                {
                    self.valu.config();
                    self.valu.set_value(0.5);
                    self.valu.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender
                                .send(Self::Event::Scale(wgt.value() * 100.0))
                                .unwrap()
                        }
                    });
                    self.valu.do_callback();
                    &self.valu
                },
                HEIGHT,
            );
            wgt.end();
            wgt.handle(add_wizard);
            wgt
        }
    }

    fn config_canvas(wgt: &mut Frame) {
        use std::collections::HashMap;
        let mut cash: HashMap<String, SharedImage> = HashMap::new();
        wgt.set_frame(FrameType::FlatBox);
        wgt.set_color(Color::Background2);
        wgt.set_callback(move |frame| {
            let scale = frame.label_size() as f64;
            match frame.tooltip() {
                None => frame.set_image(None::<SharedImage>),
                Some(file) => {
                    frame.set_image(None::<SharedImage>);
                    if let Some(mut window) = frame.window() {
                        window.redraw();
                    };
                    if !file.is_empty() {
                        if !cash.contains_key(&file)
                            && let Ok(image) = SharedImage::load(&file)
                        {
                            cash.insert(file.clone(), image);
                        }
                        let mut image = cash.get(&file).unwrap().clone();
                        image.scale(
                            (scale / 100.0 * (frame.w()) as f64) as i32,
                            (scale / 100.0 * (frame.h()) as f64) as i32,
                            true,
                            false,
                        );
                        frame.set_image(Some(image));
                    }
                }
            }
            frame.redraw();
        });
    }

    #[derive(Clone, Default)]
    pub struct Sudoku([[Frame; 9]; 9]);

    impl Component for Sudoku {
        type Event = super::msgs::Sudoku;
        type State = super::models::Sudoku;
        fn handle(msg: Self::Event, model: &mut Self::State, _: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Push(row, col, value) => model.0[row][col] = value,
                Self::Event::Clear => model.clear(),
                Self::Event::Solve => model.answer(),
            };
            true
        }
        fn update(&mut self, model: &Self::State) {
            for row in 0..9 {
                for col in 0..9 {
                    self.0[row][col].set_label(&match model.0[row][col] {
                        0 => String::new(),
                        _ => model.0[row][col].to_string(),
                    });
                }
            }
        }
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
            let mut wgt = Flex::default_fill();
            wgt.set_label("Sudoku");
            wgt.set_frame(FrameType::FlatBox);
            wgt.set_margin(0);
            wgt.set_pad(0);
            wgt.add(&{
                let mut wgt = Flex::default_fill();
                wgt.add(&Frame::default_fill()); //LEFT
                wgt.add(&{
                    let mut wgt = Flex::default_fill().column(); // CENTER
                    wgt.set_pad(PAD);
                    wgt.set_margin(0);
                    wgt.add(&Frame::default()); //UP
                    wgt.add(&{
                        let pad = 4;
                        let mut vbox = Flex::default_fill().column(); // CENTER
                        vbox.set_frame(FrameType::RoundedFrame);
                        vbox.set_color(Color::Foreground);
                        vbox.set_pad(pad);
                        vbox.set_margin(5);
                        for row in 0..9 {
                            if row > 0 && row % 3 == 0 {
                                vbox.fixed(&Frame::default(), pad * 2);
                            };
                            let mut flex = Flex::default_fill();
                            flex.set_pad(pad);
                            flex.set_margin(0);
                            for col in 0..9 {
                                if col > 0 && col % 3 == 0 {
                                    flex.fixed(&Frame::default(), pad * 2);
                                };
                                flex.add({
                                    self.0[row][col].draw(draw_frame);
                                    self.0[row][col].handle({
                                        let sender = sender.clone();
                                        move |frm, event| match event {
                                            Event::Push => match event_mouse_button() {
                                                MouseButton::Left => {
                                                        let mut wgt = MenuButton::default();
                                                        wgt.set_pos(frm.x(),frm.y());
                                                        wgt.set_size(frm.w(),frm.h());
                                                        wgt.add_choice(
                                                            "  X  |  1  |  2  |  3  |  4  |  5  |  6  |  7  |  8  |  9  "
                                                        );
                                                        wgt.set_callback({
                                                            let sender = sender.clone();
                                                            move |menu| {
                                                                sender.send(Self::Event::Push(row, col, menu.value())).unwrap();
                                                            }
                                                        });
                                                    wgt.popup();
                                                    true
                                                }
                                                MouseButton::Right => {
                                                        let mut wgt = MenuButton::default();
                                                        wgt.set_pos(frm.x(),frm.y());
                                                        wgt.set_size(frm.w(),frm.h());
                                                        wgt.add_choice("@search  Solve|@1+  Clear");
                                                        wgt.set_callback({
                                                            let sender = sender.clone();
                                                            move |menu| {
                                                                sender.send(match menu.value() {
                                                                    0 => Self::Event::Solve,
                                                                    _ => Self::Event::Clear,
                                                                }).unwrap();
                                                            }
                                                        });
                                                        wgt.popup();
                                                    true
                                                }
                                                _ => false,
                                            }
                                            Event::Enter => {
                                                draw::set_cursor(Cursor::Hand);
                                                true
                                            }
                                            Event::Leave => {
                                                draw::set_cursor(Cursor::Arrow);
                                                true
                                            }
                                            _ => false,
                                        }
                                    });
                                    &self.0[row][col]
                                });
                            }
                            flex.end();
                        }
                        vbox.end();
                        vbox
                    });
                    wgt.add(&Frame::default()); //BOTTOM
                    wgt.end();
                    wgt.handle(add_resize); // RESIZE CENTER's HEIGHT
                    wgt
                });
                wgt.add(&Frame::default()); //RIGHT
                wgt.end();
                wgt.handle(add_resize);  // RESIZE CENTER's WIDTH
                wgt.handle_event(Event::Resize);
                wgt
            });
            wgt.end();
            wgt.handle(add_wizard);
            wgt
        }
    }

    fn draw_frame(frame: &mut Frame) {
        draw::set_draw_color(Color::Background2);
        draw::set_font(Font::CourierBold, (frame.h() as f64 / 4.0 * 3.0) as i32);
        draw::draw_circle_fill(frame.x(), frame.y(), frame.h(), Color::Inactive);
        draw::draw_text2(
            &frame.label(),
            frame.x(),
            frame.y(),
            frame.w(),
            frame.h(),
            Align::Center,
        );
    }

    fn add_resize(flex: &mut Flex, event: Event) -> bool {
        if event == Event::Resize {
            if flex.children() == 3
                && let Some(child) = flex.child(1)
                && let Some(window) = flex.window()
            {
                flex.fixed(&child, window.w().min(window.h()));
            }
            return true;
        }
        false
    }

    #[derive(Clone, Default)]
    pub struct Dialect {
        translate: Button,
        source: TextEditor,
        target: TextDisplay,
        from: Choice,
        to: Choice,
    }

    impl Component for Dialect {
        type Event = super::msgs::Dialect;
        type State = super::models::Dialect;
        fn update(&mut self, state: &Self::State) {
            self.to.update((&state.lang(), state.to));
            self.from.update((&state.lang(), state.from));
            self.source.update(&state.source);
            self.target.update(&state.target);
            self.translate.update(!state.lang.is_empty());
        }
        fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
            match msg {
                Self::Event::Switch => {
                    model.switch();
                    true
                }
                Self::Event::Open(value) => {
                    model.source = std::fs::read_to_string(value).unwrap();
                    true
                }
                Self::Event::SaveAs(value) => {
                    std::fs::write(value, model.target.as_bytes()).unwrap();
                    false
                }
                Self::Event::Target(value) => {
                    model.target = value;
                    true
                }
                Self::Event::Lang(value) => {
                    model.read(value);
                    true
                }
                Self::Event::To(value) => {
                    model.to = value;
                    false
                }
                Self::Event::From(value) => {
                    model.from = value;
                    true
                }
                Self::Event::Source(value) => {
                    model.source = value;
                    true
                }
                Self::Event::Quit => {
                    if let Err(error) = model.save() {
                        eprintln!("Could not save preferences: {error}");
                    }
                    app::quit();
                    false
                }
                Self::Event::Run => {
                    if model.from != model.to && !model.source.is_empty() {
                        let Some(url) = model.url() else {
                            return false;
                        };
                        std::thread::spawn({
                            let sender = sender.clone();
                            move || {
                                let result = reqwest::blocking::get(&url)
                                    .and_then(reqwest::blocking::Response::error_for_status)
                                    .and_then(|response| response.json::<HashMap<String, String>>())
                                    .map_err(|error| error.to_string())
                                    .and_then(|value| {
                                        value.get("translation").cloned().ok_or_else(|| {
                                            "response has no translation field".to_string()
                                        })
                                    });
                                let target = result
                                    .unwrap_or_else(|error| format!("Translation failed: {error}"));
                                let _ = sender.send(Self::Event::Target(target));
                            }
                        });
                    };
                    false
                }
            }
        }
        fn view(&mut self, sender: Sender<Self::Event>) -> impl WidgetExt {
            std::thread::spawn({
                let sender = sender.clone();
                move || {
                    if let Ok(get) = reqwest::blocking::get("https://lingva.ml/api/v1/languages") {
                        let value = get
                            .json::<HashMap<String, Vec<HashMap<String, String>>>>()
                            .unwrap()
                            .get("languages")
                            .unwrap()
                            .iter()
                            .map(|lang| (lang["code"].clone(), lang["name"].clone()))
                            .collect::<Vec<(String, String)>>();
                        if !value.is_empty() {
                            sender.send(Self::Event::Lang(value)).unwrap();
                        }
                    }
                }
            });
            let mut wgt = Flex::default_fill();
            wgt.set_label("Dialect");
            wgt.set_frame(FrameType::FlatBox);
            wgt.set_type(FlexType::Column);
            wgt.set_margin(PAD);
            wgt.set_pad(PAD);
            wgt.fixed(
                &{
                    let mut wgt = Flex::default();
                    wgt.set_type(FlexType::Column);
                    wgt.set_margin(0);
                    wgt.set_pad(PAD);
                    wgt.set_type(FlexType::Row);
                    wgt.fixed(
                        &{
                            let mut wgt = Frame::default();
                            wgt.set_label("@menu");
                            wgt.set_tooltip("Menu");
                            wgt.set_color(Color::Red);
                            wgt.set_callback({
                                let sender = sender.clone();
                                move |wgt| {
                                    let mut wgt =
                                        MenuButton::default().with_pos(wgt.x(), wgt.y() + wgt.h());
                                    wgt.add(
                                        "@fileopen  &Open...",
                                        Shortcut::Ctrl | 'o',
                                        MenuFlag::Normal,
                                        {
                                            let sender = sender.clone();
                                            move |_| {
                                                if let Some(value) = choice_file("*.{txt,md}") {
                                                    sender.send(Self::Event::Open(value)).unwrap();
                                                };
                                            }
                                        },
                                    );
                                    wgt.add(
                                        "@filesaveas  &Save as...",
                                        Shortcut::Ctrl | 's',
                                        MenuFlag::MenuDivider,
                                        {
                                            let sender = sender.clone();
                                            move |_| {
                                                if let Some(value) = choice_file("*.{txt,md}") {
                                                    sender
                                                        .send(Self::Event::SaveAs(value))
                                                        .unwrap();
                                                };
                                            }
                                        },
                                    );
                                    wgt.add(
                                        "@1+  &Quit\t",
                                        Shortcut::Ctrl | 'q',
                                        MenuFlag::Normal,
                                        {
                                            let sender = sender.clone();
                                            move |_| sender.send(Self::Event::Quit).unwrap()
                                        },
                                    );
                                    wgt.popup();
                                }
                            });
                            wgt
                        },
                        HEIGHT,
                    );
                    wgt.add(&Frame::default());
                    wgt.add({
                        self.from.set_tooltip("From");
                        self.from.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                sender.send(Self::Event::From(wgt.value())).unwrap();
                            }
                        });
                        &self.from
                    });
                    wgt.fixed(&self.from, 105);
                    wgt.fixed(
                        &{
                            let mut wgt = Button::default();
                            wgt.set_label("@refresh");
                            wgt.set_tooltip("Refresh");
                            wgt.set_color(Color::Blue);
                            wgt.set_callback({
                                let sender = sender.clone();
                                move |_| {
                                    sender.send(Self::Event::Switch).unwrap();
                                }
                            });
                            wgt
                        },
                        HEIGHT,
                    );
                    wgt.add({
                        self.to.set_tooltip("To");
                        self.to.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                sender.send(Self::Event::To(wgt.value())).unwrap();
                            }
                        });
                        &self.to
                    });
                    wgt.fixed(&self.to, 105);
                    wgt.add(&Frame::default());
                    wgt.add({
                        self.translate.set_label("@search");
                        self.translate.set_tooltip("Translate");
                        self.translate.set_color(Color::Green);
                        self.translate.set_callback({
                            let sender = sender.clone();
                            move |wgt| {
                                wgt.deactivate();
                                sender.send(Self::Event::Run).unwrap();
                            }
                        });
                        &self.translate
                    });
                    wgt.fixed(&self.translate, HEIGHT);
                    wgt.end();
                    wgt
                },
                HEIGHT,
            );
            wgt.add(&{
                let mut wgt = Flex::default();
                wgt.set_type(FlexType::Column);
                wgt.set_color(Color::Background2);
                wgt.set_frame(FrameType::RFlatBox);
                wgt.set_margin(5);
                wgt.set_pad(0);
                wgt.handle(add_resize);
                wgt.add({
                    self.source.config();
                    self.source.set_tooltip("Source");
                    self.source.set_text_size(16);
                    self.source.set_frame(FrameType::FlatBox);
                    self.source.set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            sender
                                .send(Self::Event::Source(wgt.buffer().unwrap().text()))
                                .unwrap();
                        }
                    });
                    &self.source
                });
                wgt.add(&{
                    let mut wgt = Frame::default();
                    wgt.config();
                    wgt
                });
                wgt.add({
                    self.target.config();
                    self.target.set_text_size(16);
                    self.target.set_tooltip("Target");
                    self.target.set_frame(FrameType::FlatBox);
                    &self.target
                });
                wgt.end();
                wgt.handle(Flex::listen);
                wgt.handle_event(Event::Resize);
                wgt
            });
            //~ wgt.close({
            //~ let sender = sender.clone();
            //~ move |_| {
            //~ sender.send(Self::Event::Quit).unwrap();
            //~ }
            //~ });
            wgt.end();
            wgt
        }
    }

    //~ const TO_JSON: Event = Event::from_i32(101);
    //~ fn handle_highlight(display: &mut TextDisplay, event: Event) -> bool {
    //~ let text = display.buffer().unwrap().text();
    //~ match event {
    //~ TO_JSON => {
    //~ use json_tools::{Buffer, BufferType, Lexer, Span, TokenType};
    //~ let mut buffer = vec![b'A'; text.len()];
    //~ for token in Lexer::new(text.bytes(), BufferType::Span) {
    //~ let c = match token.kind {
    //~ TokenType::String => 'B',
    //~ TokenType::BooleanTrue | TokenType::BooleanFalse | TokenType::Null => 'C',
    //~ TokenType::Number => 'D',
    //~ _ => 'A',
    //~ };
    //~ if let Buffer::Span(Span { first, end }) = token.buf {
    //~ let start = first as _;
    //~ let last = end as _;
    //~ buffer[start..last]
    //~ .copy_from_slice(c.to_string().repeat(last - start).as_bytes());
    //~ }
    //~ }
    //~ let mut buf = TextBuffer::default();
    //~ buf.set_text(&String::from_utf8_lossy(&buffer));
    //~ let styles: Vec<StyleTableEntryExt> =
    //~ [Color::Red, Color::Blue, Color::Green, Color::Yellow]
    //~ .into_iter()
    //~ .map(|color| StyleTableEntryExt {
    //~ color,
    //~ font: display.text_font(),
    //~ size: display.text_size(),
    //~ attr: TextAttr::None,
    //~ bgcolor: Color::TransparentBg,
    //~ })
    //~ .collect();
    //~ display.set_highlight_data_ext(buf, styles);
    //~ true
    //~ }
    //~ _ => false,
    //~ }
    //~ }
}

use easytk::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    components::Converter::run(Settings {
        xclass: Some("io.gitlab.kbit.rust"),
        size: Some((400, 640)),
        ..Default::default()
    })?;
    Ok(())
}
