use reqwest::{Client, StatusCode};

use crate::{
    models::*
};

pub struct Request {
    client: Client,
    url: String
}

impl Request {
    pub fn new(client: Client, url: String) -> Request {
        Request {
            client: client,
            url: url
        }
    }

    pub async fn retrieve_user(
        &mut self,
        id: i32
    ) -> Result<User, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/users/{}", self.url, id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn create_user(
        &mut self,
        user: &NewUser
    ) -> Result<User, ErrorResponse> {
        let resp = self.client
            .post(format!("{}/users", self.url))
            .json(user)
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::CREATED => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn update_user(
        &mut self,
        user: &UpdatedUser
    ) -> Result<User, ErrorResponse> {
        let resp = self.client
            .patch(format!("{}/users", self.url))
            .json(user)
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn destroy_user(
        &mut self,
        id: i32,
    ) -> Result<(), ErrorResponse> {
        let resp = self.client
            .delete(format!("{}/users/{}", self.url, id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn create_group(
        &mut self,
        user_id: i32,
        group: &NewGroup
    ) -> Result<Group, ErrorResponse> {
        let resp = self.client
            .post(format!("{}/users/{}/groups", self.url, user_id))
            .json(group)
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::CREATED => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn destroy_group(
        &mut self,
        user_id: i32,
        group_id: i32
    ) -> Result<(), ErrorResponse> {
        let resp = self.client
            .delete(format!("{}/users/{}/groups/{}", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn join_group(
        &mut self,
        user_id: i32,
        group_id: i32,
    ) -> Result<Member, ErrorResponse> {
        let resp = self.client
            .put(format!("{}/users/{}/groups/{}/join", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::CREATED => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn leave_group(
        &mut self,
        user_id: i32,
        group_id: i32,
    ) -> Result<(), ErrorResponse> {
        let resp = self.client
            .put(format!("{}/users/{}/groups/{}/leave", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn admin_member(
        &mut self,
        user_id: i32,
        group_id: i32,
        member_id: i32
    ) -> Result<Member, ErrorResponse> {
        let resp = self.client
            .put(format!(
                "{}/users/{}/groups/{}/members/{}/admin",
                self.url,
                user_id,
                group_id,
                member_id
            ))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn unadmin_self(
        &mut self,
        user_id: i32,
        group_id: i32,
    ) -> Result<Member, ErrorResponse> {
        let resp = self.client
            .put(format!("{}/users/{}/groups/{}/unadmin", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn allocate(
        &mut self,
        user_id: i32,
        group_id: i32,
    ) -> Result<(), ErrorResponse> {
        let resp = self.client
            .put(format!("{}/users/{}/groups/{}/allocate", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::NO_CONTENT => Ok(()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn recipient(
        &mut self,
        user_id: i32,
        group_id: i32,
    ) -> Result<NamedMember, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/users/{}/groups/{}/recipient", self.url, user_id, group_id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn list_groups(
        &mut self,
    ) -> Result<Vec<Group>, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/groups", self.url))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn retrieve_group(
        &mut self,
        id: i32
    ) -> Result<Group, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/groups/{}", self.url, id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn list_group_admins(
        &mut self,
        id: i32
    ) -> Result<Vec<NamedMember>, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/groups/{}/admins", self.url, id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

    pub async fn list_group_members(
        &mut self,
        id: i32
    ) -> Result<Vec<NamedMember>, ErrorResponse> {
        let resp = self.client
            .get(format!("{}/groups/{}/members", self.url, id))
            .send()
            .await
            .unwrap();
        match resp.status() {
            StatusCode::OK => Ok(resp.json().await.unwrap()),
            _ => Err(resp.json().await.unwrap())
        }
    }

}

