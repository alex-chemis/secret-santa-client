

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
            "update-self" => Command::UpdateSelf,
            "delete-self" => Command::DestroyUser,
            "create" => Command::CreateGroup,
            "delete" => Command::DestroyGroup,
            "join" => Command::JoinGroup,
            "leave" => Command::LeaveGroup,
            "admin" => Command::AdminMember,
            "unadmin-self" => Command::UnadminSelf,
            "allocate" => Command::Allocate,
            "recipient" => Command::Recipient,
            "groups" => Command::ListGroups,
            "retrieve" => Command::RetrieveGroup,
            "admins" => Command::ListGroupAdmins,
            "members" => Command::ListGroupMembers,
            _ => Command::No

        }
    }
}