use super::schema::users;
use super::Database;
use crate::repository::{IUserDB, NewUser, User};
use diesel::pg::Pg;
use diesel::prelude::*;

impl<C> IUserDB for Database<C>
where
    C: Connection<Backend = Pg>,
{
    fn authenticate(&self, username: &str, password: &str) -> Result<User, ()> {
        let results = users::table
            .filter(users::username.eq(username))
            .filter(users::password.eq(password))
            .limit(1)
            .load::<User>(&self.connection)
            .unwrap();

        match results.len() {
            0 => return Err(()),
            1 => {}
            _ => panic!("More than one user with same username: {}", username),
        }

        let user_model = results[0];
        Ok(user_model)
    }

    fn signup(&self, new_user: &NewUser) {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(&self.connection)
            .expect("Error inserting user.");
    }
}
