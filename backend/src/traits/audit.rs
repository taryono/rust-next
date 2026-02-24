// src/traits/audit.rs
use crate::context::ServiceContext;
use chrono::Utc;
use sea_orm::Set;

pub trait AuditFields {
    fn set_created_at(self, ctx: &ServiceContext) -> Self;
    fn set_created_by(self, ctx: &ServiceContext) -> Self;
    fn set_updated_at(self, ctx: &ServiceContext) -> Self;
    fn set_foundation(self, ctx: &ServiceContext) -> Self;
}
// ✅ Definisikan trait kecil per field
pub trait HasFoundationId {
    fn set_foundation_id(&mut self, value: i64);
}

pub trait HasCreatedAt {
    fn set_created_at_now(&mut self);
}

pub trait HasUpdatedAt {
    fn set_updated_at_now(&mut self);
}

// ✅ Blanket impl otomatis untuk semua model
// yang mengimplementasi ketiga trait di atas
impl<T> AuditFields for T
where
    T: HasFoundationId + HasCreatedAt + HasUpdatedAt,
{
    fn set_foundation(mut self, ctx: &ServiceContext) -> Self {
        self.set_foundation_id(ctx.foundation_id());
        self
    }

    fn set_created_at(mut self, _ctx: &ServiceContext) -> Self {
        self.set_created_at_now();
        self
    }

    fn set_created_by(mut self, _ctx: &ServiceContext) -> Self {
        self.set_created_at_now();
        self
    }
    fn set_updated_at(mut self, _ctx: &ServiceContext) -> Self {
        self.set_updated_at_now();
        self
    }
}

// ✅ Registrasi per model hanya 3 baris kecil
macro_rules! impl_audit_model {
    ($model:ty) => {
        impl HasFoundationId for $model {
            fn set_foundation_id(&mut self, value: i64) {
                self.foundation_id = Set(value);
            }
        }
        impl HasCreatedAt for $model {
            fn set_created_at_now(&mut self) {
                self.created_at = Set(Utc::now());
            }
        }
        impl HasUpdatedAt for $model {
            fn set_updated_at_now(&mut self) {
                self.updated_at = Set(Utc::now());
            }
        }
    };
}

impl_audit_model!(entity::users::ActiveModel);
impl_audit_model!(entity::menus::ActiveModel);
impl_audit_model!(entity::roles::ActiveModel);
impl_audit_model!(entity::students::ActiveModel);
