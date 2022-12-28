
    //mod Downloader;
    //use Downloader::Download::{download_file};


pub mod view {
    use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

    pub fn show_sys_info(sys : sysinfo::System, ctx : &egui::Context, enable : bool){      
        if enable == true {
            egui::CentralPanel::default().show(ctx, |ui| {
            
             // our system info
             let memory_used = ((sys.used_memory() / 1024) / 1024).to_string(); 
             let memory_total = ((sys.total_memory() / 1024) / 1024).to_string(); 
             let cpu_cores = sys.cpus().len().to_string();  
             let user_name = sys.host_name().unwrap();
        

              ui.horizontal(|ui| {
                soome(ctx);

                    ui.horizontal(|ui| {

                    ui.label("Name: ");
                    ui.label(user_name);
                    ui.label("     RAM: ");
                    ui.label(memory_total);
                    ui.label("/");
                    ui.label(memory_used);
                    ui.label("     CPU: ");
                    ui.label(cpu_cores);

                    });  
            });
          });
        }
        
    }

    pub fn soome(ctx : &egui::Context){
        egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("sdf").clicked(){

        }
    });
    }
    

    pub fn show_buttons(ctx : &egui::Context, mut organize : bool){
        
        egui::CentralPanel::default().show(ctx, |ui| {
         ui.horizontal(|ui| {
      


        }); // UI
      });
    }

    

}