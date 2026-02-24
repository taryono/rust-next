// backend/src/macros/with_context.rs
// Buat macro helper
macro_rules! with_context {
    ($handler:expr) => {
        |state: web::Data<AppState>,
         claims: web::ReqData<Claims>,
         params: web::Query<PaginationParams>| async move {
            let ctx = ServiceContext::from(claims.into_inner());
            $handler(state, ctx, params).await
        }
    };
}

// Penggunaan
// pub fn configure_routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(
//         web::scope("/users")
//             .route("", web::get().to(with_context!(get_users)))
//             .route("", web::post().to(with_context!(create_user))),
//     );
// }
