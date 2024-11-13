#[cfg(test)]
pub static INIT_DB: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
pub fn init_db_for_test() {
    INIT_DB.call_once(|| {
        dotenvy::from_filename(std::path::Path::new("..").join("..").join(".env")).ok();
        crate::db_init();
    });
}

#[cfg(test)]
mod routes;
