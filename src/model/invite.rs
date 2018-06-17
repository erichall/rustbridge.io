use chrono::NaiveDateTime;
use failure::{Error, ResultExt};
use form::invite::Invite as InviteForm;
use rocket::request::Form;

#[derive(Identifiable, Queryable, Serialize, Debug)]
#[table_name = "invites"]
pub struct InviteModel {
    pub id: i32,
    pub workshop_id: i32,
    pub email: String,
    pub attending: bool,
    pub pending: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

use schema::invites;

#[derive(Insertable, Deserialize, AsChangeset, Debug)]
#[table_name = "invites"]
pub struct Invite<'i> {
    pub workshop_id: Option<i32>,
    pub email: Option<&'i str>,
    pub attending: Option<bool>,
    pub pending: Option<bool>,
}

impl<'i> From<&'i Form<'i, InviteForm>> for Invite<'i> {
    fn from(form: &'i Form<'i, InviteForm>) -> Self {
        Invite {
            workshop_id: Some(form.get().id() as i32),
            email: Some(form.get().email()),
            attending: None,
            pending: None,
        }
    }
}

impl<'i> super::Validate for Invite<'i> {
    fn validate(&self) -> Result<(), Error> {
        if let None = self.email {
            bail!("Email validation failed");
        }

        Ok(())
    }
}

impl<'i> super::Sanitize for Invite<'i> {
    fn sanitize(&self) -> Result<(), Error> {
        if let Some(em) = self.email {
          if em.len() == 0 {
            bail!("Invalid Email Format: {}", em);
          }
        }

        Ok(())
    }
}

use db;

impl<'i> super::Resource for Invite<'i> {
    type Model = InviteModel;

    fn create(&self) -> Result<Option<i32>, Error> {
        use super::{Sanitize, Validate};
        use diesel::RunQueryDsl;
        use schema::invites::dsl::*;

        self.validate()?;
        self.sanitize()?;

        let _ = ::diesel::insert_into(invites)
            .values(self)
            .execute(&db::establish_connection())?;

        let model_id = Self::read_all()
          .unwrap()
          .iter()
          .filter(|i| i.email == self.email.unwrap())
          .collect::<Vec<&Self::Model>>()[0]
          .id;

        Ok(Some(model_id))
    }

    fn read_all() -> Result<Vec<Self::Model>, Error> {
        use diesel::prelude::*;
        use schema::invites::dsl::*;

        let items: Vec<Self::Model> = invites.get_results(&db::establish_connection())?;

        Ok(items)
    }

    fn read_one(model_id: usize) -> Result<Self::Model, Error> {
        use diesel::prelude::*;
        use schema::invites::dsl::*;

        let item: Self::Model = invites
            .filter(id.eq(model_id as i32))
            .get_result(&db::establish_connection())?;

        Ok(item)
    }

    fn update(&self, model_id: usize) -> Result<(), Error> {
        use super::{Sanitize, Validate};
        use diesel::prelude::*;
        use schema::invites::dsl::*;

        self.sanitize()?;

        let existing_invite = Self::read_one(model_id)?;
        ::diesel::update(&existing_invite)
            .set(self)
            .execute(&db::establish_connection())?;

        Ok(())
    }

    fn delete(model_id: usize) -> Result<(), Error> {
        use diesel::prelude::*;

        let existing_invite = Self::read_one(model_id)?;
        ::diesel::delete(&existing_invite).execute(&db::establish_connection())?;

        Ok(())
    }
}
