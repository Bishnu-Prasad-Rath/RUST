pub fn login(cred: models::New) {
    crate::database::get_user(); //Here to access the database module from auth_utils module u have to get out of the auth_utils so u have to use super or crate keyword
}

pub mod models;
