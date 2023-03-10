use serde_json;
use std::{io};

use crate::{
    commands::*,
    models::*,
    requests::*
};

pub struct App {
    request: Request,
    user_id: Option<i32>,
    
}

impl App {
    pub fn new(request: Request) -> App {
        App {
            request: request,
            user_id: None,
        }
    }

    pub async fn start(&mut self) {
        println!("Welcome to the secret santa service!");
        
        loop {
            println!("commands:");
            println!("\tsign-up <user_name>");
            println!("\tlog-in <user_id>");
            println!("\tstop");

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();
            buf = buf.trim().to_string();
            let mut strs = buf.split(" ");
            let res = match strs.next() {
                Some(o) => match Command::new(&o) {
                    Command::SignUp => self.sign_up(strs.next()).await,
                    Command::LogIn => self.log_in(strs.next()).await,
                    Command::Stop => {
                        return;
                    }
                    _ => {
                        eprintln!("Command not found");
                        continue;
                    }
                }
                None => {
                    eprintln!("Command not found");
                    continue;
                }
            };

            match res {
                Ok(o) => println!("{}", o),
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }

            println!("commands:");
            println!("\tupdate-self <user_name>");
            println!("\tdelete-self");
            println!("\tcreate <group_name>");
            println!("\tdelete <group_id>");
            println!("\tjoin <group_id>");
            println!("\tleave <group_id>");
            println!("\tadmin <group_id> <member_id>");
            println!("\tunadmin-self <group_id>");
            println!("\tallocate <group_id>");
            println!("\trecipient <group_id>");
            println!("\tgroups");
            println!("\tretrieve <group_id>");
            println!("\tadmins <group_id>");
            println!("\tmembers <group_id>");
            println!("\tlog-out");
            println!("\tstop");
            
            loop {
                let mut buf = String::new();
                io::stdin().read_line(&mut buf).unwrap();
                buf = buf.trim().to_string();
                let mut strs = buf.split(" ");
                let res = match strs.next() {
                    Some(o) => match Command::new(&o) {
                        Command::UpdateSelf => self.update_self(strs.next()).await,
                        Command::CreateGroup => self.create_group(strs.next()).await,
                        Command::DestroyGroup => self.destroy_group(strs.next()).await,
                        Command::JoinGroup => self.join_group(strs.next()).await,
                        Command::LeaveGroup => self.leave_group(strs.next()).await,
                        Command::AdminMember => self.admin_member(strs.next(), strs.next()).await,
                        Command::UnadminSelf => self.unadmin_self(strs.next()).await,
                        Command::Allocate => self.allocate(strs.next()).await,
                        Command::Recipient => self.recipient(strs.next()).await,
                        Command::ListGroups => self.list_groups().await,
                        Command::RetrieveGroup => self.retrieve_group(strs.next()).await,
                        Command::ListGroupAdmins => self.list_group_admins(strs.next()).await,
                        Command::ListGroupMembers => self.list_group_members(strs.next()).await,
                        Command::DestroyUser => match self.destroy_self().await {
                            Ok(_) => break,
                            Err(e) => {
                                Err(e)
                            }
                        }
                        Command::LogOut => {
                            self.user_id = None;
                            break;
                        }
                        Command::Stop => {
                            return;
                        }
                        _ => {
                            eprintln!("Command not found");
                            continue;
                        }
                    }
                    None => {
                        eprintln!("Command not found");
                        continue;
                    }
                };

                match res {
                    Ok(o) => println!("{}", o),
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                }
            }
        }
    }

    async fn sign_up(&mut self, arg: Option<&str>) -> Result<String, String> {
        let name = match arg {
            Some(s) => s.to_string(),
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.create_user(&NewUser { name: name }).await;
        match ret {
            Ok(o) => {
                self.user_id = Some(o.id);
                Ok(format!("Hello, {}. Your id: {}", o.name, o.id).to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn log_in(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.retrieve_user(id).await;
        match ret {
            Ok(o) => {
                self.user_id = Some(o.id);
                Ok(format!("Hello, {}. Your id: {}", o.name, o.id).to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn update_self(&mut self, arg: Option<&str>) -> Result<String, String> {
        let name = match arg {
            Some(s) => s.to_string(),
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.update_user(&UpdatedUser { name: Some(name) }).await;
        match ret {
            Ok(o) => {
                self.user_id = Some(o.id);
                Ok(format!("Name has been updated").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn destroy_self(&mut self) -> Result<String, String> {
        let ret = self.request.destroy_user(self.user_id.unwrap()).await;
        match ret {
            Ok(_) => {
                self.user_id = None;
                Ok("".to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn create_group(&mut self, arg: Option<&str>) -> Result<String, String> {
        let name = match arg {
            Some(s) => s.to_string(),
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.create_group(
            self.user_id.unwrap(), 
            &NewGroup { name: name }
        ).await;
        match ret {
            Ok(o) => {
                Ok(format!("Group {} was created. Group id: {}", o.name, o.id).to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn destroy_group(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.destroy_group(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(_) => {
                Ok(format!("Group id:{id} was was destroyed").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn join_group(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.join_group(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(o) => {
                Ok(format!("You join to group id:{}. Your member id: {}", id, o.id).to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn leave_group(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.leave_group(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(_) => {
                Ok(format!("You leave from group id:{id}").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn admin_member(&mut self, arg1: Option<&str>, arg2: Option<&str>) -> Result<String, String> {
        let group_id = match arg1 {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let member_id = match arg2 {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.admin_member(
            self.user_id.unwrap(),
            group_id,
            member_id
        ).await;
        match ret {
            Ok(_) => {
                Ok(format!("You have given the member id:{member_id} admin rights:").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn unadmin_self(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.unadmin_self(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(_) => {
                Ok(format!("You have removed administrator rights in group id:{id}").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn allocate(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.allocate(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(_) => {
                Ok(format!("Santa's members distributed in group id:{id}").to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn recipient(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.recipient(
            self.user_id.unwrap(),
            id
        ).await;
        match ret {
            Ok(o) => {
                Ok(format!("Your recipient: member_id: \"{}\" name: \"{}\"", o.id, o.name).to_string())
            }
            Err(e) => Err(e.message)
        }
    }

    async fn list_groups(&mut self) -> Result<String, String> {
        let ret = self.request.list_groups().await;
        match ret {
            Ok(o) => Ok(serde_json::to_string_pretty(&o).unwrap()),
            Err(e) => Err(e.message)
        }
    }

    async fn retrieve_group(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.retrieve_group(id).await;
        match ret {
            Ok(o) => Ok(serde_json::to_string_pretty(&o).unwrap()),
            Err(e) => Err(e.message)
        }
    }

    async fn list_group_admins(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.list_group_admins(id).await;
        match ret {
            Ok(o) => Ok(serde_json::to_string_pretty(&o).unwrap()),
            Err(e) => Err(e.message)
        }
    }

    async fn list_group_members(&mut self, arg: Option<&str>) -> Result<String, String> {
        let id = match arg {
            Some(s) => match s.trim().parse::<i32>() {
                Ok(o) => o,
                Err(e) => return Err(e.to_string())
            }
            None => return Err("Not enough arguments".to_string())
        };
        let ret = self.request.list_group_members(id).await;
        match ret {
            Ok(o) => Ok(serde_json::to_string_pretty(&o).unwrap()),
            Err(e) => Err(e.message)
        }
    }




}
