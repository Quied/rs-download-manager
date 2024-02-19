pub mod thread_update {
    use arc_swap::{ArcSwap, ArcSwapOption};
    use std::{sync::Arc, thread};

    use crate::Update::update::draw_download_window;

    pub enum UpdateStatus {
        ShowUpdateDialog,
        HideUpdateDialog,
        UpdatesChecked,
        UpdatesNotChecked,
    }

    enum Msg2Backend {}

    #[derive(Default, Debug)]
    pub struct Updater {
        pub show_modal_window: bool,
        pub current_version: String,
        pub data: Arc<ArcSwapOption<i32>>,
        // pub update_status: UpdateStatus,
    }

    impl<'a> Updater {
        pub fn default() -> Self {
            Self {
                show_modal_window: true,
                current_version: "0.0.1".to_owned(),
                data: Arc::new(ArcSwapOption::from(None)),
            }
        }

        pub fn thread_update_main(&self) {
            let config = ArcSwap::from(Arc::new(String::default()));

            dbg!("check for updates...");

            let upd_receive = true;
            
            // send to main thr
            

            thread::spawn(move || loop {
                dbg!("thread_update_main");
                thread::sleep(std::time::Duration::from_secs(1));
            });
        }

        pub fn thread_init(&self) {
            println!("backend thread init");
        }
    }
}
