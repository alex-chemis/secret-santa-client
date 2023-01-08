

pub enum Command {
    SignUp,
    LogIn,
    LogOut,
    Stop,
    UpdateSelf,
    DestroyUser,
    CreateGroup,
    DestroyGroup,
    JoinGroup,
    LeaveGroup,
    AdminMember,
    UnadminSelf,
    Allocate,
    Recipient,
    ListGroups,
    RetrieveGroup,
    ListGroupAdmins,
    ListGroupMembers,
    No
}

impl Command {
    pub fn new(str: &str) -> Command {
        match str {
            "sign-up" => Command::SignUp,
            "log-in" => Command::LogIn,
            "log-out" => Command::LogOut,
            "stop" => Command::Stop,
            "update_self" => Command::UpdateSelf,
            "destroy_self" => Command::DestroyUser,
            "create_group" => Command::CreateGroup,
            "destroy_group" => Command::DestroyGroup,
            "join_group" => Command::JoinGroup,
            "leave_group" => Command::LeaveGroup,
            "admin_member" => Command::AdminMember,
            "unadmin_self" => Command::UnadminSelf,
            "allocate" => Command::Allocate,
            "recipient" => Command::Recipient,
            "list_groups" => Command::ListGroups,
            "retrieve_group" => Command::RetrieveGroup,
            "list_group_admins" => Command::ListGroupAdmins,
            "list_group_members" => Command::ListGroupMembers,
            _ => Command::No

        }
    }
}